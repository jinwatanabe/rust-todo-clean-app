use domain::todo::{Todos};

#[mry::mry]
#[async_trait::async_trait]
pub trait TodoPort {
	async fn get_all(&self) -> anyhow::Result<Todos>;
	async fn get_by_id(&self, id: domain::todo::TodoId) -> anyhow::Result<domain::todo::Todo>;
	async fn create(&self, title: domain::todo::TodoTitle) -> anyhow::Result<domain::response::Response>;
	async fn update(&self, id: domain::todo::TodoId, title: Option<domain::todo::TodoTitle>, done: Option<domain::todo::TodoDone>) -> anyhow::Result<domain::response::Response>;
}