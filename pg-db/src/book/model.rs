use validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CategoryData {
    category: String,
    exists: bool,
}

#[derive(Deserialize, Validate)]
pub struct ParentRequest {
    #[validate(length(min = 2))]
    pub title: String,
    pub body: Option<String>,
    pub metadata: String,
    pub image_url: Option<String>,
}

#[derive(Deserialize)]
pub struct DeleteBookRequest {
    pub uid: String,
}

#[derive(Deserialize, Validate, Serialize, Clone)]
pub struct UpdateRequest {
    pub title: String,
    pub body: String,
    pub uid: String,
    pub metadata: String
}

#[derive(Deserialize, Validate, Serialize, Clone)]
pub struct GetBook {
    pub uid: i32,
    pub authorid: i32,
    pub bookid: i32,
    pub parentid: Option<i32>,
    pub title: String,
    pub body: String,
    pub identity: i16,
    pub metadata: String
}