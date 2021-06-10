use crate::{
    api::{self, BDAddr, Characteristic, PeripheralProperties, ValueNotification, WriteType},
    Error, Result,
};
use async_trait::async_trait;
use futures::stream::Stream;
use std::collections::BTreeSet;
use std::pin::Pin;

#[derive(Clone, Debug)]
pub struct Peripheral;

#[async_trait]
impl api::Peripheral for Peripheral {
    fn address(&self) -> BDAddr {
        panic!("TODO")
    }

    async fn properties(&self) -> Result<Option<PeripheralProperties>> {
        Err(Error::NotSupported("TODO".to_string()))
    }

    fn characteristics(&self) -> BTreeSet<Characteristic> {
        panic!("TODO")
    }

    async fn is_connected(&self) -> Result<bool> {
        Err(Error::NotSupported("TODO".to_string()))
    }

    async fn connect(&self) -> Result<()> {
        Err(Error::NotSupported("TODO".to_string()))
    }

    async fn disconnect(&self) -> Result<()> {
        Err(Error::NotSupported("TODO".to_string()))
    }

    async fn discover_characteristics(&self) -> Result<Vec<Characteristic>> {
        Err(Error::NotSupported("TODO".to_string()))
    }

    async fn write(
        &self,
        characteristic: &Characteristic,
        data: &[u8],
        write_type: WriteType,
    ) -> Result<()> {
        Err(Error::NotSupported("TODO".to_string()))
    }

    async fn read(&self, characteristic: &Characteristic) -> Result<Vec<u8>> {
        Err(Error::NotSupported("TODO".to_string()))
    }

    async fn subscribe(&self, characteristic: &Characteristic) -> Result<()> {
        Err(Error::NotSupported("TODO".to_string()))
    }

    async fn unsubscribe(&self, characteristic: &Characteristic) -> Result<()> {
        Err(Error::NotSupported("TODO".to_string()))
    }

    async fn notifications(&self) -> Result<Pin<Box<dyn Stream<Item = ValueNotification>>>> {
        Err(Error::NotSupported("TODO".to_string()))
    }
}
