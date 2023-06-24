use domain::todo::{Todos};

#[mry::mry]
#[async_trait::async_trait]
pub trait TodoPort {
	async fn get_all(&self) -> anyhow::Result<Todos>;
	async fn get_by_id(&self, id: domain::todo::TodoId) -> anyhow::Result<domain::todo::Todo>;
}