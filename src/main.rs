use axum::{
    Json, Router,
    extract::State,
    extract::Path,
    routing::{get, delete, put},
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

#[derive(Serialize, Deserialize)]
struct UpdateTest {
    title: Option<String>,
    subject: Option<String>,
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
        .route("/tests/{id}", get(get_test_by_id))
        .route("/tests/{id}", delete(delete_test))
        .route("/tests/{id}", put(update_test))
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
    Json(payload): Json<CreateTest>,
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

async fn get_test_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {

    let tests = state.tests.lock().unwrap();

    let test = tests.iter().find(|t| t.id == id);

    match test {
        Some(t) => Json(json!({
            "data": t
        })),
        None => Json(json!({
            "error": "Test not found"
        })),
    }
}

async fn delete_test(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let mut tests = state.tests.lock().unwrap();

    let initial_len = tests.len();

    tests.retain(|t| t.id != id);

    let deleted = tests.len() < initial_len;

    Json(json!({
        "success": deleted,
        "message": if deleted { "Test deleted" } else { "Test not found" }
    }))
}

async fn update_test(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTest>,
) -> Json<serde_json::Value> {

    let mut tests = state.tests.lock().unwrap();

    let test = tests.iter_mut().find(|t| t.id == id);

    match test {
        Some(t) => {
            if let Some(title) = payload.title {
                t.title = title;
            }

            if let Some(subject) = payload.subject {
                t.subject = subject;
            }

            Json(json!({
                "success": true,
                "message": "Test updated",
                "data": t
            }))
        }

        None => Json(json!({
            "success": false,
            "message": "Test not found"
        }))
    }
}