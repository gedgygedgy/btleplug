use super::{
    jni::{global_jvm, objects::JScanResult},
    peripheral::Peripheral,
};
use crate::{
    api::{BDAddr, Central, CentralEvent, PeripheralProperties},
    common::adapter_manager::AdapterManager,
    Error, Result,
};
use async_trait::async_trait;
use futures::stream::Stream;
use jni::objects::JObject;
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

    pub fn report_scan_result(&self, scan_result: JObject) -> Result<Peripheral> {
        use std::convert::TryInto;

        let env = global_jvm().get_env()?;
        let scan_result = JScanResult::from_env(&env, scan_result)?;

        let properties: PeripheralProperties = scan_result.try_into()?;

        let peripheral = match self.manager.peripheral(properties.address) {
            Some(p) => p,
            None => {
                let peripheral = Peripheral::new(&env, properties.address)?;
                self.manager
                    .add_peripheral(properties.address, peripheral.clone());
                peripheral
            }
        };
        peripheral.report_properties(properties);

        Ok(peripheral)
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
        let env = global_jvm().get_env()?;
        let peripheral = Peripheral::new(&env, address)?;
        self.manager.add_peripheral(address, peripheral.clone());
        Ok(peripheral)
    }
}
