use jni::{
    errors::Result,
    objects::{JClass, JMethodID, JObject},
    signature::{JavaType, Primitive},
    sys::jint,
    JNIEnv,
};
use jni_utils::{future::JFuture, stream::JStream, uuid::JUuid};
use uuid::Uuid;

use crate::api::{BDAddr, CharPropFlags};

pub struct JPeripheral<'a: 'b, 'b> {
    internal: JObject<'a>,
    connect: JMethodID<'a>,
    disconnect: JMethodID<'a>,
    is_connected: JMethodID<'a>,
    discover_characteristics: JMethodID<'a>,
    read: JMethodID<'a>,
    write: JMethodID<'a>,
    set_characteristic_notification: JMethodID<'a>,
    get_notifications: JMethodID<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> ::std::ops::Deref for JPeripheral<'a, 'b> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a: 'b, 'b> From<JPeripheral<'a, 'b>> for JObject<'a> {
    fn from(other: JPeripheral<'a, 'b>) -> JObject<'a> {
        other.internal
    }
}

impl<'a: 'b, 'b> JPeripheral<'a, 'b> {
    pub fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> Result<Self> {
        let class = env.find_class("com/nonpolynomial/btleplug/android/impl/Peripheral")?;
        Self::from_env_impl(env, obj, class)
    }

    fn from_env_impl(env: &'b JNIEnv<'a>, obj: JObject<'a>, class: JClass<'a>) -> Result<Self> {
        let class = env.auto_local(class);

        let connect = env.get_method_id(&class, "connect", "()Lgedgygedgy/rust/future/Future;")?;
        let disconnect =
            env.get_method_id(&class, "disconnect", "()Lgedgygedgy/rust/future/Future;")?;
        let is_connected = env.get_method_id(&class, "isConnected", "()Z")?;
        let discover_characteristics = env.get_method_id(
            &class,
            "discoverCharacteristics",
            "()Lgedgygedgy/rust/future/Future;",
        )?;
        let read = env.get_method_id(
            &class,
            "read",
            "(Ljava/util/UUID;)Lgedgygedgy/rust/future/Future;",
        )?;
        let write = env.get_method_id(
            &class,
            "write",
            "(Ljava/util/UUID;[BI)Lgedgygedgy/rust/future/Future;",
        )?;
        let set_characteristic_notification = env.get_method_id(
            &class,
            "setCharacteristicNotification",
            "(Ljava/util/UUID;Z)Lgedgygedgy/rust/future/Future;",
        )?;
        let get_notifications = env.get_method_id(
            &class,
            "getNotifications",
            "()Lgedgygedgy/rust/stream/Stream;",
        )?;
        Ok(Self {
            internal: obj,
            connect,
            disconnect,
            is_connected,
            discover_characteristics,
            read,
            write,
            set_characteristic_notification,
            get_notifications,
            env,
        })
    }

    pub fn new(env: &'b JNIEnv<'a>, addr: BDAddr) -> Result<Self> {
        let class = env.find_class("com/nonpolynomial/btleplug/android/impl/Peripheral")?;
        let addr_jstr = env.new_string(format!("{:X}", addr))?;
        let obj = env.new_object(class, "(Ljava/lang/String;)V", &[addr_jstr.into()])?;
        Self::from_env_impl(env, obj, class)
    }

    pub fn connect(&self) -> Result<JFuture<'a, 'b>> {
        let future_obj = self
            .env
            .call_method_unchecked(
                self.internal,
                self.connect,
                JavaType::Object("Lgedgygedgy/rust/future/Future;".to_string()),
                &[],
            )?
            .l()?;
        JFuture::from_env(self.env, future_obj)
    }

    pub fn disconnect(&self) -> Result<JFuture<'a, 'b>> {
        let future_obj = self
            .env
            .call_method_unchecked(
                self.internal,
                self.disconnect,
                JavaType::Object("Lgedgygedgy/rust/future/Future;".to_string()),
                &[],
            )?
            .l()?;
        JFuture::from_env(self.env, future_obj)
    }

    pub fn is_connected(&self) -> Result<bool> {
        self.env
            .call_method_unchecked(
                self.internal,
                self.is_connected,
                JavaType::Primitive(Primitive::Boolean),
                &[],
            )?
            .z()
    }

    pub fn discover_characteristics(&self) -> Result<JFuture<'a, 'b>> {
        let future_obj = self
            .env
            .call_method_unchecked(
                self.internal,
                self.discover_characteristics,
                JavaType::Object("Lgedgygedgy/rust/future/Future;".to_string()),
                &[],
            )?
            .l()?;
        JFuture::from_env(self.env, future_obj)
    }

    pub fn read(&self, uuid: JUuid<'a, 'b>) -> Result<JFuture<'a, 'b>> {
        let future_obj = self
            .env
            .call_method_unchecked(
                self.internal,
                self.read,
                JavaType::Object("Lgedgygedgy/rust/future/Future;".to_string()),
                &[uuid.into()],
            )?
            .l()?;
        JFuture::from_env(self.env, future_obj)
    }

    pub fn write(
        &self,
        uuid: JUuid<'a, 'b>,
        data: JObject<'a>,
        write_type: jint,
    ) -> Result<JFuture<'a, 'b>> {
        let future_obj = self
            .env
            .call_method_unchecked(
                self.internal,
                self.write,
                JavaType::Object("Lgedgygedgy/rust/future/Future;".to_string()),
                &[uuid.into(), data.into(), write_type.into()],
            )?
            .l()?;
        JFuture::from_env(self.env, future_obj)
    }

    pub fn set_characteristic_notification(
        &self,
        uuid: JUuid<'a, 'b>,
        enable: bool,
    ) -> Result<JFuture<'a, 'b>> {
        let future_obj = self
            .env
            .call_method_unchecked(
                self.internal,
                self.set_characteristic_notification,
                JavaType::Object("Lgedgygedgy/rust/future/Future;".to_string()),
                &[uuid.into(), enable.into()],
            )?
            .l()?;
        JFuture::from_env(self.env, future_obj)
    }

    pub fn get_notifications(&self) -> Result<JStream<'a, 'b>> {
        let stream_obj = self
            .env
            .call_method_unchecked(
                self.internal,
                self.get_notifications,
                JavaType::Object("Lgedgygedgy/rust/stream/Stream;".to_string()),
                &[],
            )?
            .l()?;
        JStream::from_env(self.env, stream_obj)
    }
}

