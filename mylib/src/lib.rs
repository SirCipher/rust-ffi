#![allow(warnings)]

use jni::objects::JClass;
use jni::JNIEnv;
use tokio::runtime::Runtime;

mod buf;
mod utils;

#[no_mangle]
pub extern "system" fn Java_ai_swim_FfiTest_initRuntime(
    _env: JNIEnv,
    _class: JClass,
) -> *mut Runtime {
    Box::into_raw(Box::new(
        tokio::runtime::Builder::new_multi_thread()
            .build()
            .expect("Failed to build Tokio runtime"),
    ))
}
