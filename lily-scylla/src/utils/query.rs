use crate::ScyllaConnectionManager;
use crate::AppError;

use scylla::QueryResult;
use scylla::IntoTypedRows;
use r2d2::PooledConnection;
use scylla::transport::errors::QueryError;
use scylla::frame::response::cql_to_rust::FromRow;

pub trait ConnectionResult {
	fn conn_result(&self) -> Result<PooledConnection<ScyllaConnectionManager>, actix_web::Error>;
}

pub trait GetQueryResult<T> {
	type Request;
	fn get_query_result(self) -> Result<Option<Vec<Self::Request>>, actix_web::Error>;
}

impl<T: FromRow> GetQueryResult<T> for Result<QueryResult, QueryError> {
    type Request = T;
	fn get_query_result(self) -> Result<Option<Vec<Self::Request>>, actix_web::Error> {
		self
		.map_err(|err| AppError::from(err).into())
		.map(|res| {
			res.rows.map(|rows| {
				rows.into_typed::<Self::Request>()
					.map(|a| a.unwrap())
					.collect::<Vec<Self::Request>>()
			})
		})
    }
}

pub struct Update {
	query: String,
}
impl Update {
	pub fn from(table: &str) -> Self {
		let mut q = String::from("UPDATE");
		q.push_str(" ");
		q.push_str(table);
		q.push_str(" ");
		q.push_str("SET");
		q.push_str(" ");

		Self {
			query: q,
		}
	}

	pub fn set(mut self, key: &str, value: &str) -> Self {
		self.query.push_str(key);
		self.query.push_str("=");
		self.query.push_str("'");
		self.query.push_str(value);
		self.query.push_str("'");
		self.query.push_str(" ");
		self
	}

	pub fn where_in(mut self, key: &str, value: &str) -> Self {
		self.query.push_str("WHERE");
		self.query.push_str(" ");
		self.query.push_str(key);
		self.query.push_str("=");
		self.query.push_str(value);
		self
	}

	pub fn query(self) -> String {
		self.query.clone()
	} 


}