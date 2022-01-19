package ai.swim;

public class RustByteWriter {

    private final long inner;

    public RustByteWriter(long inner) {
        this.inner = inner;
    }

    public void writeAll(byte[] buffer) {
        RustByteWriter.__writeAll(this.inner, buffer);
    }

    private static native void __writeAll(long ptr, byte[] buffer);

    @Override
    public String toString() {
        return "RustByteWriter{" +
                "inner=" + inner +
                '}';
    }

}
