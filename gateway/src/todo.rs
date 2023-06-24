
use driver::todo::TodoDriver;
use domain::todo::{Todo, TodoId, TodoTitle, TodoDone, Todos};
use usecase::port::todo::TodoPort;

pub struct TodoGateway {
	driver: TodoDriver
}

#[async_trait::async_trait]
impl TodoPort for TodoGateway {
	async fn get_all(&self) -> anyhow::Result<Todos> {
		let json = self.driver.get_all().await?;
		let results = json
			.todos
			.into_iter()
			.map(|t| Todo {
				id: TodoId(t.id),
				title: TodoTitle(t.title),
				done: TodoDone(t.completed),
			})
			.collect::<Vec<Todo>>();

		Ok(Todos(results))
	}
}

#[cfg(test)]
mod tests {
	use driver::todo::TodosJson;

use super::*;

	#[tokio::test]
	async fn test_get_all() {
		let mut driver = mry::new!(TodoDriver {}	);
		driver.mock_get_all().returns_with(|| Ok(TodosJson{todos: vec![]}));

		let gateway = TodoGateway { driver };
		let actual = gateway.get_all().await;
		let expected = Todos(vec![]);

		assert_eq!(actual.unwrap(), expected)
	}
}