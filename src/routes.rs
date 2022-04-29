/// Module for GET/POST requests from shell frontend
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Returns a an AXUM router with the configured routes
/// for handling HTTP requests
pub async fn setup_routes(port: u32) -> Router {
    println!("Listening on http://localhost:{:}", port);
    let app = Router::new()
        .route("/", get(init_ctx_mgr))
        .route("/create", post(init_ctx_mgr_json))
        .route("/exec/:id", post(exec_command));

    // axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
    app
}

async fn init_ctx_mgr() -> String {
    "Hello world".to_string()
}

async fn init_ctx_mgr_json() -> Json<String> {
    Json("Create with ".to_string())
}

async fn exec_command() -> String {
    "Executing command".to_string()
}
