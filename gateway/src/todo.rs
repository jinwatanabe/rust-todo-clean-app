
use driver::todo::TodoDriver;
use domain::{todo::{Todo, TodoId, TodoTitle, TodoDone, Todos}, response::Response};
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
				done: TodoDone(t.done),
			})
			.collect::<Vec<Todo>>();

		Ok(Todos(results))
	}

	async fn get_by_id(&self, id: TodoId) -> anyhow::Result<Todo> {
		let json = self.driver.get_by_id(id.0).await?;
		let result = Todo {
			id: TodoId(json.id),
			title: TodoTitle(json.title),
			done: TodoDone(json.done),
		};

		Ok(result)
	}

	async fn create(&self, title: TodoTitle) -> anyhow::Result<Response> {
		let json = self.driver.create(title.0).await?;
		let result = Response {
			message: json.message,
		};

		Ok(result)
	}

	async fn update(&self, id: TodoId, title: Option<TodoTitle>, done: Option<TodoDone>) -> anyhow::Result<Response> {
		let title = match title {
			Some(title) => Some(title.0),
			None => None,
		};
		let done = match done {
			Some(done) => Some(done.0),
			None => None,
		};
		let json = self.driver.update(id.0, title, done).await?;
		let result = Response {
			message: json.message,
		};

		Ok(result)
	}

	async fn delete(&self, id: TodoId) -> anyhow::Result<Response> {
		let json = self.driver.delete(id.0).await?;
		let result = Response {
			message: json.message,
		};

		Ok(result)
	}
}

impl TodoGateway {
    pub fn new(todo_driver: TodoDriver) -> Self {
        TodoGateway { driver: todo_driver }
    }
}

#[cfg(test)]
mod tests {
	use domain::response::Response;
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

	#[tokio::test]
	async fn test_get_by_id() {
		let mut driver = mry::new!(TodoDriver {});
		driver.mock_get_by_id(1).returns_with(|_| Ok(driver::schema::Todo{
			id: 1,
			title: "title".to_string(),
			done: false,
		}));
		
		let gateway = TodoGateway { driver };
		let actual = gateway.get_by_id(TodoId(1)).await;
		let expected = Todo {
			id: TodoId(1),
			title: TodoTitle("title".to_string()),
			done: TodoDone(false),
		};

		assert_eq!(actual.unwrap(), expected)
	}

	#[tokio::test]
	async fn test_create() {
		let mut driver = mry::new!(TodoDriver {});
		driver.mock_create("title".to_string()).returns_with(|_| Ok(Response { message: "ok".to_string() }));
		let gateway = TodoGateway { driver };
		let actual = gateway.create(TodoTitle("title".to_string())).await;
		let expected = Response{ message: "ok".to_string()};
		
		assert_eq!(actual.unwrap(), expected)
	}

	#[tokio::test]
	async fn test_update() {
		let mut driver = mry::new!(TodoDriver {});
		driver.mock_update(1, Some("title".to_string()), Some(false)).returns_with(|_, _, _| Ok(Response { message: "ok".to_string() }));
		let gateway = TodoGateway { driver };
		let actual = gateway.update(TodoId(1), Some(TodoTitle("title".to_string())), Some(TodoDone(false))).await;
		let expected = Response{ message: "ok".to_string()};

		assert_eq!(actual.unwrap(), expected)
	}

	#[tokio::test]
	async fn test_delete() {
		let mut driver = mry::new!(TodoDriver {});
		driver.mock_delete(1).returns_with(|_| Ok(Response { message: "ok".to_string() }));
		let gateway = TodoGateway { driver };
		let actual = gateway.delete(TodoId(1)).await;
		let expected = Response{ message: "ok".to_string()};

		assert_eq!(actual.unwrap(), expected)
	}
}