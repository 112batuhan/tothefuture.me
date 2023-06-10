use std::sync::Arc;

use axum::extract::State;
use axum::http::header::COOKIE;
use axum::http::{HeaderMap, HeaderValue, Request};
use axum::response::Response;
use axum::Form;
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

// TODO: Optimize this to get rid of clones
// At this point I'm coding for 13 hours straight.
// I just want to get this over with.
fn extract_token(header: HeaderValue) -> String {
    header
        .to_str()
        .unwrap()
        .split("=")
        .skip(1)
        .next()
        .unwrap()
        .to_string()
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
) -> Result<HeaderMap, ApiError> {
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
        .upsert_session(user.id, &session_token)
        .await?;

    let mut headers = HeaderMap::new();
    let cookie_value = format!("session_token={}; Max-Age=3600", session_token);
    // Handle the unwrap here, I feel like this is unfaillable but might as well.
    // There used to be an error for this but now I don't know where.
    headers.insert("Set-Cookie", HeaderValue::from_str(&cookie_value).unwrap());

    Ok(headers)
}
pub async fn logout(
    State(state): State<Arc<SharedState>>,
    headers: HeaderMap,
) -> Result<(), ApiError> {
    let token = headers.get(COOKIE).unwrap();
    let token = extract_token(token.clone());

    state.database.delete_session(&token).await?;
    Ok(())
}

pub async fn check_session_token<T>(
    State(state): State<Arc<SharedState>>,
    request: Request<T>,
    next: axum::middleware::Next<T>,
) -> Result<Response, ApiError> {
    let token_option = request.headers().get(COOKIE);
    let token = match token_option {
        None => return Err(ApiError::MissingSessionTokenInClientRequest),
        Some(token) => extract_token(token.clone()),
    };

    state.database.get_session(&token).await?;

    Ok(next.run(request).await)
}
