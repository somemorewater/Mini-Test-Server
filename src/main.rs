use axum::{Json, Router, routing::get, serve::Listener};
use serde_json::json;

async fn health() -> Json<serde_json::Value> {
    Json(json!({"status": "ok"}))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on port 3000");

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Json<serde_json::Value> {
    Json(json!({
        "message": "Mini Test Server",
        "version": 1
    }))
}