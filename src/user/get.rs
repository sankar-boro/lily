use actix_session::Session;
use actix_web::{HttpResponse,web};
use crate::App;
use uuid::Uuid;
use serde::Serialize;
use crate::AppError;

use scylla::macros::FromRow;
use crate::utils::{GetQueryResult};

#[derive(FromRow, Serialize)]
struct GetUser {
    id: Uuid,
    fname: Option<String>,
    lname: Option<String>,
    email: String,
}

fn get_user_query(user_id: &str) 
-> Result<String, actix_web::Error> {
    match Uuid::parse_str(user_id) {
        Ok(user_id) => {
            Ok(format!("SELECT userId, fname, lname, email from sankar.users where userId={} LIMIT 1", user_id))
        }
        Err(err) => Err(AppError::from(err).into())
    }
}

pub async fn get(app: web::Data<App>, userId: web::Path<String>) 
-> Result<HttpResponse, crate::AppError> {
    let rows: Option<Vec<GetUser>> = 
		app.query(get_user_query(&userId)?, &[])
		.await
		.get_query_result()?;
    match rows {
        Some(rows) => {
            Ok(HttpResponse::Ok().json(rows))
        }
        None => {
            let mt: Vec<GetUser> = Vec::new();
            Ok(HttpResponse::Ok().json(mt))
        }
    }
}

pub async fn user_session(session: Session) 
-> Result<HttpResponse, actix_web::Error> {
    let auth_user_session = session.get::<String>("AUTH_USER")?;
    match auth_user_session {
        Some(session) => {
            Ok(HttpResponse::Ok().body(session))
        }
        None => Err(AppError::from("REQUEST_LOGIN").into())   
    }
}