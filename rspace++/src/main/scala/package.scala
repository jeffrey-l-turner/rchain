package rspacePlusPlus

import com.sun.jna._
import java.nio.charset.StandardCharsets
import java.nio.ByteBuffer

import firefly.rtypes.{Address, Entry, Name, Send}

object RustLibrary extends Library {
  val _ = System.setProperty("jna.library.path", "./rspace++/target/release")

  trait RustLib extends Library {
    trait Pattern[D] extends Function1[D, Boolean]

    // def process_strings(strings: Array[String], length: Int): Unit

    def space_new(): Pointer
    def space_print(rspace: Pointer, channel: String): Unit
    def space_clear(rspace: Pointer): Unit

    def space_get_once_durable_concurrent(
        rspace: Pointer,
        pdata: Array[Byte]
    ): Unit

    // def space_put_once_durable_concurrent(
    //     rspace: Pointer,
    //     channels: Array[String],
    //     patterns: List[Pattern[String]],
    //     continuation: String
    // ): Pointer
  }

  val lib = Native.load("rspace_plus_plus", classOf[RustLib]).asInstanceOf[RustLib]

  def run(): Unit = {
    val spacePtr = lib.space_new()

    val channel = "friends"
    // val entry   = "alice"
    // val continuation = "k-function"

    val alice_name = Name(first = "Alice", last = "Lincoln")
    val alice_address =
      Address(street = "777 Ford St", city = "Crystal Lake", state = "Idaho", zip = "223322")
    val alice = Entry(
      name = Some(alice_name),
      address = Some(alice_address),
      email = "alicel@ringworld.net",
      phone = "787-555-1212"
    )

    println(alice, alice_name, alice_address, "\n\n\n")

    val send1 = Send("friends", Some(alice), false).toByteArray;

    lib.space_get_once_durable_concurrent(spacePtr, send1)

    lib.space_print(spacePtr, channel)

    lib.space_clear(spacePtr)

    // val strings = Array("Hello", "from", "Scala!")
    // lib.process_strings(strings, strings.length)
  }

}
