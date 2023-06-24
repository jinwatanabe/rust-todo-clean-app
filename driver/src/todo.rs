use diesel::Queryable;
use serde::Deserialize;
use utils::establish_connection;
use crate::utils;
use diesel::RunQueryDsl;
#[mry::mry]
pub struct TodoDriver {}

#[mry::mry]
impl TodoDriver {
	pub async fn get_all(&self) -> anyhow::Result<TodosJson> {
		use crate::schema::todos::dsl::todos;
		let connection = establish_connection();
		let results = todos.load::<Todo>(&connection).expect("Error loading todos");
		Ok(TodosJson { todos: results })
	}
}

#[derive(Deserialize, Debug)]
pub struct TodosJson {
	pub todos: Vec<Todo>,
}

#[derive(Deserialize, Debug, Queryable)]
pub struct Todo {
	pub id: i32,
	pub title: String,
	pub completed: bool,
}
