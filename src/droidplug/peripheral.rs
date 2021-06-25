use crate::{
    api::{self, BDAddr, Characteristic, PeripheralProperties, ValueNotification, WriteType},
    Error, Result,
};
use async_trait::async_trait;
use futures::stream::Stream;
use jni::{
    objects::{GlobalRef, JThrowable},
    JNIEnv,
};
use jni_utils::future::JavaFuture;
use std::{
    collections::BTreeSet,
    convert::TryFrom,
    fmt::{Debug, Formatter},
    pin::Pin,
};

use super::jni::{global_jvm, objects::JPeripheral};

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
        let guard = global_jvm().attach_current_thread()?;
        let env = &*guard;
        let obj = JPeripheral::from_env(env, self.internal.as_obj())?;
        f(env, obj)
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
