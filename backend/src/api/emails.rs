use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::{Extension, Json};
use mail_parser::Message;
use serde::{Deserialize, Serialize};

use super::{ApiError, CurrentUser, SharedState};
use crate::entities::emails;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestEmailBody {
    is_html: bool,
    email: String,
    date: String,
}

pub async fn create_email(
    Extension(session): Extension<CurrentUser>,
    State(state): State<Arc<SharedState>>,
    Json(request_body): Json<RequestEmailBody>,
) -> Result<(), ApiError> {
    let message = Message::parse(request_body.email.as_bytes()).ok_or(ApiError::BadEmail)?;
    let subject = message.subject().ok_or(ApiError::BadEmail)?;
    let body = message.body_html(0).ok_or(ApiError::BadEmail)?;
    let send_date = chrono::NaiveDate::from_str(&request_body.date)?;

    state
        .database
        .create_email(
            session.get_user_id(),
            subject.to_string(),
            // Rework later to allow md aswell.
            true,
            body.into(),
            send_date,
        )
        .await?;

    Ok(())
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
    Path(email_id): Path<i64>,
    Extension(session): Extension<CurrentUser>,
    State(state): State<Arc<SharedState>>,
) -> Result<(), ApiError> {
    let email = state.database.get_emails_by_id(email_id).await?;
    if email.owner != session.get_user_id() {
        // handle error
    }
    let user = state.database.get_user_by_id(session.get_user_id()).await?;
    state
        .external_request
        .send_email(user.email, email.subject, email.body)
        .await?;
    Ok(())
}
