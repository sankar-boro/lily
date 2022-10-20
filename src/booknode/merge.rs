use actix_web::{HttpResponse, web};
use scylla::{
    batch::Batch, 
    frame::value::BatchValues,
    BatchResult
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::App;
use validator::Validate;
use lily_utils::time_uuid;
use scylla::macros::FromRow;
use actix_session::Session;
use crate::{auth::AuthSession};

#[derive(Deserialize, Validate, FromRow)]
pub struct MergeNodeRequest {
    title: String,
    body: String,
    identity: i16,
    bookId: String,
    metadata: String,
    topUniqueId: String,
    botUniqueId: String,
}

#[derive(Serialize)]
pub struct Response {
    uniqueId: String,
}

pub static UPDATE_PARENT_ID: &str = "UPDATE sankar.book SET parentId=? WHERE bookId=? AND uniqueId=?";
pub static CHILD: &str = "INSERT INTO sankar.book (
    bookId, uniqueId, parentId, authorId, title, body, identity, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
)";

impl MergeNodeRequest {

    async fn batch(&self, app: &App, batch_values: impl BatchValues) -> Result<BatchResult, crate::AppError> {
        let mut batch: Batch = Default::default();
        batch.append_statement(UPDATE_PARENT_ID);
        batch.append_statement(CHILD);
        Ok(app.batch(&batch, batch_values).await?)
    }

    async fn run(&self, app: &App, session: &Session) -> Result<HttpResponse, crate::AppError> {
        let auth = session.user_info()?;
        let author_id = Uuid::parse_str(&auth.userId)?;

        // Create and parse elements
        let new_id = time_uuid();
        let book_id = Uuid::parse_str(&self.bookId)?;
        let top_unique_id = Uuid::parse_str(&self.topUniqueId)?;
        let bot_unique_id = Uuid::parse_str(&self.botUniqueId)?;
        let unique_id = new_id.to_string();

        // Create data
        let create_data = ( 
            &book_id,
            &new_id,
            &top_unique_id,
            &author_id,
            &self.title,
            &self.body,
            &self.identity,
            &self.metadata,
            &new_id,
            &new_id
        );

        let update_data = (
            &new_id,
            &book_id,
            bot_unique_id
        );
        let batch_values = (
            update_data,
            create_data
        );
        self.batch(app, batch_values).await?;

        Ok(HttpResponse::Ok().json(Response {
            uniqueId: unique_id
        }))
    }
}

pub async fn merge(
    app: web::Data<App>, 
    payload: web::Json<MergeNodeRequest>,
    session: Session

) 
-> Result<HttpResponse, crate::AppError> 
{   
    payload.run(&app, &session).await
}
