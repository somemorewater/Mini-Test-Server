use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use serde_json::json;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct AppState {
    tests: Arc<Mutex<Vec<Test>>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Test {
    title: String,
    subject: String,
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({"status": "ok"}))
}

#[tokio::main]
async fn main() {
    let state = AppState {
        tests: Arc::new(Mutex::new(Vec::new())),
    };

    let app = Router::new()
        .route("/", get(home))
        .route("/tests", get(get_tests).post(create_test))
        .route("/health", get(health))
        .with_state(state);

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

async fn get_tests(
    State(state): State<AppState>,
) -> Json<Vec<Test>> {
    let tests = state.tests.lock().unwrap().clone();

    Json(tests)
}

async fn create_test(
    State(state): State<AppState>,
    Json(payload): Json<Test>,
) -> Json<serde_json::Value> {

    state.tests.lock().unwrap().push(payload);

    Json(json!({
        "message": "Test created",
    }))
}