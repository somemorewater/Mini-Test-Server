use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use serde_json::json;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    tests: Arc<Mutex<Vec<Test>>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct CreateTest {
    title: String,
    subject: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Test {
    id: Uuid,
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
    Json(mut payload): Json<CreateTest>,
) -> Json<serde_json::Value> {

    let test = Test {
        id: Uuid::new_v4(),
        title: payload.title,
        subject: payload.subject,
    };

    let id = test.id;

    state.tests.lock().unwrap().push(test);

    Json(json!({
        "message": "Test created",
        "id": id
    }))
}