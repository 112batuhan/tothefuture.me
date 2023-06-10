pub mod api;
pub mod database;

use std::net::SocketAddr;

use api::authentication::{check_session_token, logout, sign_in, sign_up};
use api::SharedState;
use axum::http::{header, Method};
use axum::routing::{delete, get, post};
use axum::{middleware, Router};
use tower_http::cors::{Any, CorsLayer};

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
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([header::COOKIE])
        .allow_origin(origins)
        .allow_credentials(true);

    let app = Router::new()
        .route("/logout", delete(logout))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            check_session_token,
        ))
        .route("/sign_up", post(sign_up))
        .route("/sign_in", post(sign_in))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3040));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
