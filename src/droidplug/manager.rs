use super::adapter::Adapter;
use crate::{api, Result};
use async_trait::async_trait;

#[derive(Clone)]
pub struct Manager;

#[async_trait]
impl api::Manager for Manager {
    type Adapter = Adapter;

    async fn adapters(&self) -> Result<Vec<Adapter>> {
        vec![Adapter::new()]
    }
}
