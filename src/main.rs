mod handlers;
mod models;
mod routes;
mod health;

use models::AppState;
use std::sync::{Arc, Mutex};
use routes::create_routes;

#[tokio::main]
async fn main() {
    let state = AppState {
        tests: Arc::new(Mutex::new(Vec::new())),
    };

    let app = create_routes(state);
        

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on port 3000");

    axum::serve(listener, app).await.unwrap();
}
