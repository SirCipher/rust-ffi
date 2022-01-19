#![allow(warnings)]

use crate::utils::byte_channel::byte_channel;
use jni::objects::{JClass, JObject, JValue};
use jni::sys::{jlong, jobject};
use jni::JNIEnv;
use std::cell::RefCell;
use std::num::NonZeroUsize;
use std::ops::Deref;
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

fn into_handle<T: 'static>(t: T) -> jlong {
    Box::into_raw(Box::new(t)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_ai_swim_RustByteChannel_create<'a>(
    env: JNIEnv<'a>,
    _class: JClass,
) -> JObject<'a> {
    let (tx, rx) = byte_channel(NonZeroUsize::new(8).unwrap());
    let tx = into_handle(tx);
    let rx = into_handle(rx);

    let reader = env
        .new_object("ai/swim/RustByteReader", "(J)V", &[rx.into()])
        .unwrap();
    let writer = env
        .new_object("ai/swim/RustByteReader", "(J)V", &[tx.into()])
        .unwrap();

    let r = env.new_object(
        "ai/swim/RustByteChannel",
        "(Lai/swim/RustByteReader;Lai/swim/RustByteWriter;)V",
        &[reader.into(), writer.into()],
    );
    match r {
        Ok(obj) => obj,
        Err(e) => {
            eprintln!("{:?}", e);
            env.exception_describe();
            panic!()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_ai_swim_ByteWriter__1_1len(
    _env: JNIEnv,
    _class: JClass,
    ptr: *mut ByteWriter,
) -> jint {
    unsafe { (&*ptr).len().into() }
}
