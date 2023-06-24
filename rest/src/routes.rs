use axum::Extension;
use axum::routing::get;
use axum::{response::IntoResponse, Json, Router};
use serde::Serialize;
use gateway::todo::TodoGateway;
use std::{sync::Arc};

pub struct Container {
    pub todo_gateway: TodoGateway,
}

pub fn routes() -> Router {
    Router::new()
        .route("/v1/todos", get(get_all))
}

async fn get_all(
    Extension(container): Extension<Arc<Container>>,
) -> impl IntoResponse {
    let results = usecase::todo::get_all(&container.todo_gateway).await;
    let todos_json = results
        .unwrap()
        .into_iter()
        .map(|t| TodoJson {
            id: t.id.0,
            title: t.title.0,
            done: t.done.0,
        })
        .collect::<Vec<TodoJson>>();
    Json(todos_json)
}

#[derive(Serialize)]
pub struct TodoJson {
    pub id: i32,
    pub title: String,
    pub done: bool,
}

#[derive(Serialize)]
pub struct TodosJson {
    pub todos: Vec<TodoJson>,
}
