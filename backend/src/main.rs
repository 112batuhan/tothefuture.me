pub mod api;
pub mod entities;
pub mod queries;
pub mod resend;

use std::net::SocketAddr;
use std::time::Duration;

use api::authentication::{auto_login, check_session_token, login, logout, sign_up};
use api::emails::{create_email, get_emails, send_demo_email};
use api::SharedState;
use axum::http::{header, Method};
use axum::routing::{delete, get, post};
use axum::{middleware, Router};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let state = SharedState::new()
        .await
        .expect("Failed to initiate Shared State");

    let origins = ["http://localhost:5173".parse().unwrap()];

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::COOKIE, header::ALLOW, header::CONTENT_TYPE])
        .allow_origin(origins)
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600));

    let app = Router::new()
        .route("/send_email/:email_id", get(send_demo_email))
        .route("/auto_login", get(auto_login))
        .route("/get_emails", get(get_emails))
        .route("/create_email", post(create_email))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            check_session_token,
        ))
        .route("/logout", delete(logout))
        .route("/sign_up", post(sign_up))
        .route("/login", post(login))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3040));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
