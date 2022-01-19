package ai.swim;

public class RustByteChannel {

    public RustByteReader reader;
    public RustByteWriter writer;

    public RustByteChannel(RustByteReader reader, RustByteWriter writer) {
        this.reader = reader;
        this.writer = writer;
    }

    public static native RustByteChannel create();

    @Override
    public String toString() {
        return "RustByteChannel{" +
                "reader=" + reader +
                ", writer=" + writer +
                '}';
    }

}
