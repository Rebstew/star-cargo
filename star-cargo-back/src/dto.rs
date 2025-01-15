use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostImageDto {
    pub image_bytes: Vec<u8>,
    pub name: String,
}