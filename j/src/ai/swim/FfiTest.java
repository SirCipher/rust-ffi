package ai.swim;


class FfiTest {

    private static native long initRuntime();
    private static native long asyncTest();

    private static  final long TOKIO_HANDLE;

    static {
        System.loadLibrary("ffiproto");
        TOKIO_HANDLE = initRuntime();
    }

    public static void main(String[] args) {
        asyncTest();

        System.out.println("Creating ByteWriter");

        ByteWriter byteWriter = new ByteWriter(8);
        System.out.println("ByteWriter len: " + byteWriter.len());

        byteWriter.writeAll(new byte []{0,1,2});

        System.out.println("ByteWriter len: " + byteWriter.len());
    }
}
