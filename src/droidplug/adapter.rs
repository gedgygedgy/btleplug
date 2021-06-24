use super::{jni::global_jvm, peripheral::Peripheral};
use crate::{
    api::{BDAddr, Central, CentralEvent},
    common::adapter_manager::AdapterManager,
    Error, Result,
};
use async_trait::async_trait;
use futures::stream::Stream;
use std::pin::Pin;

#[derive(Clone)]
pub struct Adapter {
    manager: AdapterManager<Peripheral>,
}

impl Adapter {
    pub(crate) fn new() -> Self {
        Adapter {
            manager: AdapterManager::default(),
        }
    }

    pub fn add(&self, addr: BDAddr, peripheral: Peripheral) {
        self.manager.add_peripheral(addr, peripheral)
    }
}

#[async_trait]
impl Central for Adapter {
    type Peripheral = Peripheral;

    async fn events(&self) -> Result<Pin<Box<dyn Stream<Item = CentralEvent>>>> {
        Ok(self.manager.event_stream())
    }

    async fn start_scan(&self) -> Result<()> {
        Err(Error::NotSupported("TODO".to_string()))
    }

    async fn stop_scan(&self) -> Result<()> {
        Err(Error::NotSupported("TODO".to_string()))
    }

    async fn peripherals(&self) -> Result<Vec<Peripheral>> {
        Ok(self.manager.peripherals())
    }

    async fn peripheral(&self, address: BDAddr) -> Result<Peripheral> {
        self.manager
            .peripheral(address)
            .ok_or(Error::DeviceNotFound)
    }

    async fn add_peripheral(&self, address: BDAddr) -> Result<Peripheral> {
        let guard = global_jvm().attach_current_thread()?;
        let peripheral = Peripheral::new(&guard, address)?;
        self.add(address, peripheral.clone());
        Ok(peripheral)
    }
}
