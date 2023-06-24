use domain::todo::{Todos};

#[mry::mry]
#[async_trait::async_trait]
pub trait TodoPort {
	async fn get_all(&self) -> anyhow::Result<Todos>;
}