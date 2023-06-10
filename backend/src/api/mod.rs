pub mod authentication;

use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use rand_chacha::ChaCha8Rng;
use rand_core::{OsRng, RngCore, SeedableRng};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::Mutex;

use super::database::Db;
use crate::database::DbError;

pub struct SharedState {
    database: Db,
    random: Arc<Mutex<ChaCha8Rng>>,
}

impl SharedState {
    pub async fn new() -> Result<Arc<Self>, ApiError> {
        let database = Db::new().await?;

        let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());
        let random = Arc::new(Mutex::new(random));

        Ok(Arc::new(SharedState { database, random }))
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
    // Can't use #[from] here because missing std::error implementation in hash library
    #[error("Error during hashing.")]
    Hash,
    #[error("Database error: {0}")]
    Database(#[from] DbError),
    #[error("Wrong password.")]
    WrongPassword,
    #[error("Missing token from the client request.")]
    MissingSessionTokenInClientRequest,
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
            ApiError::WrongPassword => StatusCode::UNAUTHORIZED,
            ApiError::Database(database_error) => match database_error {
                DbError::UniqueConstraintViolation => StatusCode::CONFLICT,
                DbError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
                DbError::EmptyQuery => StatusCode::NOT_FOUND,
                DbError::MissingSessionToken => StatusCode::UNAUTHORIZED,
            },
            ApiError::MissingSessionTokenInClientRequest => StatusCode::UNAUTHORIZED,
        }
    }

    fn to_error_json(&self) -> Json<ErrorMessage> {
        let error_type = match self {
            ApiError::General => "Unhandled_internal_error".to_string(),
            ApiError::Hash => "hash_error".to_string(),
            ApiError::WrongPassword => "password_mismatch".to_string(),
            ApiError::Database(database_error) => match database_error {
                DbError::UniqueConstraintViolation => "existing_user".to_string(),
                DbError::Database(_) => "unhandled_database_error".to_string(),
                DbError::EmptyQuery => "empty_query_result".to_string(),
                DbError::MissingSessionToken => "missing_token_in_server".to_string(),
            },
            ApiError::MissingSessionTokenInClientRequest => {
                "missing_token_in_client_request".to_string()
            }
        };

        let message = self.to_string();

        Json(ErrorMessage {
            error_type,
            message,
        })
    }
}
