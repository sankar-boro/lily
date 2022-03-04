use actix_web::{HttpResponse,web};
use crate::App;
use uuid::Uuid;
use serde::Serialize;

use scylla::macros::FromRow;
use crate::utils::{
	GetQueryResult
};

#[derive(FromRow, Serialize)]
pub struct NewDocument {
    documentId: Uuid,
    title: String,
    tags: String,
    body: String
}

#[derive(FromRow, Serialize)]
pub struct Book {
    bookId: Uuid,
    uniqueId: Uuid,
    parentId: Option<Uuid>,
    authorId: Option<Uuid>,
    authorName: Option<String>,
    title: String,
    body: String,
    identity: i16,
    createdAt: Uuid,
    updatedAt: Uuid,
}

static GET_ALL_DOCUMENTS: &'static str = "SELECT bookId, uniqueId, parentId, authorId, authorName, title, body, identity, createdAt, updatedAt from sankar.book WHERE parentId=null";
static GET_ALL_DOCUMENTS_FROM_ID: &'static str = "SELECT bookId, uniqueId, parentId, authorId, authorName, title, body, identity, createdAt, updatedAt from sankar.book WHERE bookId=";

pub async fn getAllBooks(app: web::Data<App>) 
-> Result<HttpResponse, actix_web::Error> {
    let documents: Option<Vec<Book>> = 
    app.query(GET_ALL_DOCUMENTS, &[])
    .await
    .get_query_result()?;

    match documents {
        Some(docs) => Ok(HttpResponse::Ok().json(docs)),
        None => {
            let mt: Vec<Book> = Vec::new();
            Ok(HttpResponse::Ok().json(mt))
        },
    }
}

pub async fn getAllNodesFromBookId(app: web::Data<App>, book_id: web::Path<String>) -> Result<HttpResponse, crate::AppError> {
    let bookId = Uuid::parse_str(&book_id)?;
    let query = format!("{}{}", GET_ALL_DOCUMENTS_FROM_ID, &bookId);
    let documents: Option<Vec<Book>> = 
		app.query(query, &[])
		.await
		.get_query_result()?;

    // TODO: should recover from unwrap()
    match documents {
        Some(docs) => Ok(HttpResponse::Ok().json(docs)),
        None => {
            let mt: Vec<Book> = Vec::new();
            Ok(HttpResponse::Ok().json(mt))
        },
    }
}