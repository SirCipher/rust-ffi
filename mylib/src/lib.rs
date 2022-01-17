#![allow(warnings)]

use jni::objects::JClass;
use jni::JNIEnv;
use std::cell::RefCell;
use tokio::runtime::{Handle, Runtime};

mod buf;
mod utils;

thread_local! {
    static RUNTIME: RefCell<Option<Handle>> = RefCell::new(None)
}

pub(crate) fn get_runtime() -> Result<Handle, ()> {
    match RUNTIME.try_with(|ctx| ctx.borrow().clone()) {
        Ok(Some(handle)) => Ok(handle),
        _ => panic!(),
    }
}

#[no_mangle]
pub extern "system" fn Java_ai_swim_FfiTest_initRuntime(
    _env: JNIEnv,
    _class: JClass,
) -> *mut Runtime {
    println!("Building tokio runtime");

    let builder = tokio::runtime::Builder::new_multi_thread()
        .build()
        .expect("Failed to build Tokio runtime");
    let handle = builder.handle().clone();

    RUNTIME.try_with(|cell| cell.borrow_mut().replace(handle));

    Box::into_raw(Box::new(builder))
}

#[no_mangle]
pub extern "system" fn Java_ai_swim_FfiTest_asyncTest(_env: JNIEnv, _class: JClass) {
    let handle = get_runtime().unwrap();
    handle.block_on(async move {
        println!("Hello from tokio");
    });
}
