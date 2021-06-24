use jni::{
    errors::Result,
    objects::{JClass, JMethodID, JObject},
    signature::{JavaType, Primitive},
    JNIEnv,
};
use jni_utils::future::JFuture;

use crate::api::BDAddr;

pub struct JPeripheral<'a: 'b, 'b> {
    internal: JObject<'a>,
    connect: JMethodID<'a>,
    disconnect: JMethodID<'a>,
    is_connected: JMethodID<'a>,
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
        Ok(Self {
            internal: obj,
            connect,
            disconnect,
            is_connected,
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
}
