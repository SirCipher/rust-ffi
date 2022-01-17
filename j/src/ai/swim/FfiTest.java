package ai.swim;

import java.nio.ByteBuffer;

class FfiTest {
    static {
        System.loadLibrary("ffiproto");
    }

    public static void main(String[] args) {
        System.out.println("Creating ByteWriter");

        ByteWriter byteWriter = new ByteWriter(8);
        System.out.println("ByteWriter len: " + byteWriter.len());

        byteWriter.writeAll(new byte []{0,1,2});

        System.out.println("ByteWriter len: " + byteWriter.len());
    }
}
