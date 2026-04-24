use axum::Json;
use serde_json::json;
use crate::models::AppState;
use crate::models::{CreateTest, Test, UpdateTest};
use axum::extract::{Path, State};
use uuid::Uuid;

pub async fn home() -> Json<serde_json::Value> {
    Json(json!({
        "message": "Mini Test Server",
        "version": 1
    }))
}

pub async fn get_tests(
    State(state): State<AppState>,
) -> Json<Vec<Test>> {
    let tests = state.tests.lock().unwrap().clone();

    Json(tests)
}

pub async fn create_test(
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

pub async fn get_test_by_id(
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

pub async fn delete_test(
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

pub async fn update_test(
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
