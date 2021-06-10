use jni::{
    errors::Result,
    objects::{AutoLocal, JMethodID, JObject},
    signature::JavaType,
    sys::jboolean,
    JNIEnv,
};

pub struct JBluetoothAdapter<'a: 'b, 'b> {
    internal: JObject<'a>,
    get_remote_device: JMethodID<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> ::std::ops::Deref for JBluetoothAdapter<'a, 'b> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a: 'b, 'b> From<JBluetoothAdapter<'a, 'b>> for JObject<'a> {
    fn from(other: JBluetoothAdapter<'a, 'b>) -> JObject<'a> {
        other.internal
    }
}

impl<'a: 'b, 'b> JBluetoothAdapter<'a, 'b> {
    pub fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> Result<JBluetoothAdapter<'a, 'b>> {
        let class = env.auto_local(env.find_class("android/bluetooth/BluetoothAdapter")?);
        Self::from_env_impl(env, class, obj)
    }

    pub fn get_remote_device(&self, address: JObject<'a>) -> Result<JObject<'a>> {
        self.env
            .call_method_unchecked(
                self.internal,
                self.get_remote_device,
                JavaType::Object("android/bluetooth/BluetoothDevice".into()),
                &[address.into()],
            )?
            .l()
    }

    pub fn get_default_adapter(env: &'b JNIEnv<'a>) -> Result<JBluetoothAdapter<'a, 'b>> {
        let class = env.auto_local(env.find_class("android/bluetooth/BluetoothAdapter")?);
        let adapter = env
            .call_static_method(&class, "getDefaultAdapter", "", &[])?
            .l()?;
        Self::from_env_impl(env, class, adapter)
    }

    fn from_env_impl(
        env: &'b JNIEnv<'a>,
        class: AutoLocal<'a, 'b>,
        obj: JObject<'a>,
    ) -> Result<JBluetoothAdapter<'a, 'b>> {
        let get_remote_device = env.get_method_id(&class, "getRemoteDevice", "[B")?;

        Ok(JBluetoothAdapter {
            internal: obj,
            get_remote_device,
            env,
        })
    }
}

pub struct JBluetoothDevice<'a: 'b, 'b> {
    internal: JObject<'a>,
    connect_gatt: JMethodID<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> JBluetoothDevice<'a, 'b> {
    pub fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> Result<JBluetoothDevice<'a, 'b>> {
        let class = env.auto_local(env.find_class("android/bluetooth/BluetoothAdapter")?);

        let connect_gatt = env.get_method_id(
            &class,
            "connectGatt",
            "Landroid/content/Context;ZLandroid/bluetooth/BluetoothGattCallback;",
        )?;

        Ok(JBluetoothDevice {
            internal: obj,
            connect_gatt,
            env,
        })
    }

    pub fn connect_gatt(
        &self,
        context: JObject<'a>,
        auto_connect: jboolean,
        callback: JObject<'a>,
    ) -> Result<JObject<'a>> {
        self.env
            .call_method_unchecked(
                self.internal,
                self.connect_gatt,
                JavaType::Object("android/bluetooth/BluetoothGatt".into()),
                &[context.into(), auto_connect.into(), callback.into()],
            )?
            .l()
    }
}
