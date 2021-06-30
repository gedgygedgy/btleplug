use crate::{
    api::{self, BDAddr, Characteristic, PeripheralProperties, ValueNotification, WriteType},
    Error, Result,
};
use async_trait::async_trait;
use futures::stream::Stream;
use jni::{
    objects::{GlobalRef, JList, JThrowable},
    JNIEnv,
};
use jni_utils::{future::JavaFuture, stream::JavaStream, uuid::JUuid};
use std::{
    collections::BTreeSet,
    convert::TryFrom,
    fmt::{Debug, Formatter},
    pin::Pin,
};

use super::jni::{
    global_jvm,
    objects::{JBluetoothGattCharacteristic, JPeripheral},
};

#[derive(Clone)]
pub struct Peripheral {
    addr: BDAddr,
    internal: GlobalRef,
}

impl Peripheral {
    pub(crate) fn new(env: &JNIEnv, addr: BDAddr) -> Result<Self> {
        let obj = JPeripheral::new(env, addr)?;
        Ok(Self {
            addr,
            internal: env.new_global_ref(obj)?,
        })
    }

    fn with_obj<T, E>(
        &self,
        f: impl FnOnce(&JNIEnv, JPeripheral) -> std::result::Result<T, E>,
    ) -> std::result::Result<T, E>
    where
        E: From<::jni::errors::Error>,
    {
        let env = global_jvm().get_env()?;
        let obj = JPeripheral::from_env(&env, self.internal.as_obj())?;
        f(&env, obj)
    }

    async fn set_characteristic_notification(
        &self,
        characteristic: &Characteristic,
        enable: bool,
    ) -> Result<()> {
        let future = self.with_obj(|env, obj| {
            let uuid_obj = JUuid::new(env, characteristic.uuid)?;
            JavaFuture::try_from(obj.set_characteristic_notification(uuid_obj, enable)?)
        })?;
        match future.await? {
            Ok(_) => Ok(()),
            Err(ex) => self.with_obj(|env, _obj| {
                let ex: JThrowable = ex.as_obj().into();
                Err(
                    if env.is_instance_of(
                        ex,
                        "com/nonpolynomial/btleplug/android/impl/NotConnectedException",
                    )? {
                        Error::NotConnected
                    } else if env.is_instance_of(
                        ex,
                        "com/nonpolynomial/btleplug/android/impl/PermissionDeniedException",
                    )? {
                        Error::PermissionDenied
                    } else {
                        env.throw(ex)?; // Something else, so pass it back to Java.
                        Error::Other(Box::new(::jni::errors::Error::JavaException))
                    },
                )
            }),
        }
    }
}

impl Debug for Peripheral {
    fn fmt(&self, fmt: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self.internal.as_obj())
    }
}

#[async_trait]
impl api::Peripheral for Peripheral {
    fn address(&self) -> BDAddr {
        self.addr
    }

    async fn properties(&self) -> Result<Option<PeripheralProperties>> {
        Err(Error::NotSupported("TODO".to_string()))
    }

    fn characteristics(&self) -> BTreeSet<Characteristic> {
        panic!("TODO")
    }

    async fn is_connected(&self) -> Result<bool> {
        self.with_obj(|_env, obj| Ok(obj.is_connected()?))
    }

    async fn connect(&self) -> Result<()> {
        let future = self.with_obj(|_env, obj| JavaFuture::try_from(obj.connect()?))?;
        match future.await? {
            Ok(_) => Ok(()),
            Err(ex) => self.with_obj(|env, _obj| {
                let ex: JThrowable = ex.as_obj().into();
                Err(
                    if env.is_instance_of(
                        ex,
                        "com/nonpolynomial/btleplug/android/impl/NotConnectedException",
                    )? {
                        Error::NotConnected
                    } else if env.is_instance_of(
                        ex,
                        "com/nonpolynomial/btleplug/android/impl/PermissionDeniedException",
                    )? {
                        Error::PermissionDenied
                    } else {
                        env.throw(ex)?; // Something else, so pass it back to Java.
                        Error::Other(Box::new(::jni::errors::Error::JavaException))
                    },
                )
            }),
        }
    }

