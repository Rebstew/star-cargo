use actix_web::{web, HttpResponse};
use mongodb::{options::FindOptions, Collection};
use bson::doc;
use crate::models::StarCargoEntry;
use futures_util::stream::StreamExt;

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

pub async fn get_popular_entries(db: web::Data<Collection<StarCargoEntry>>) -> HttpResponse {
    // Add logs to the console
    println!("get_popular_entries -- Getting popular entries");

    let find_options = FindOptions::builder()
        .sort(doc! { "likes": -1 })
        .limit(3)
        .build();

    println!("get_popular_entries -- Built the options, before find in DB...");

    let mut cursor = db.find(None, find_options).await.unwrap();
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