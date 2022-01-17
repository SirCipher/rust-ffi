package ai.swim;

class FfiTest {
    static {
        System.loadLibrary("ffiproto");
    }

    public static void main(String[] args) {
        System.out.println("Creating ByteWriter");

        ByteWriter byteWriter = new ByteWriter(8);
        System.out.println("ByteWriter len: " + byteWriter.len());
    }
}