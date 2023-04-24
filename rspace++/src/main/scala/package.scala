package rspacePlusPlus

import com.sun.jna._
import java.nio.charset.StandardCharsets
import java.nio.ByteBuffer

object RustLibrary extends Library {
  val _ = System.setProperty("jna.library.path", "./rspace++/target/release")

  trait RustLib extends Library {
    trait Pattern[D] extends Function1[D, Boolean]

    def space_new(): Pointer
    def space_print(rspace: Pointer, channel: Pointer): Unit
    def space_clear(rspace: Pointer): Unit

    def space_get_once_durable_concurrent(
        rspace: Pointer,
        channel: Pointer,
        entry: Pointer
    ): Pointer

    def space_put_once_durable_concurrent(
        rspace: Pointer,
        channels: Array[Pointer],
        patterns: List[Pattern[String]],
        continuation: Pointer
    ): Pointer
  }

  val lib = Native.load("rspace_plus_plus", classOf[RustLib]).asInstanceOf[RustLib]

  def run(): Unit = {
    val spacePtr = lib.space_new()

    val channel      = "friends"
    val entry        = "alice"
    val continuation = "k-function"

    val channelBytes  = channel.getBytes(StandardCharsets.UTF_8)
    val channelBuffer = ByteBuffer.allocateDirect(channelBytes.length)
    channelBuffer.put(channelBytes)

    val entryBytes  = entry.getBytes(StandardCharsets.UTF_8)
    val entryBuffer = ByteBuffer.allocateDirect(entryBytes.length)
    entryBuffer.put(entryBytes)

    val continuationBytes  = continuation.getBytes(StandardCharsets.UTF_8)
    val continuationBuffer = ByteBuffer.allocateDirect(continuationBytes.length)
    continuationBuffer.put(continuationBytes)

    val channelPtr: Pointer = Native.getDirectBufferPointer(channelBuffer)
    // val entryPtr: Pointer   = Native.getDirectBufferPointer(entryBuffer)

    // lib.space_get_once_durable_concurrent(spacePtr, channelPtr, entryPtr)

    lib.space_print(spacePtr, channelPtr)

    lib.space_clear(spacePtr)
  }

}
