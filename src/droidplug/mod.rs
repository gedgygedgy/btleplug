pub mod adapter;
pub mod manager;
pub mod peripheral;

use jni::JavaVM;
use once_cell::sync::OnceCell;

static GLOBAL_JVM: OnceCell<JavaVM> = OnceCell::new();

pub fn init(jvm: JavaVM) {
    let _ = GLOBAL_JVM.set(jvm);
}

fn global_jvm() -> &'static JavaVM {
    GLOBAL_JVM.get().expect(
        "Droidplug has not been initialized. Please initialize it with btleplug::platform::init().",
    )
}
