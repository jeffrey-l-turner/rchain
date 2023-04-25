package rspacePlusPlus

import com.sun.jna._
import java.nio.charset.StandardCharsets
import java.nio.ByteBuffer

object RustLibrary extends Library {
  val _ = System.setProperty("jna.library.path", "./rspace++/target/release")

  trait RustLib extends Library {
    trait Pattern[D] extends Function1[D, Boolean]

    def process_strings(strings: Array[String], length: Int): Unit

    def space_new(): Pointer
    def space_print(rspace: Pointer, channel: String): Unit
    def space_clear(rspace: Pointer): Unit

    def space_get_once_durable_concurrent(
        rspace: Pointer,
        channel: String,
        entry: String
    ): Pointer

    def space_put_once_durable_concurrent(
        rspace: Pointer,
        channels: Array[String],
        patterns: List[Pattern[String]],
        continuation: String
    ): Pointer
  }

  val lib = Native.load("rspace_plus_plus", classOf[RustLib]).asInstanceOf[RustLib]

  def run(): Unit = {
    val spacePtr = lib.space_new()

    val channel = "friends"
    val entry   = "alice"
    // val continuation = "k-function"

    lib.space_get_once_durable_concurrent(spacePtr, channel, entry)

    lib.space_print(spacePtr, channel)

    lib.space_clear(spacePtr)

    val strings = Array("Hello", "from", "Scala!")
    lib.process_strings(strings, strings.length)
  }

}
