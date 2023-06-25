use diesel::{Queryable, QueryDsl};
use domain::response::Response;
use serde::Deserialize;
use utils::establish_connection;
use crate::{utils};
use diesel::RunQueryDsl;
use diesel::prelude::*;

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

	pub async fn get_by_id(&self, id_: i32) -> anyhow::Result<Todo> {
		use crate::schema::todos::dsl::*;
		let connection = establish_connection();
		let result = todos.filter(id.eq(id_)).first::<Todo>(&connection)?;
		Ok(result)
	}

pub async fn create(&self, title: String) -> anyhow::Result<Response> {
		todo!()
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
	pub done: bool,
}
