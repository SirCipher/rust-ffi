use bytes::BytesMut;
use jni::objects::JClass;
use jni::sys::{jbyteArray, jint};
use jni::JNIEnv;

pub struct ByteWriter {
    buf: BytesMut,
}

impl ByteWriter {
    pub fn new(len: i32) -> ByteWriter {
        ByteWriter {
            buf: BytesMut::with_capacity(len as usize),
        }
    }

    pub fn len(&self) -> i32 {
        self.buf.len() as i32
    }

    pub fn write_all(&mut self, buf: &[u8]) {
        self.buf.extend_from_slice(buf);
    }
}

#[no_mangle]
pub extern "system" fn Java_ai_swim_ByteWriter__1_1createWriter(
    _env: JNIEnv,
    _class: JClass,
    len: jint,
) -> *const ByteWriter {
    Box::into_raw(Box::new(ByteWriter::new(len.into())))
}

#[no_mangle]
pub extern "system" fn Java_ai_swim_ByteWriter__1_1len(
    _env: JNIEnv,
    _class: JClass,
    ptr: *mut ByteWriter,
) -> jint {
    unsafe { (&*ptr).len().into() }
}

#[no_mangle]
pub extern "system" fn Java_ai_swim_ByteWriter__1_1writeAll(
    env: JNIEnv,
    _class: JClass,
    ptr: *mut ByteWriter,
    buf: jbyteArray,
) {
    let bytes = env
        .convert_byte_array(buf)
        .expect("ByteWriter provided with a null pointer");
    let writer = unsafe { &mut *ptr };

    writer.write_all(bytes.as_ref());
}
