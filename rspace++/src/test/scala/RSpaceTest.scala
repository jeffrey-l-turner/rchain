// package rspacePlusPlus

import org.scalatest.funsuite.AnyFunSuite
import rspacePlusPlus.RustLibrary
import firefly.rtypes.{Address, Entry, Name, Receive, Send}
import com.sun.jna._
import java.io.File

case class Setup(
    cityPattern: String,
    namePattern: String,
    statePattern: String,
    alice: Entry,
    bob: Entry,
    carol: Entry,
    dan: Entry,
    erin: Entry
)

object Setup {
  def apply(): Setup = {
    // Alice
    val aliceName    = Name("Alice", "Lincoln")
    val aliceAddress = Address("777 Ford St", "Crystal Lake", "Idaho", "223322")
    val alice        = Entry(Some(aliceName), Some(aliceAddress), "alicel@ringworld.net", "787-555-1212")

    // Bob
    val bobName    = Name("Bob", "Lahblah")
    val bobAddress = Address("1000 Main St", "Crystal Lake", "Idaho", "223322")
    val bob        = Entry(Some(bobName), Some(bobAddress), "blablah@tenex.net", "698-555-1212")

    // Carol
    val carolName    = Name("Carol", "Lahblah")
    val carolAddress = Address("22 Goldwater Way", "Herbert", "Nevada", "334433")
    val carol        = Entry(Some(carolName), Some(carolAddress), "carol@blablah.org", "232-555-1212")

    // Dan
    val danName    = Name("Dan", "Walters")
    val danAddress = Address("40 Shady Lane", "Crystal Lake", "Idaho", "223322")
    val dan        = Entry(Some(danName), Some(danAddress), "deejwalters@sdf.lonestar.org", "444-555-1212")

    // Erin
    val erinName    = Name("Erin", "Rush")
    val erinAddress = Address("23 Market St.", "Peony", "Idaho", "224422")
    val erin        = Entry(Some(erinName), Some(erinAddress), "erush@lasttraintogoa.net", "333-555-1212")

    Setup(
      "Crystal Lake",
      "Lahblah",
      "Idaho",
      alice,
      bob,
      carol,
      dan,
      erin
    )
  }
}

class RSpaceTest extends AnyFunSuite {
  val _ = System.setProperty("jna.library.path", "./target/release/")
  val lib =
    Native.load("rspace_plus_plus", classOf[RustLibrary.RustLib]).asInstanceOf[RustLibrary.RustLib]

  def cityMatchCase(entry: Entry): String =
    entry.address.get.city

  def nameMatchCase(entry: Entry): String =
    entry.name.get.last

  def stateMatchCase(entry: Entry): String =
    entry.address.get.state

  val spacePtr = lib.space_new();
  val setup    = Setup.apply();

