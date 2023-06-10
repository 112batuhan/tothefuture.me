pub mod authentication;

use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::database::Db;
use crate::database::DbError;

pub struct SharedState {
    database: Db,
}

impl SharedState {
    pub async fn new() -> Result<Arc<Self>, ApiError> {
        let database = Db::new().await?;
        Ok(Arc::new(SharedState { database }))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    error_type: String,
    message: String,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("General internal error.")]
    General,
    // Can't use #[from] here because of poor error implementation in hash library
    #[error("Error during hashing.")]
    Hash,
    #[error("Database error.")]
    Database(#[from] DbError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = self.to_error_json();
        let status_code = self.to_status_code();
        (status_code, body).into_response()
    }
}

impl ApiError {
    fn to_status_code(&self) -> StatusCode {
        match self {
            ApiError::General => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Hash => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Database(database_error) => match database_error {
                DbError::UniqueConstraintViolation => StatusCode::CONFLICT,
                DbError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }

    fn to_error_json(&self) -> Json<ErrorMessage> {
        let error_type = self.to_string();

        let message = match self {
            ApiError::Database(err) => err.to_string(),
            _ => "".to_string(),
        };

        Json(ErrorMessage {
            error_type,
            message,
        })
    }
}
