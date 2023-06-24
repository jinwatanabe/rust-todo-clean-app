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

#[cfg(test)]
mod tests {

	use domain::todo::{TodoId, TodoTitle, TodoDone};

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
}