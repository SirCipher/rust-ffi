use jni::objects::JClass;
use jni::sys::jint;
use jni::JNIEnv;

pub struct ByteWriter {
    len: i32,
}

impl ByteWriter {
    pub fn new(len: i32) -> ByteWriter {
        ByteWriter { len }
    }

    pub fn len(&self) -> i32 {
        self.len
    }

    pub fn write_all(&mut self, buf: &[u8]) {}
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