pub struct JBluetoothGattCharacteristic<'a: 'b, 'b> {
    internal: JObject<'a>,
    get_uuid: JMethodID<'a>,
    get_properties: JMethodID<'a>,
    get_value: JMethodID<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> JBluetoothGattCharacteristic<'a, 'b> {
    pub fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> Result<Self> {
        let class =
            env.auto_local(env.find_class("android/bluetooth/BluetoothGattCharacteristic")?);

        let get_uuid = env.get_method_id(&class, "getUuid", "()Ljava/util/UUID;")?;
        let get_properties = env.get_method_id(&class, "getProperties", "()I")?;
        let get_value = env.get_method_id(&class, "getValue", "()[B")?;
        Ok(Self {
            internal: obj,
            get_uuid,
            get_properties,
            get_value,
            env,
        })
    }

    pub fn get_uuid(&self) -> Result<Uuid> {
        let obj = self
            .env
            .call_method_unchecked(
                self.internal,
                self.get_uuid,
                JavaType::Object("Ljava/util/UUID;".to_string()),
                &[],
            )?
            .l()?;
        let uuid_obj = JUuid::from_env(self.env, obj)?;
        Ok(uuid_obj.as_uuid()?)
    }

    pub fn get_properties(&self) -> Result<CharPropFlags> {
        let flags = self
            .env
            .call_method_unchecked(
                self.internal,
                self.get_properties,
                JavaType::Primitive(Primitive::Int),
                &[],
            )?
            .i()?;
        Ok(CharPropFlags::from_bits_truncate(flags as u8))
    }

    pub fn get_value(&self) -> Result<Vec<u8>> {
        use std::iter::FromIterator;
        let value = self
            .env
            .call_method_unchecked(
                self.internal,
                self.get_value,
                JavaType::Array(JavaType::Primitive(Primitive::Byte).into()),
                &[],
            )?
            .l()?;

        let result = self
            .env
            .get_byte_array_elements(*value, jni::objects::ReleaseMode::NoCopyBack)?;
        let size = result.size()? as usize;
        let v = unsafe { Vec::from_raw_parts(result.as_ptr(), size, size) };
        Ok(Vec::from_iter(v.into_iter().map(|i| i as u8)))
    }
}
