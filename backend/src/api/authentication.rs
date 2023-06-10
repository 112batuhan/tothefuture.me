use std::sync::Arc;

use axum::extract::State;
use axum::Form;
use pbkdf2::password_hash::rand_core::OsRng;
use pbkdf2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use pbkdf2::Pbkdf2;
use serde::{Deserialize, Serialize};
use tracing_subscriber::fmt::format::debug_fn;

use super::{ApiError, SharedState};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestUserBody {
    email: String,
    password: String,
}

fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash_result = Pbkdf2.hash_password(password.as_bytes(), &salt);

    // Manual result handling because missing std::error implementation in hash library
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
        .create_user(&body.email, &hashed_password)
        .await?;

    Ok(())
}

pub async fn sign_in(
    State(state): State<Arc<SharedState>>,
    Form(body): Form<RequestUserBody>,
) -> Result<(), ApiError> {
    dbg!(&body);
    let user = state.database.find_user_by_email(&body.email).await?;
    dbg!(&user);
    let parsed_hash = PasswordHash::new(&user.password).unwrap();
    dbg!(&parsed_hash);
    let password_result = Pbkdf2.verify_password(body.password.as_bytes(), &parsed_hash);
    dbg!(password_result);
    match password_result {
        Ok(_) => Ok(()),
        Err(err) => match err {
            pbkdf2::password_hash::Error::Password => Err(ApiError::WrongPassword),
            _ => Err(ApiError::Hash),
        },
    }
}
