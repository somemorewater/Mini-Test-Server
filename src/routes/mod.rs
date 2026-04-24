use axum::{Router, routing::get};

use crate::{
    handlers::{create_test, delete_test, get_test_by_id, get_tests, home, update_test},
    health::health,
    models::AppState,
};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(home))
        .route("/tests", get(get_tests).post(create_test))
        .route(
            "/tests/{id}",
            get(get_test_by_id).delete(delete_test).put(update_test),
        )
        .route("/health", get(health))
        .with_state(state)
}
