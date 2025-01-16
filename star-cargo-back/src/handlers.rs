use actix_web::{web, HttpResponse, Responder};
use bson::{doc, oid::ObjectId, Bson};
use futures_util::AsyncWriteExt;
use mongodb::options::FindOptions;
use crate::{models::StarCargoEntry, AppState};
use crate::dto::PostImageDto;
use futures_util::{stream::StreamExt, AsyncReadExt};

pub async fn create_entry(
    entry: web::Json<StarCargoEntry>,
    app_state: web::Data<AppState>
) -> HttpResponse {
    let new_entry = entry.into_inner();

    match app_state.collection.insert_one(new_entry, None).await {
        Ok(_) => HttpResponse::Created().json("Entry added successfully"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn get_entries(state: web::Data<AppState>) -> HttpResponse {

    let db = &state.collection;
    let mut cursor = db.find(None, None).await.unwrap();
    let mut entries: Vec<StarCargoEntry> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(entry) => entries.push(entry),
            Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
        }
    }

    HttpResponse::Ok().json(entries)
}

pub async fn get_popular_entries(app_state: web::Data<AppState>) -> HttpResponse {
    // Add logs to the console
    println!("get_popular_entries -- Getting popular entries");

    let find_options = FindOptions::builder()
        .sort(doc! { "likes": -1 })
        .limit(3)
        .build();

    println!("get_popular_entries -- Built the options, before find in DB...");

    let mut cursor = app_state.collection.find(None, find_options).await.unwrap();
    let mut entries: Vec<StarCargoEntry> = Vec::new();

    println!("get_popular_entries -- Found entries, iterating...");

    while let Some(result) = cursor.next().await {
        match result {
            Ok(entry) => {
                println!("get_popular_entries -- Found entry");
                entries.push(entry)
            },
            Err(e) => {
                eprintln!("get_popular_entries -- Found error {e}");
            },
        }
    }

    println!("get_popular_entries -- Returning popular entries");

    HttpResponse::Ok().json(entries)
}

pub async fn get_default_ship_image() -> HttpResponse {
    // Load the default image from the file system
    let default_image = include_bytes!("../res/img/ship.jpeg");

    // Return the default image as an HTTP response, cached for 1 day
    HttpResponse::Ok()
    .content_type("image/jpeg")
    .append_header(("Cache-Control", "public, max-age=86400"))
    .body(&default_image[..])
}

/// Retrieves an image from the server. The image id is passed as a path parameter.
/// Returns the default image if the image is not found.
pub async fn get_image(app_state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let file_id = id.into_inner();

    // Log the request
    println!("{}", format!("get_image -- Received image request, image id: {file_id}"));

    // Parse the file_id into an ObjectId
    let object_id = match ObjectId::parse_str(&file_id) {
        Ok(id) => Bson::ObjectId(id),
        Err(_) => return HttpResponse::BadRequest().body("Invalid image ID format"),
    };

    // Check if the file exists
    let file_exists = match app_state.bucket.find(doc! { "_id": &object_id }, None).await {
        Ok(cursor) => cursor.count().await > 0,
        Err(_) => false,
    };

    println!("{}", format!("get_image -- File exists: {file_exists}"));

    // If the file does not exist, return the default file, ship.jpeg
    if !file_exists {
        return get_default_ship_image().await;
    }

    // Open a download stream for the file
    let mut download_stream: mongodb::GridFsDownloadStream = match app_state.bucket.open_download_stream(object_id).await {
        Ok(stream) => stream,
        Err(_) => return HttpResponse::NotFound().body("File not found"),
    };

    // Read the file into a buffer
    let mut buffer = Vec::new();
    if let Err(_) = download_stream.read_to_end(&mut buffer).await {
        return HttpResponse::InternalServerError().body("Error reading file");
    }

    let buflen = buffer.len();
    println!("{}", format!("get_image -- Returning image, length: {buflen}"));

    // Return the file as an HTTP response
    HttpResponse::Ok()
    .content_type("image/png") // Adjust MIME type as needed
    .body(buffer)
}

/// Uploads an image to the server. The request body contains the image id (taken from the ship id) and the image bytes.
pub async fn post_image(app_state: web::Data<AppState>, data: web::Json<PostImageDto>) -> impl Responder {
    // Log the request
    let data_name = &data.name;
    let data_length = &data.image_bytes.len().to_string();

    println!("{}", format!("post_image -- Received image upload request, image id: {data_name}, image length: {data_length}"));

    // Reject if the supplied name is empty
    if data.name.is_empty() {
        return HttpResponse::BadRequest().body("Name cannot be empty");
    }

    // Reject if the data bytes is empty
    if data.image_bytes.is_empty() {
        return HttpResponse::BadRequest().body("Image bytes cannot be empty");
    }

    // Create a new file in the bucket
    let upload_options = mongodb::options::GridFsUploadOptions::builder()
        .metadata(doc! { "name": &data.name })
        .build();
    let mut upload_stream = app_state.bucket.open_upload_stream(&data.name, upload_options);

    // Read the payload into a buffer
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&data.image_bytes);

    // Write the buffer to the file
    if let Err(_) = upload_stream.write_all(&buffer).await {
        return HttpResponse::InternalServerError().body("Error writing file");
    }

    HttpResponse::Created().json(&data.name)
}