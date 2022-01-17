package ai.swim;

import java.nio.ByteBuffer;

public class ByteWriter {
    private final long inner;

    public ByteWriter(int len) {
        this.inner = ByteWriter.__createWriter(len);
    }

    public void writeAll(ByteBuffer buffer) {
        ByteWriter.__writeAll(this.inner, buffer);
    }

    public int len() {
        return ByteWriter.__len(this.inner);
    }

    private static native long __createWriter(int capacity);

    private static native int __len(long ptr);

    private static native void __writeAll(long ptr, ByteBuffer buffer);
}
