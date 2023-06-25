use axum::Extension;
use axum::routing::{get, post, patch};
use axum::{response::IntoResponse, Json, Router, extract};
use domain::todo::{TodoId, TodoTitle};
use serde::Serialize;
use gateway::todo::TodoGateway;
use std::{sync::Arc};
use serde::{Deserialize};

pub struct Container {
    pub todo_gateway: TodoGateway,
}

pub fn routes() -> Router {
    Router::new()
        .route("/v1/todos", get(get_all))
        .route("/v1/todos/:id", get(get_by_id))
        .route("/v1/todos", post(create))
        .route("/v1/todos/:id", patch(update))
        .route("/v1/todos/:id", axum::routing::delete(delete))
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

async fn create(
    Extension(container): Extension<Arc<Container>>,
    extract::Json(title): extract::Json<Title>,
) -> impl IntoResponse {
    if title.title.is_empty() {
        return Json(domain::response::Response { message: "title is empty".to_string() });
    }

    let title = TodoTitle(title.title);
    let result = usecase::todo::create(&container.todo_gateway, title).await;
    let response = result.unwrap();
    Json(response)
}

async fn update(
    Extension(container): Extension<Arc<Container>>,
    path: axum::extract::Path<i32>,
    extract::Json(option): extract::Json<UpdateJson>,
) -> impl IntoResponse {
    
    if option.title.is_none() && option.done.is_none() {
        return Json(domain::response::Response{ message: "title and done are empty".to_string() });
    }

    let id = TodoId(path.0);
    let title = match option.title {
        Some(title) => Some(TodoTitle(title)),
        None => None,
    };
    let done = match option.done {
        Some(done) => Some(domain::todo::TodoDone(done)),
        None => None,
    };
    let result = usecase::todo::update(&container.todo_gateway, id, title, done).await;
    let response = result.unwrap();
    Json(response)
}

async fn delete(
    Extension(container): Extension<Arc<Container>>,
    path: axum::extract::Path<i32>,
) -> impl IntoResponse {
    let id = TodoId(path.0); // `path.into_inner()` を使用してi32の値を取得
    let result = usecase::todo::delete(&container.todo_gateway, id).await;

    let response = match result {
        Ok(_) => result.unwrap(),
        Err(_) => domain::response::Response { message: "user id not found".to_string() },
    };

    Json(response)
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

#[derive(Deserialize)]
struct Title {
    title: String,
}

#[derive(Deserialize)]
struct UpdateJson {
    title: Option<String>,
    done: Option<bool>,
}