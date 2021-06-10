use super::peripheral::Peripheral;
use crate::{
    api::{BDAddr, Central, CentralEvent},
    Error, Result,
};
use async_trait::async_trait;
use futures::stream::Stream;
use std::pin::Pin;

#[derive(Clone)]
pub struct Adapter;

#[async_trait]
impl Central for Adapter {
    type Peripheral = Peripheral;

    async fn events(&self) -> Result<Pin<Box<dyn Stream<Item = CentralEvent>>>> {
        Err(Error::NotSupported("TODO".to_string()))
    }

    async fn start_scan(&self) -> Result<()> {
        Err(Error::NotSupported("TODO".to_string()))
    }

    async fn stop_scan(&self) -> Result<()> {
        Err(Error::NotSupported("TODO".to_string()))
    }

    async fn peripherals(&self) -> Result<Vec<Peripheral>> {
        Err(Error::NotSupported("TODO".to_string()))
    }

    async fn peripheral(&self, address: BDAddr) -> Result<Peripheral> {
        Err(Error::NotSupported("TODO".to_string()))
    }
}
