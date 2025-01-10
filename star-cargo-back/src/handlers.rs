use actix_web::{web, HttpResponse, Responder};
use bson::{doc, oid::ObjectId, Bson};
use mongodb::options::FindOptions;
use crate::{models::StarCargoEntry, AppState};
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

    while let Some(result) = cursor.next().await {
        match result {
            Ok(entry) => entries.push(entry),
            Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
        }
    }

    println!("get_popular_entries -- Returning popular entries");

    HttpResponse::Ok().json(entries)
}

pub async fn get_image(app_state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let file_id = id.into_inner();

    // Parse the file_id into an ObjectId
    let object_id = Bson::ObjectId(ObjectId::parse_str(&file_id).unwrap());

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

    // Return the file as an HTTP response
    HttpResponse::Ok()
    .content_type("image/png") // Adjust MIME type as needed
    .body(buffer)
}