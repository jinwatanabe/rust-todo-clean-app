use domain::todo::{Todos};

use crate::port::todo::TodoPort;

pub async fn get_all(
	todo_port: &impl TodoPort,
) -> anyhow::Result<Todos> {
	todo_port.get_all().await
}

#[cfg(test)]
mod tests {

	use crate::port::todo::MockTodoPort;

use super::*;

	#[tokio::test]
	async fn test_get_all() {
		
		let todos = Todos(vec![]);
		let mut news_port = MockTodoPort::default();
		news_port.mock_get_all().returns_with(|| Ok(Todos(vec![])));
		let actual = news_port.get_all().await;
		let expected = todos.clone();
		assert_eq!(actual.unwrap(), expected)
	}
}