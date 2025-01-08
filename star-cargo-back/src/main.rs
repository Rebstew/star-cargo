use actix_web::{web, App, HttpServer};
use mongodb::Collection;
use crate::database::connect_to_db;
use crate::handlers::{create_entry, get_entries};
use crate::models::StarCargoEntry;

mod database;
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Connect to MongoDB and get the collection.
    let db = connect_to_db().await.expect("Failed to connect to database");
    let collection: Collection<StarCargoEntry> = db.collection("entries");

    // Start the HTTP server.
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(collection.clone())) // Pass MongoDB collection to app state
            .route("/entries", web::post().to(create_entry))
            .route("/entries", web::get().to(get_entries))
            .route("/popularEntries", web::get().to(handlers::get_popular_entries))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
