package ai.swim;

public class RustByteReader {

    private final long inner;

    public RustByteReader(long inner) {
        this.inner = inner;
    }

    public void readAll(byte[] buffer) {
        RustByteReader.readAll(this.inner, buffer);
    }

    private static native byte[] readAll(long ptr, byte[] buffer);

    @Override
    public String toString() {
        return "RustByteReader{" +
                "inner=" + inner +
                '}';
    }

}
