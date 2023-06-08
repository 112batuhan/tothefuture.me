use std::sync::Arc;

use axum::extract::State;
use axum::response::Redirect;
use axum::Json;
use pbkdf2::password_hash::rand_core::OsRng;
use pbkdf2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use pbkdf2::Pbkdf2;
use serde::{Deserialize, Serialize};

use super::{ApiError, SharedState};

#[derive(Serialize, Deserialize)]
pub struct RequestUserBody {
    username: String,
    email: String,
    password: String,
}

fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);

    let hash_result = Pbkdf2.hash_password(password.as_bytes(), &salt);

    // Manual result handling because of poor error implementation in library
    match hash_result {
        Ok(hashed_password) => Ok(hashed_password.to_string()),
        Err(_) => Err(ApiError::Hash),
    }
}

pub async fn password_authenticate(
    State(state): State<Arc<SharedState>>,
    Json(body): Json<RequestUserBody>,
) -> String {
    let hashed_password = hash_password(&body.password).unwrap();

    state
        .database
        .create_user(&body.username, &body.email, &hashed_password)
        .await
        .unwrap();

    hashed_password
}
