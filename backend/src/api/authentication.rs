use std::sync::Arc;

use axum::extract::State;
use axum::Form;
use pbkdf2::password_hash::rand_core::OsRng;
use pbkdf2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use pbkdf2::Pbkdf2;
use serde::{Deserialize, Serialize};

use super::{ApiError, SharedState};

#[derive(Serialize, Deserialize, Debug)]
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

pub async fn sign_up(
    State(state): State<Arc<SharedState>>,
    Form(body): Form<RequestUserBody>,
) -> Result<(), ApiError> {
    let hashed_password = hash_password(&body.password)?;

    state
        .database
        .create_user(&body.username, &body.email, &hashed_password)
        .await?;

    Ok(())
}
