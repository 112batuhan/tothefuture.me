use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::Request;
use axum::response::Response;
use axum::{Extension, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use super::{ApiError, CurrentUser, SharedState};
use crate::entities::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestEmailBody {
    is_html: bool,
    subject: String,
    body: String,
    date: String,
}

fn check_and_set_date(raw_date: String) -> Result<chrono::NaiveDate, ApiError> {
    let send_date = chrono::NaiveDate::from_str(&raw_date)?;
    let current_date: chrono::NaiveDate = chrono::Utc::now().naive_utc().into();
    if current_date > send_date {
        return Ok(current_date);
    }
    Ok(send_date)
}

pub async fn create_email(
    Extension(session): Extension<CurrentUser>,
    State(state): State<Arc<SharedState>>,
    Json(request_body): Json<RequestEmailBody>,
) -> Result<StatusCode, ApiError> {
    let send_date = check_and_set_date(request_body.date)?;
    state
        .database
        .create_email(
            session.get_user_id(),
            request_body.subject,
            request_body.is_html,
            request_body.body,
            send_date,
        )
        .await?;

    Ok(StatusCode::CREATED)
}

fn filter_hidden_emails(emails: Vec<emails::Model>) -> Vec<emails::Model> {
    emails.into_iter();
    todo!()
}

pub async fn get_emails(
    Extension(session): Extension<CurrentUser>,
    State(state): State<Arc<SharedState>>,
) -> Result<Json<Vec<emails::Model>>, ApiError> {
    let email_list = state
        .database
        .get_emails_by_user(session.get_user_id())
        .await?;

    Ok(Json(email_list))
}

pub async fn send_demo_email(
    Extension(email): Extension<emails::Model>,
    Extension(session): Extension<CurrentUser>,
    State(state): State<Arc<SharedState>>,
) -> Result<(), ApiError> {
    let user = state.database.get_user_by_id(session.get_user_id()).await?;
    state
        .external_request
        .send_email(user.email, email.subject, email.body)
        .await?;
    Ok(())
}

pub async fn update_email(
    Extension(email): Extension<emails::Model>,
    State(state): State<Arc<SharedState>>,
    Json(request_body): Json<RequestEmailBody>,
) -> Result<(), ApiError> {
    let send_date = check_and_set_date(request_body.date)?;
    state
        .database
        .update_email(
            email,
            request_body.subject,
            request_body.is_html,
            request_body.body,
            send_date,
        )
        .await?;

    Ok(())
}

pub async fn duplicate_email(
    Extension(email): Extension<emails::Model>,
    State(state): State<Arc<SharedState>>,
) -> Result<StatusCode, ApiError> {
    state.database.duplicate_email(email).await?;
    Ok(StatusCode::CREATED)
}

pub async fn delete_email(
    Extension(email): Extension<emails::Model>,
    State(state): State<Arc<SharedState>>,
) -> Result<(), ApiError> {
    state.database.delete_email(email).await?;
    Ok(())
}

pub async fn hide_email(
    Extension(email): Extension<emails::Model>,
    State(state): State<Arc<SharedState>>,
) -> Result<(), ApiError> {
    state.database.hide_email(email).await?;
    Ok(())
}

pub async fn check_email_owner<T>(
    Path(email_id): Path<i64>,
    Extension(session): Extension<CurrentUser>,
    State(state): State<Arc<SharedState>>,
    mut request: Request<T>,
    next: axum::middleware::Next<T>,
) -> Result<Response, ApiError> {
    let email = state.database.get_email_by_id(email_id).await?;
    if email.owner != session.get_user_id() {
        return Err(ApiError::UnauthorizedEmail);
    }
    request.extensions_mut().insert(email);
    Ok(next.run(request).await)
}
