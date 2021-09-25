use actix_web::{HttpResponse, web};
use actix_session::Session;
use lily_service::WebResponseError;
use {serde_json, serde_json::{Value as JsonValue}};


fn get_user_session(u_id: i32, session: &Session) -> Result<Option<String>, WebResponseError> {
  Ok(session.get::<String>(&u_id.to_string())?)
}

fn try_logout(u_id: i32, session: &Session) -> Result<JsonValue, WebResponseError> {
  let session_id = get_user_session(u_id, session)?;
  if let Some(_) = session_id {
    session.remove(&u_id.to_string());

    let data = r#"{"status_code":"200", "message": "Logged out successfully."}"#;
    let json_value : JsonValue = serde_json::from_str(data).unwrap();
    return Ok(json_value);
  }

  let data = r#"{"status_code":"None", "message": "Logout failed."}"#;
  let json_value : JsonValue = serde_json::from_str(data).unwrap();
  Ok(json_value)
}

pub fn logout_user(info: web::Path<i32>, session: Session) -> HttpResponse {
  match try_logout(info.0, &session) {
    Ok(d) => HttpResponse::Ok().json(d),
    Err(_) => HttpResponse::Ok().body("Failed.")
  }
}