package rspacePlusPlus

// import com.sun.jna._
import java.nio.charset.StandardCharsets
import java.nio.ByteBuffer

import firefly.rtypes.{Address, Entry, Name, Receive, Send}

object RustLibrary {
  // val _ = System.setProperty("jna.library.path", "./rspace++/target/release/")

  // val libraryPath = System.getProperty("jna.library.path")
  // println(s"JNA library path is: $libraryPath")

  // trait RustLib extends Library {
  //   def space_new(): Pointer
  //   def is_empty(rspace: Pointer): Boolean
  //   def space_print(rspace: Pointer, channel: String): Unit
  //   def space_clear(rspace: Pointer): Unit

  //   def space_get_once_durable_concurrent(
  //       rspace: Pointer,
  //       pdata: Array[Byte],
  //       pdata_len: Int
  //   ): ByteBuffer

  //   def space_put_once_durable_concurrent(
  //       rspace: Pointer,
  //       cdata: Array[Byte],
  //       cdata_len: Int
  //   ): Pointer
  // }

  /*
		val libraryDir = "/path/to/library/directory" // set the directory where the library is located
    System.setProperty("java.library.path", libraryDir) // set the java.library.path system property
    val fieldSysPath = Class.forName("java.lang.ClassLoader")
      .getDeclaredField("sys_paths")
    fieldSysPath.setAccessible(true)
    fieldSysPath.set(null, null)
    System.loadLibrary("mylib") // load the Rust library
    val rust = new RustLibrary() // create an instance of the RustLibrary class
    val input = "Hello, world!".getBytes(StandardCharsets.US_ASCII) // convert the input string to a byte array
    val result = rust.string_length(input) // call the Rust function and store the result
    println(result) // print the result
	*/

  class RustLib {
    // @native def my_function(): ByteBuffer

    // @native def space_new(): Pointer
    // @native def is_empty(rspace: Pointer): Boolean
    // @native def space_print(rspace: Pointer, channel: String): Unit
    // @native def space_clear(rspace: Pointer): Unit

    @native def space_get_once_durable_concurrent(
        // rspace: Pointer,
        pdata: Array[Byte],
        pdata_len: Int
    ): Name

    // @native def space_put_once_durable_concurrent(
    //     rspace: Pointer,
    //     cdata: Array[Byte],
    //     cdata_len: Int
    // ): Pointer
  }

  // val _ = System.setProperty("java.library.path", "~/src/f1refl3y/rspace++/target/release/")
  // System.loadLibrary("rspace_plus_plus")
  System.load("/Users/Jack/src/f1r3fly/rspace++/target/release/librspace_plus_plus.dylib")

  val lib = new RustLib()
  // val lib = Native.load("rspace_plus_plus", classOf[RustLib]).asInstanceOf[RustLib]

  def run(): Unit = {
    // val spacePtr = lib.space_new()

    // val channel = "friends"

    // Consume
    // val rec1     = Receive(Seq("friends"), Seq("Lincoln"), "I am the continuation, for now...", false);
    // val rec1_buf = rec1.toByteArray;
    // lib.space_put_once_durable_concurrent(spacePtr, rec1_buf, rec1_buf.length);

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
    val res1      = lib.space_get_once_durable_concurrent(send1_buf, send1_buf.length);
    println("\n", res1);

    // lib.space_print(spacePtr, channel)
    // lib.space_clear(spacePtr)
  }

}
