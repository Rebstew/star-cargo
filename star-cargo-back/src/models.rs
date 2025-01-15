use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct StarCargoEntry {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,  // MongoDB ID type
    pub name: String,
    pub description: String,
    pub comments: Vec<Comment>,
    pub floors: Vec<Floor>,
    pub upvotes: Vec<ObjectId>,  // MongoDB ID type
    pub downvotes: Vec<ObjectId>,  // MongoDB ID type
    pub image_bytes: Vec<u8>,
    pub creation_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub author: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Floor {
    pub cells: Vec<Cell>,
    pub height: i32,
    pub length: i32,
    pub width: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub height: i32,
}
