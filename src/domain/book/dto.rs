use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBookDto {
    pub title: String,
    pub author: String,
    pub year: i16,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBookDto {
    pub title: Option<String>,
    pub author: Option<String>,
    pub year: Option<i16>,
}

#[derive(Debug, Serialize)]
pub struct DeleteBookResult {
    pub id: i32,
}
