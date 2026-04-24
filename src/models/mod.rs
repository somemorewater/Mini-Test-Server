use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub tests: Arc<Mutex<Vec<Test>>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateTest {
    pub title: String,
    pub subject: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Test {
    pub id: Uuid,
    pub title: String,
    pub subject: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTest {
    pub title: Option<String>,
    pub subject: Option<String>,
}