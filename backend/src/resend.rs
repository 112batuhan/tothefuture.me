use std::sync::Arc;

use lazy_static::lazy_static;
use reqwest::StatusCode;
use serde::Serialize;
use thiserror::Error;

lazy_static! {
    static ref RESEND_API_KEY: Arc<String> = Arc::new(
        std::env::var("RESEND_API_TOKEN")
            .expect("Environment variable RESEND_API_TOKEN is not set.")
    );
    static ref RESEND_EMAIL_ADDRESS: String = std::env::var("RESEND_EMAIL_ADDRESS")
        .expect("Environment variable RESEND_EMAIL_ADDRESS is not set.");
}

#[derive(Error, Debug)]
pub enum ResendError {
    #[error("General Error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Response returned with error: {0}")]
    Response(StatusCode),
}

#[derive(Serialize, Debug)]
pub struct RequestBody {
    from: &'static str,
    to: String,
    subject: String,
    html: String,
}

pub struct ExternalRequest {
    client: reqwest::Client,
}

impl ExternalRequest {
    pub fn new() -> Self {
        ExternalRequest {
            client: reqwest::Client::new(),
        }
    }

    pub async fn send_email(
        &self,
        to: String,
        subject: String,
        html: String,
    ) -> Result<(), ResendError> {
        let request_body = RequestBody {
            from: &RESEND_EMAIL_ADDRESS,
            to,
            subject,
            html,
        };
        let res = self
            .client
            .post("https://api.resend.com/emails")
            .bearer_auth(RESEND_API_KEY.clone())
            .json(&request_body)
            .send()
            .await?;
        res.error_for_status_ref()
            .map_err(|error| ResendError::Response(error.status().unwrap()))?;
        Ok(())
    }
}
