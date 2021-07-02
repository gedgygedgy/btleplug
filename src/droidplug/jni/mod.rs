pub mod objects;

use super::adapter::Adapter;
use ::jni::{objects::JObject, JNIEnv, JavaVM, NativeMethod};
use once_cell::sync::OnceCell;
use std::ffi::c_void;

static GLOBAL_JVM: OnceCell<JavaVM> = OnceCell::new();

pub fn init(env: &JNIEnv) -> crate::Result<()> {
    if let Ok(()) = GLOBAL_JVM.set(env.get_java_vm()?) {
        env.register_native_methods(
            "com/nonpolynomial/btleplug/android/impl/Adapter",
            &[NativeMethod {
                name: "reportScanResult".into(),
                sig: "(Landroid/bluetooth/le/ScanResult;)V".into(),
                fn_ptr: adapter_report_scan_result as *mut c_void,
            }],
        )?;
    }
    Ok(())
}

pub fn global_jvm() -> &'static JavaVM {
    GLOBAL_JVM.get().expect(
        "Droidplug has not been initialized. Please initialize it with btleplug::platform::init().",
    )
}

impl From<::jni::errors::Error> for crate::Error {
    fn from(err: ::jni::errors::Error) -> Self {
        Self::Other(Box::new(err))
    }
}

fn adapter_report_scan_result_internal(
    env: &JNIEnv,
    obj: JObject,
    scan_result: JObject,
) -> crate::Result<()> {
    let adapter = env.get_rust_field::<_, _, Adapter>(obj, "handle")?;
    adapter.report_scan_result(scan_result)?;
    Ok(())
}

extern "C" fn adapter_report_scan_result(env: JNIEnv, obj: JObject, scan_result: JObject) {
    let _ = adapter_report_scan_result_internal(&env, obj, scan_result);
}
