/* DO NOT EDIT THIS FILE - it is machine generated */
#include <jni.h>
/* Header for class ai_swim_ByteWriter */

#ifndef _Included_ai_swim_ByteWriter
#define _Included_ai_swim_ByteWriter
#ifdef __cplusplus
extern "C" {
#endif
/*
 * Class:     ai_swim_ByteWriter
 * Method:    __createWriter
 * Signature: (I)J
 */
JNIEXPORT jlong JNICALL Java_ai_swim_ByteWriter__1_1createWriter
  (JNIEnv *, jclass, jint);

/*
 * Class:     ai_swim_ByteWriter
 * Method:    __len
 * Signature: (J)I
 */
JNIEXPORT jint JNICALL Java_ai_swim_ByteWriter__1_1len
  (JNIEnv *, jclass, jlong);

/*
 * Class:     ai_swim_ByteWriter
 * Method:    __writeAll
 * Signature: (JLjava/nio/ByteBuffer;)V
 */
JNIEXPORT void JNICALL Java_ai_swim_ByteWriter__1_1writeAll
  (JNIEnv *, jclass, jlong, jobject);

#ifdef __cplusplus
}
#endif
#endif