  // On-Disk Concurrent
  test("DiskConcProduceMatch") {
    // Consume
    val receive =
      Receive(Seq("friends"), Seq(setup.cityPattern), "I am the continuation, for now...");
    val receive_buf = receive.toByteArray;
    lib.space_put_once_durable_concurrent(spacePtr, receive_buf, receive_buf.length);

    // Produce
    val send     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send_buf = send.toByteArray;
    lib.space_get_once_durable_concurrent(spacePtr, send_buf, send_buf.length);

    assert(lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }

  test("DiskConcProduceNoMatch") {
    // Consume
    val receive =
      Receive(Seq("friends"), Seq(setup.cityPattern), "I am the continuation, for now...");
    val receive_buf = receive.toByteArray;
    lib.space_put_once_durable_concurrent(spacePtr, receive_buf, receive_buf.length);

    // Produce
    val send     = Send("friends", Some(setup.carol), cityMatchCase(setup.carol));
    val send_buf = send.toByteArray;
    lib.space_get_once_durable_concurrent(spacePtr, send_buf, send_buf.length);

    assert(!lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }

  test("DiskConcConsumeMatch") {
    // Produce
    val send     = Send("friends", Some(setup.bob), nameMatchCase(setup.bob));
    val send_buf = send.toByteArray;
    lib.space_get_once_durable_concurrent(spacePtr, send_buf, send_buf.length);

    // Consume
    val receive =
      Receive(Seq("friends"), Seq(setup.namePattern), "I am the continuation, for now...");
    val receive_buf = receive.toByteArray;
    lib.space_put_once_durable_concurrent(spacePtr, receive_buf, receive_buf.length);

    assert(lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }

  test("DiskConcMultipleChannelsConsumeMatch") {
    // Produce
    val send1     = Send("colleagues", Some(setup.dan), stateMatchCase(setup.dan));
    val send1_buf = send1.toByteArray;
    lib.space_get_once_durable_concurrent(spacePtr, send1_buf, send1_buf.length);

    // Produce
    val send2     = Send("friends", Some(setup.erin), stateMatchCase(setup.erin));
    val send2_buf = send2.toByteArray;
    lib.space_get_once_durable_concurrent(spacePtr, send2_buf, send2_buf.length);

    // Consume
    val receive =
      Receive(
        Seq("friends", "colleagues"),
        Seq(setup.statePattern, setup.statePattern),
        "I am the continuation, for now..."
      );
    val receive_buf = receive.toByteArray;
    lib.space_put_once_durable_concurrent(spacePtr, receive_buf, receive_buf.length);

    assert(lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }

  test("DiskConcConsumePersist") {
    // Consume
    val receive =
      Receive(
        Seq("friends"),
        Seq(setup.cityPattern),
        "I am the continuation, for now..."
      );
    val receive_buf = receive.toByteArray;
    lib.space_put_always_durable_concurrent(spacePtr, receive_buf, receive_buf.length);

    assert(!lib.is_empty(spacePtr));

    // Produce
    val send1     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send1_buf = send1.toByteArray;
    lib.space_get_once_durable_concurrent(spacePtr, send1_buf, send1_buf.length);

    assert(!lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }

  test("DiskConcConsumePersistExistingMatches") {
    // Produce
    val send1     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send1_buf = send1.toByteArray;
    lib.space_get_once_durable_concurrent(spacePtr, send1_buf, send1_buf.length);

    // Produce
    val send2     = Send("friends", Some(setup.bob), cityMatchCase(setup.alice));
    val send2_buf = send2.toByteArray;
    lib.space_get_once_durable_concurrent(spacePtr, send2_buf, send2_buf.length);

    // Consume
    val receive1 =
      Receive(
        Seq("friends"),
        Seq(setup.cityPattern),
        "I am the continuation, for now..."
      );
    val receive1_buf = receive1.toByteArray;
    lib.space_put_always_durable_concurrent(spacePtr, receive1_buf, receive1_buf.length);

    assert(!lib.is_empty(spacePtr));

    val receive2 =
      Receive(
        Seq("friends"),
        Seq(setup.cityPattern),
        "I am the continuation, for now..."
      );
    val receive2_buf = receive2.toByteArray;
    lib.space_put_always_durable_concurrent(spacePtr, receive2_buf, receive2_buf.length);

    assert(lib.is_empty(spacePtr));

    val receive3 =
      Receive(
        Seq("friends"),
        Seq(setup.cityPattern),
        "I am the continuation, for now..."
      );
    val receive3_buf = receive3.toByteArray;
    lib.space_put_always_durable_concurrent(spacePtr, receive3_buf, receive3_buf.length);

    assert(!lib.is_empty(spacePtr));

    // Produce
    val send3     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send3_buf = send3.toByteArray;
    lib.space_get_once_durable_concurrent(spacePtr, send3_buf, send3_buf.length);

    assert(!lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }

  test("DiskConcProducePersist") {
    // Produce
    val send     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send_buf = send.toByteArray;
    lib.space_get_always_durable_concurrent(spacePtr, send_buf, send_buf.length);

    assert(!lib.is_empty(spacePtr));

    // Consume
    val receive =
      Receive(
        Seq("friends"),
        Seq(setup.cityPattern),
        "I am the continuation, for now..."
      );
    val receive_buf = receive.toByteArray;
    lib.space_put_once_durable_concurrent(spacePtr, receive_buf, receive_buf.length);

    assert(!lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }

  test("DiskConcProducePersistExistingMatches") {
    // Consume
    val receive1 =
      Receive(
        Seq("friends"),
        Seq(setup.cityPattern),
        "I am the continuation, for now..."
      );
    val receive1_buf = receive1.toByteArray;
    lib.space_put_once_durable_concurrent(spacePtr, receive1_buf, receive1_buf.length);

    assert(!lib.is_empty(spacePtr));

    // Produce
    val send1     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send1_buf = send1.toByteArray;
    lib.space_get_always_durable_concurrent(spacePtr, send1_buf, send1_buf.length);

    assert(lib.is_empty(spacePtr));

    // Produce
    val send2     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send2_buf = send2.toByteArray;
    lib.space_get_always_durable_concurrent(spacePtr, send2_buf, send2_buf.length);

    // Consume
    val receive2 =
      Receive(
        Seq("friends"),
        Seq(setup.cityPattern),
        "I am the continuation, for now..."
      );
    val receive2_buf = receive2.toByteArray;
    lib.space_put_once_durable_concurrent(spacePtr, receive2_buf, receive2_buf.length);

    assert(!lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }
}
