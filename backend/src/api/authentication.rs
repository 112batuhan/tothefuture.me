use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::{http, Form};
use pbkdf2::password_hash::rand_core::OsRng;
use pbkdf2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use pbkdf2::Pbkdf2;
use rand_chacha::ChaCha8Rng;
use rand_core::RngCore;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

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

async fn generate_session_token(random: Arc<Mutex<ChaCha8Rng>>) -> String {
    let mut u128_pool = [0u8; 16];
    {
        // to drop the lock early
        let mut random = random.lock().await;
        random.fill_bytes(&mut u128_pool);
    }
    u128::from_le_bytes(u128_pool).to_string()
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
) -> Result<impl IntoResponse, ApiError> {
    let user = state.database.find_user_by_email(&body.email).await?;
    let parsed_hash = PasswordHash::new(&user.password).unwrap();
    let password_result = Pbkdf2.verify_password(body.password.as_bytes(), &parsed_hash);
    if let Err(err) = password_result {
        return match err {
            pbkdf2::password_hash::Error::Password => Err(ApiError::WrongPassword),
            _ => Err(ApiError::Hash),
        };
    };

    let session_token = generate_session_token(state.random.clone()).await;
    state
        .database
        .create_session(user.id, &session_token)
        .await?;

    http::Response::builder()
        .status(http::StatusCode::SEE_OTHER)
        .header("Location", "/")
        .header(
            "Set-Cookie",
            format!("session_token={}; Max-Age=999999", session_token),
        )
        .body(http_body::Empty::<String>::new())
        .unwrap();

    Ok(())
}
