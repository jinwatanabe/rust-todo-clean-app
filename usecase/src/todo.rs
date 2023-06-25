use domain::todo::{Todos, Todo};

use crate::port::todo::TodoPort;

pub async fn get_all(
	todo_port: &impl TodoPort,
) -> anyhow::Result<Todos> {
	todo_port.get_all().await
}

pub async fn get_by_id(
	todo_port: &impl TodoPort,
	id: domain::todo::TodoId,
) -> anyhow::Result<Todo> {
	todo_port.get_by_id(id).await
}

pub async fn create(
	todo_port: &impl TodoPort,
	title: domain::todo::TodoTitle,
) -> anyhow::Result<domain::response::Response> {
	todo_port.create(title).await
}

pub async fn update(
	todo_port: &impl TodoPort,
	id: domain::todo::TodoId,
	title: Option<domain::todo::TodoTitle>,
	done: Option<domain::todo::TodoDone>,
) -> anyhow::Result<domain::response::Response> {
	todo_port.update(id, title, done).await
}

pub async fn delete(
	todo_port: &impl TodoPort,
	id: domain::todo::TodoId,
) -> anyhow::Result<domain::response::Response> {
	todo_port.delete(id).await
}

#[cfg(test)]
mod tests {

	use domain::{todo::{TodoId, TodoTitle, TodoDone}, response::Response};

use crate::port::todo::MockTodoPort;

use super::*;

	#[tokio::test]
	async fn test_get_all() {
		
		let todos = Todos(vec![]);
		let mut todo_port = MockTodoPort::default();
		todo_port.mock_get_all().returns_with(|| Ok(Todos(vec![])));
		let actual = todo_port.get_all().await;
		let expected = todos.clone();
		assert_eq!(actual.unwrap(), expected)
	}

	#[tokio::test]
	async fn test_get_by_id() {

		let mut todo_port = MockTodoPort::default();
		todo_port.mock_get_by_id(TodoId(1)).returns_with(|_| Ok(Todo{
			id: TodoId(1),
			title: TodoTitle("".to_string()),
			done: TodoDone(false),
		}));
		let actual = todo_port.get_by_id(TodoId(1)).await;
		let expected = Todo{
			id: TodoId(1),
			title: TodoTitle("".to_string()),
			done: TodoDone(false),
		};

		assert_eq!(actual.unwrap(), expected)
	}

	#[tokio::test]
	async fn create() {
		let mut todo_port = MockTodoPort::default();
		todo_port.mock_create(TodoTitle("title".to_string())).returns_with(|_| Ok(Response{ message: "ok".to_string() }));
		let actual = todo_port.create(TodoTitle("title".to_string())).await;
		let expected = Response{ message: "ok".to_string() };
		assert_eq!(actual.unwrap(), expected)
	}

	#[tokio::test]
	async fn update() {
		let mut todo_port = MockTodoPort::default();
		todo_port.mock_update(TodoId(1), Some(TodoTitle("title".to_string())), Some(TodoDone(true))).returns_with(|_, _, _| Ok(Response{ message: "ok".to_string() }));
		let actual = todo_port.update(TodoId(1), Some(TodoTitle("title".to_string())), Some(TodoDone(true))).await;
		let expected = Response{ message: "ok".to_string() };
		assert_eq!(actual.unwrap(), expected)
	}

	#[tokio::test]
	async fn delete() {
		let mut todo_port = MockTodoPort::default();
		todo_port.mock_delete(TodoId(1)).returns_with(|_| Ok(Response{ message: "ok".to_string() }));
		let actual = todo_port.delete(TodoId(1)).await;
		let expected = Response{ message: "ok".to_string() };
		assert_eq!(actual.unwrap(), expected)
	}
}