import com.sun.jna._
import java.nio.charset.StandardCharsets
import java.nio.ByteBuffer

object BuildRustLibrary extends Library {
  // Load the Rust library. The name of the library file may differ depending on your platform.
  val _ = System.setProperty("jna.library.path", "./rspace++/target/release")

  // Define interface for Rust functions
  trait RustLib extends Library {
    def space_new(): Pointer
    def space_print(rspace: Pointer, channel: Pointer): Unit
    def space_clear(rspace: Pointer): Unit

    def space_get_once_durable_concurrent(
        rspace: Pointer,
        channel: Pointer,
        entry: Pointer
    ): Pointer
  }

  def main(args: Array[String]): Unit = {
    // Load the Rust library
    val lib = Native.load("rspace_plus_plus", classOf[RustLib]).asInstanceOf[RustLib]

    val spacePtr = lib.space_new()

    val channel = "friends"
    val entry   = "alice"

    val channelBytes  = channel.getBytes(StandardCharsets.UTF_8)
    val channelBuffer = ByteBuffer.allocateDirect(channelBytes.length)
    channelBuffer.put(channelBytes)

    val entryBytes  = entry.getBytes(StandardCharsets.UTF_8)
    val entryBuffer = ByteBuffer.allocateDirect(entryBytes.length)
    entryBuffer.put(entryBytes)

    val channelPtr: Pointer = Native.getDirectBufferPointer(channelBuffer)
    val entryPtr: Pointer   = Native.getDirectBufferPointer(entryBuffer)

    val res1 = lib.space_get_once_durable_concurrent(spacePtr, channelPtr, entryPtr)
    println("Result 1: ", res1)

    lib.space_print(spacePtr, channelPtr)

    lib.space_clear(spacePtr)
  }
}

// .dylib on mac os
// .so on linux
