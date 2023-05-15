package rspacePlusPlus

import com.sun.jna._
import scala.beans.BeanProperty
import java.nio.charset.StandardCharsets
import java.nio.ByteBuffer

import firefly.rtypes.{Address, Entry, Name, OptionResult, Receive, Send}

// Problem here is JNA can't find decalared fields "x" and "y" bc uses reflection
// Adding "MyStruct" as return type to any of 16 functions will throw error
@SuppressWarnings(Array("org.wartremover.warts.Var"))
@Structure.FieldOrder(Array("x", "y"))
class MyStruct extends Structure {
  @scala.native
  val x: Int = 0
  @scala.native
  val y: Int = 0
}

object RustLibrary extends Library {
  val _ = System.setProperty("jna.library.path", "./rspace++/target/release/")

  val libraryPath = System.getProperty("jna.library.path")
  // println(s"JNA library path is: $libraryPath")

  trait RustLib extends Library {
    def space_new(): Pointer
    def is_empty(rspace: Pointer): Boolean
    def space_print(rspace: Pointer, channel: String): Unit
    def space_clear(rspace: Pointer): Unit

    // Verb Set 1
    def space_get_once_durable_concurrent(
        rspace: Pointer,
        pdata: Array[Byte],
        pdata_len: Int
    ): String

    def space_get_once_non_durable_concurrent(
        rspace: Pointer,
        pdata: Array[Byte],
        pdata_len: Int
    ): String

    def space_get_once_durable_sequential(
        rspace: Pointer,
        pdata: Array[Byte],
        pdata_len: Int
    ): String

    def space_get_once_non_durable_sequential(
        rspace: Pointer,
        pdata: Array[Byte],
        pdata_len: Int
    ): String

    // Verb Set 2
    def space_get_always_durable_concurrent(
        rspace: Pointer,
        pdata: Array[Byte],
        pdata_len: Int
    ): String

    def space_get_always_non_durable_concurrent(
        rspace: Pointer,
        pdata: Array[Byte],
        pdata_len: Int
    ): String

    def space_get_always_durable_sequential(
        rspace: Pointer,
        pdata: Array[Byte],
        pdata_len: Int
    ): String

    def space_get_always_non_durable_sequential(
        rspace: Pointer,
        pdata: Array[Byte],
        pdata_len: Int
    ): String

    // Verb Set 3
    def space_put_once_durable_concurrent(
        rspace: Pointer,
        cdata: Array[Byte],
        cdata_len: Int
    ): Array[String]

    def space_put_once_non_durable_concurrent(
        rspace: Pointer,
        cdata: Array[Byte],
        cdata_len: Int
    ): Array[String]

    def space_put_once_durable_sequential(
        rspace: Pointer,
        cdata: Array[Byte],
        cdata_len: Int
    ): Array[String]

    def space_put_once_non_durable_sequential(
        rspace: Pointer,
        cdata: Array[Byte],
        cdata_len: Int
    ): Array[String]

    // Verb Set 4
    def space_put_always_durable_concurrent(
        rspace: Pointer,
        cdata: Array[Byte],
        cdata_len: Int
    ): Array[String]

    def space_put_always_non_durable_concurrent(
        rspace: Pointer,
        cdata: Array[Byte],
        cdata_len: Int
    ): Array[String]

    def space_put_always_durable_sequential(
        rspace: Pointer,
        cdata: Array[Byte],
        cdata_len: Int
    ): Array[String]

    def space_put_always_non_durable_sequential(
        rspace: Pointer,
        cdata: Array[Byte],
        cdata_len: Int
    ): Array[String]
  }

  val lib = Native.load("rspace_plus_plus", classOf[RustLib]).asInstanceOf[RustLib]

  def cityMatchCase(entry: Entry): String =
    entry.address.get.city

  def nameMatchCase(entry: Entry): String =
    entry.name.get.last

  def stateMatchCase(entry: Entry): String =
    entry.address.get.state

  def run(): Unit = {
    val spacePtr = lib.space_new()
    val setup    = Setup.apply();

    val channel = "friends"

    // Consume
    val rec1     = Receive(Seq("friends"), Seq(setup.cityPattern), "I am the continuation, for now...");
    val rec1_buf = rec1.toByteArray;
    val cres     = lib.space_put_once_durable_concurrent(spacePtr, rec1_buf, rec1_buf.length);
    println(cres)

    // Produce
    val send1     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send1_buf = send1.toByteArray;
    val pres      = lib.space_get_once_durable_concurrent(spacePtr, send1_buf, send1_buf.length);
    println(pres);

    lib.space_print(spacePtr, channel)
    lib.space_clear(spacePtr)
  }
}
