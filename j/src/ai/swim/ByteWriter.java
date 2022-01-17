package ai.swim;

public class ByteWriter {

    private final long inner;

    public ByteWriter(int capacity) {
        this.inner = ByteWriter.__createWriter(capacity);
    }

    public void writeAll(byte[] buffer) {
        ByteWriter.__writeAll(this.inner, buffer);
    }

    public int len() {
        return ByteWriter.__len(this.inner);
    }

    private static native long __createWriter(int capacity);

    private static native int __len(long ptr);

    private static native void __writeAll(long ptr, byte[] buffer);

}
