use jni::{
    errors::Result,
    objects::{JClass, JMethodID, JObject},
    signature::{JavaType, Primitive},
    sys::jint,
    JNIEnv,
};
use jni_utils::{future::JFuture, uuid::JUuid};
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
        Ok(Self {
            internal: obj,
            connect,
            disconnect,
            is_connected,
            discover_characteristics,
            read,
            write,
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
}

pub struct JBluetoothGattCharacteristic<'a: 'b, 'b> {
    internal: JObject<'a>,
    get_uuid: JMethodID<'a>,
    get_properties: JMethodID<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> JBluetoothGattCharacteristic<'a, 'b> {
    pub fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> Result<Self> {
        let class =
            env.auto_local(env.find_class("android/bluetooth/BluetoothGattCharacteristic")?);

        let get_uuid = env.get_method_id(&class, "getUuid", "()Ljava/util/UUID;")?;
        let get_properties = env.get_method_id(&class, "getProperties", "()I")?;
        Ok(Self {
            internal: obj,
            get_uuid,
            get_properties,
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
}
