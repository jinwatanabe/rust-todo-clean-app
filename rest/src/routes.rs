use axum::Extension;
use axum::routing::get;
use axum::{response::IntoResponse, Json, Router};
use domain::todo::TodoId;
use serde::Serialize;
use gateway::todo::TodoGateway;
use std::{sync::Arc};

pub struct Container {
    pub todo_gateway: TodoGateway,
}

pub fn routes() -> Router {
    Router::new()
        .route("/v1/todos", get(get_all))
        .route("/v1/todos/:id", get(get_by_id))
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

async fn get_by_id(
    Extension(container): Extension<Arc<Container>>,
    path: axum::extract::Path<i32>,
) -> impl IntoResponse {
    let id = TodoId(path.0);
    let result = usecase::todo::get_by_id(&container.todo_gateway, id).await;
    let todo = result.unwrap();
    let todo_json = TodoJson {
        id: todo.id.0,
        title: todo.title.0,
        done: todo.done.0,
    };
    Json(todo_json)
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
