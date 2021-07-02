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
use jni::objects::{GlobalRef, JObject};
use std::pin::Pin;

#[derive(Clone)]
pub struct Adapter {
    manager: AdapterManager<Peripheral>,
    internal: GlobalRef,
}

impl Adapter {
    pub(crate) fn new() -> Result<Self> {
        let env = global_jvm().get_env()?;

        let obj = env.new_object(
            "com/nonpolynomial/btleplug/android/impl/Adapter",
            "()V",
            &[],
        )?;
        let internal = env.new_global_ref(obj)?;
        let adapter = Self {
            manager: AdapterManager::default(),
            internal,
        };
        env.set_rust_field(obj, "handle", adapter.clone())?;

        Ok(adapter)
    }

    pub fn report_scan_result(&self, scan_result: JObject) -> Result<Peripheral> {
        use std::convert::TryInto;

        let env = global_jvm().get_env()?;
        let scan_result = JScanResult::from_env(&env, scan_result)?;

        let (addr, properties): (BDAddr, Option<PeripheralProperties>) = scan_result.try_into()?;

        match self.manager.peripheral(addr) {
            Some(p) => match properties {
                Some(properties) => {
                    p.report_properties(properties);
                    self.manager.emit(CentralEvent::DeviceUpdated(addr));
                    Ok(p)
                }
                None => {
                    self.manager.emit(CentralEvent::DeviceLost(addr));
                    Err(Error::DeviceNotFound)
                }
            },
            None => match properties {
                Some(properties) => {
                    let p = self.add(addr)?;
                    p.report_properties(properties);
                    self.manager.emit(CentralEvent::DeviceDiscovered(addr));
                    Ok(p)
                }
                None => Err(Error::DeviceNotFound),
            },
        }
    }

    fn add(&self, address: BDAddr) -> Result<Peripheral> {
        let env = global_jvm().get_env()?;
        let peripheral = Peripheral::new(&env, address)?;
        self.manager.add_peripheral(address, peripheral.clone());
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
        let env = global_jvm().get_env()?;
        env.call_method(&self.internal, "startScan", "()V", &[])?;
        Ok(())
    }

    async fn stop_scan(&self) -> Result<()> {
        let env = global_jvm().get_env()?;
        env.call_method(&self.internal, "stopScan", "()V", &[])?;
        Ok(())
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
        self.add(address)
    }
}
