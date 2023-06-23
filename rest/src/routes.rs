use axum::routing::get;
use axum::{response::IntoResponse, Json, Router};
use serde::Serialize;

pub fn routes() -> Router {
    Router::new().route("/v1/todos", get(get_all))
}

async fn get_all() -> impl IntoResponse {
    let todos = TodosJson {
        todos: vec![
            TodoJson {
                id: 1,
                title: "Learn Axum".to_string(),
                completed: false,
            },
            TodoJson {
                id: 2,
                title: "Learn Tokio".to_string(),
                completed: false,
            },
        ],
    };

    Json(todos)
}

#[derive(Serialize)]
pub struct TodoJson {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

#[derive(Serialize)]
pub struct TodosJson {
    pub todos: Vec<TodoJson>,
}