    async fn disconnect(&self) -> Result<()> {
        let future = self.with_obj(|_env, obj| JavaFuture::try_from(obj.disconnect()?))?;
        match future.await? {
            Ok(_) => Ok(()),
            Err(ex) => self.with_obj(|env, _obj| {
                let ex: JThrowable = ex.as_obj().into();
                env.throw(ex)?;
                Err(Error::Other(Box::new(::jni::errors::Error::JavaException)))
            }),
        }
    }

    async fn discover_characteristics(&self) -> Result<Vec<Characteristic>> {
        let future =
            self.with_obj(|_env, obj| JavaFuture::try_from(obj.discover_characteristics()?))?;
        match future.await? {
            Ok(obj) => self.with_obj(|env, _obj| {
                let list = JList::from_env(env, obj.as_obj())?;
                let mut result = Vec::new();
                for characteristic in list.iter()? {
                    let characteristic =
                        JBluetoothGattCharacteristic::from_env(env, characteristic)?;
                    result.push(Characteristic {
                        uuid: characteristic.get_uuid()?,
                        properties: characteristic.get_properties()?,
                    });
                }
                Ok(result)
            }),
            Err(ex) => self.with_obj(|env, _obj| {
                let ex: JThrowable = ex.as_obj().into();
                env.throw(ex)?;
                Err(Error::Other(Box::new(::jni::errors::Error::JavaException)))
            }),
        }
    }

    async fn write(
        &self,
        characteristic: &Characteristic,
        data: &[u8],
        write_type: WriteType,
    ) -> Result<()> {
        let future = self.with_obj(|env, obj| {
            let uuid = JUuid::new(env, characteristic.uuid)?;
            let data_obj = jni_utils::arrays::slice_to_byte_array(env, data)?;
            let write_type = match write_type {
                WriteType::WithResponse => 2,
                WriteType::WithoutResponse => 1,
            };
            JavaFuture::try_from(obj.write(uuid, data_obj.into(), write_type)?)
        })?;
        match future.await? {
            Ok(_) => Ok(()),
            Err(ex) => self.with_obj(|env, _obj| {
                let ex: JThrowable = ex.as_obj().into();
                env.throw(ex)?;
                Err(Error::Other(Box::new(::jni::errors::Error::JavaException)))
            }),
        }
    }

    async fn read(&self, characteristic: &Characteristic) -> Result<Vec<u8>> {
        let future = self.with_obj(|env, obj| {
            let uuid = JUuid::new(env, characteristic.uuid)?;
            JavaFuture::try_from(obj.read(uuid)?)
        })?;
        match future.await? {
            Ok(result) => self.with_obj(|env, _obj| {
                Ok(jni_utils::arrays::byte_array_to_vec(
                    env,
                    result.as_obj().into_inner(),
                )?)
            }),
            Err(ex) => self.with_obj(|env, _obj| {
                let ex: JThrowable = ex.as_obj().into();
                env.throw(ex)?;
                Err(Error::Other(Box::new(::jni::errors::Error::JavaException)))
            }),
        }
    }

    async fn subscribe(&self, characteristic: &Characteristic) -> Result<()> {
        self.set_characteristic_notification(characteristic, true)
            .await
    }

    async fn unsubscribe(&self, characteristic: &Characteristic) -> Result<()> {
        self.set_characteristic_notification(characteristic, false)
            .await
    }

    async fn notifications(&self) -> Result<Pin<Box<dyn Stream<Item = ValueNotification>>>> {
        use futures::stream::StreamExt;
        let stream = self.with_obj(|_env, obj| JavaStream::try_from(obj.get_notifications()?))?;
        let stream = stream
            .map(|item| match item {
                Ok(item) => {
                    let env = global_jvm().get_env()?;
                    let item = item.as_obj();
                    let characteristic = JBluetoothGattCharacteristic::from_env(&env, item)?;
                    let uuid = characteristic.get_uuid()?;
                    let value = characteristic.get_value()?;
                    Ok(ValueNotification { uuid, value })
                }
                Err(err) => Err(err),
            })
            .filter_map(|item| async { item.ok() });
        Ok(Box::pin(stream))
    }
}
