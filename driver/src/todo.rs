use serde::Deserialize;

#[mry::mry]
pub struct TodoDriver {}

#[mry::mry]
impl TodoDriver {
	pub async fn get_all(&self) -> anyhow::Result<TodosJson> {
		todo!("get_all")
	}
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TodosJson {
	pub todos: Vec<TodoJson>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TodoJson {
	pub id: u64,
	pub title: String,
	pub completed: bool,
}