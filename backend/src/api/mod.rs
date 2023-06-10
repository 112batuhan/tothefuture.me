pub mod authentication;

use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use super::database_sea::Db;
use crate::database_sea;

pub struct SharedState {
    database: Db,
}

impl SharedState {
    pub async fn new() -> Result<Arc<Self>, ApiError> {
        let database = Db::new().await?;
        Ok(Arc::new(SharedState { database }))
    }
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("General internal error.")]
    General,
    // Can't use #[from] here because of poor error implementation in hash library
    #[error("Error during hashing")]
    Hash,
    #[error("Database error {0}")]
    Database(#[from] database::DbError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = self.to_string();
        let status_code = self.to_status_code();
        (status_code, body).into_response()
    }
}

impl ApiError {
    fn to_status_code(self) -> StatusCode {
        match self {
            ApiError::General => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Hash => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
