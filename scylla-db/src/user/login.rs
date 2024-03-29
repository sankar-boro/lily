use crate::Connections;
use crate::error::Error;
use scylla::macros::FromRow;

use time::Duration;
use uuid::Uuid;
use validator::Validate;
use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use crate::utils::GetQueryResult;
use serde_json::json;
use lily_utils::validate_user_credentials;
use actix_web::cookie;

#[derive(Deserialize, Debug, Validate)]
pub struct LoginForm {
    #[validate(email)]
	email: String,
	password: String,
}

#[derive(FromRow, Serialize, Debug)]
pub struct GetUser {
	userId: i32,
	email: String,
	password: String,
	fname: String,
	lname: String,
}

fn get_user_query(email: &str) 
-> String {
	let mut query = String::new();
	query.push_str("SELECT userId, email, password, fname, lname from sankar.userCredentials where email='");
	query.push_str(email);
	query.push_str("'LIMIT 1");
	query
}

static GET_USER: &str = "SELECT userId, fname, lname, password FROM users WHERE email=$1";
pub async fn login(
	request: web::Json<LoginForm>, 
	app: web::Data<Connections>, 
	session: Session
) 
-> Result<HttpResponse, Error> 
{
	request.validate()?;
	let client = app.pool.get().await?;
    let stmt = client.prepare_cached(GET_USER).await?;
    let rows = client.query(&stmt, &[&request.email]).await?;
	if rows.len() == 0 {
		let unf = json!({
			"status": 500,
			"message": "user not found.".to_string(),
		});
		return Ok(HttpResponse::NotFound().json(unf));
	}
	let user_id: i32 = rows[0].get(0);
	let fname: String = rows[0].get(1);
	let lname: String = rows[0].get(2);
	let password: String = rows[0].get(3);

	validate_user_credentials(&request.password, &password)?;
	
	let auth_user_session = json!({
		"userId": user_id,
		"email": &request.email.clone(),
		"fname": fname.clone(),
		"lname": lname.clone(),
	});
	let x = auth_user_session.clone().to_string();
	
	session.insert("AUTH_USER", x.clone())?;
	session.insert("AUTH_ID", user_id)?;
	// Ok(HttpResponse::Ok().json(auth_user_session))
	Ok(HttpResponse::Ok()
        .append_header(("Authorization", x)).json(auth_user_session))
}

#[derive(Deserialize)]
pub struct GetUserX {
	email: String,
}

static GET_USERX: &str = "SELECT userId, fname, lname, password FROM users WHERE email=$1";
pub async fn get_user(
	request: web::Json<GetUserX>, 
	app: web::Data<Connections>
) 
-> Result<HttpResponse, Error> 
{
	let client = app.pool.get().await?;
    let stmt = client.prepare_cached(GET_USERX).await?;
    let rows = client.query(&stmt, &[&request.email]).await?;
	let user_id: i32 = rows[0].get(0);
	let fname: String = rows[0].get(1);
	let lname: String = rows[0].get(2);
	
	let auth_user_session = json!({
		"userId": user_id.clone(),
		"email": &request.email.clone(),
		"fname": fname.clone(),
		"lname": lname.clone(),
	});
	Ok(HttpResponse::Ok().json(auth_user_session))
}


#[derive(FromRow, Serialize, Debug)]
pub struct GetUserScylla {
	userId: Uuid,
	email: String,
	password: Vec<u8>,
	fname: String,
	lname: String,
}

pub async fn get_user_scylla(
	request: web::Json<GetUserX>, 
	app: web::Data<Connections>
) 
-> Result<HttpResponse, Error> 
{
	let rows: Option<Vec<GetUserScylla>> = 
		app.query(get_user_query(&request.email), &[])
		.await
		.get_query_result()?;
	let auth_user: &GetUserScylla = match &rows {
		Some(users) => {
			match users.first() {
				Some(user) => user,
				None => return Err(Error::from("USER_NOT_FOUND").into())
			}
		},
		None => return Err(Error::from("USER_NOT_FOUND").into())
	};
	
	let auth_user_session = json!({
		"userId": auth_user.userId.to_string(),
		"email": auth_user.email.clone(),
		"fname": auth_user.fname.clone(),
		"lname": auth_user.lname.clone(),
	});
	Ok(HttpResponse::Ok().json(auth_user_session))
}