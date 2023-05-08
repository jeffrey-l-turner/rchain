package rspacePlusPlus

import com.sun.jna._
import java.nio.charset.StandardCharsets
import java.nio.ByteBuffer

import firefly.rtypes.{Address, Entry, Name, Receive, Send}

object RustLibrary extends Library {
  val _ = System.setProperty("jna.library.path", "./rspace++/target/release/")

  val libraryPath = System.getProperty("jna.library.path")
  // println(s"JNA library path is: $libraryPath")

  trait RustLib extends Library {
    def space_new(): Pointer
    def is_empty(rspace: Pointer): Boolean
    def space_print(rspace: Pointer, channel: String): Unit
    def space_clear(rspace: Pointer): Unit

    def space_get_once_durable_concurrent(
        rspace: Pointer,
        pdata: Array[Byte],
        pdata_len: Int
    ): Pointer

    def space_put_once_durable_concurrent(
        rspace: Pointer,
        cdata: Array[Byte],
        cdata_len: Int
    ): Pointer
  }

  val lib = Native.load("rspace_plus_plus", classOf[RustLib]).asInstanceOf[RustLib]

  def run(): Unit = {
    val spacePtr = lib.space_new()

    val channel = "friends"

    // Consume
    val rec1     = Receive(Seq("friends"), Seq("Lincoln"), "I am the continuation, for now...", false);
    val rec1_buf = rec1.toByteArray;
    lib.space_put_once_durable_concurrent(spacePtr, rec1_buf, rec1_buf.length);

    // Produce
    val alice_name = Name(first = "Alice", last = "Lincoln")
    val alice_address =
      Address(street = "777 Ford St", city = "Crystal Lake", state = "Idaho", zip = "223322")
    val alice = Entry(
      name = Some(alice_name),
      address = Some(alice_address),
      email = "alicel@ringworld.net",
      phone = "787-555-1212"
    )

    val send1     = Send("friends", Some(alice), "Lincoln", false);
    val send1_buf = send1.toByteArray;
    val res1      = lib.space_get_once_durable_concurrent(spacePtr, send1_buf, send1_buf.length);
    println("\n", res1);

    lib.space_print(spacePtr, channel)
    lib.space_clear(spacePtr)
  }

}
