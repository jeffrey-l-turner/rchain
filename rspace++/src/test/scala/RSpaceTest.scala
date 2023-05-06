// package rspacePlusPlus

import org.scalatest.funsuite.AnyFunSuite
import rspacePlusPlus.RustLibrary
import firefly.rtypes.{Address, Entry, Name, Receive, Send}
import com.sun.jna._
import java.io.File

class RSpaceTest extends AnyFunSuite {

  val _ = System.setProperty("jna.library.path", "./target/release/")

  val lib =
    Native.load("rspace_plus_plus", classOf[RustLibrary.RustLib]).asInstanceOf[RustLibrary.RustLib]

  val spacePtr = lib.space_new();

  test("produce match") {
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
    val pres1     = lib.space_get_once_durable_concurrent(spacePtr, send1_buf, send1_buf.length);
    println(pres1);

    assert(lib.is_empty(spacePtr));
  }
}
