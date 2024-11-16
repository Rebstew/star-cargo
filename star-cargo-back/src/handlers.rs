use actix_web::{web, HttpResponse};
use mongodb::bson::doc;
use mongodb::Collection;
use crate::models::StarCargoEntry;

pub async fn create_entry(
    entry: web::Json<StarCargoEntry>,
    db: web::Data<Collection<StarCargoEntry>>
) -> HttpResponse {
    let new_entry = entry.into_inner();

    match db.insert_one(new_entry, None).await {
        Ok(_) => HttpResponse::Created().json("Entry added successfully"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn get_entries(db: web::Data<Collection<StarCargoEntry>>) -> HttpResponse {
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