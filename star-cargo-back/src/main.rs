use actix_web::{web, App, HttpServer};
use mongodb::options::GridFsBucketOptions;
use mongodb::{Collection, GridFsBucket};
use crate::database::connect_to_db;
use crate::handlers::{create_entry, get_entries};
use crate::models::StarCargoEntry;

mod database;
mod handlers;
mod models;
mod dto;

struct AppState {
    bucket: GridFsBucket,                     // For GridFS operations
    collection: Collection<StarCargoEntry>,       // A MongoDB collection
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Connect to MongoDB and get the collection.
    let db = connect_to_db().await.expect("Failed to connect to database");

    // The gridfs bucket called images
    let bucket_options = GridFsBucketOptions::builder()
        .bucket_name("images".to_string())
        .build();

    let bucket = db.gridfs_bucket(bucket_options);
    let collection: Collection<StarCargoEntry> = db.collection("entries");

    let app_state = web::Data::new(AppState {
        bucket,
        collection,
    });

    // Start the HTTP server.
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone()) // Pass MongoDB collection to app state
            .route("/entries", web::post().to(create_entry))
            .route("/entries", web::get().to(get_entries))
            .route("/popularEntries", web::get().to(handlers::get_popular_entries))
            .route("/image/{image_id}", web::get().to(handlers::get_image))
            .route("/image", web::post().to(handlers::post_image))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
