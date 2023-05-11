import org.scalatest.funsuite.AnyFunSuite
import rspacePlusPlus.{RustLibrary, Setup}
import firefly.rtypes.{Entry, Receive, Send}
import com.sun.jna._
import java.io.File

class MemSeqTest extends AnyFunSuite {
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

  // In-Memory Sequential
  test("MemSeqProduceMatch") {
    // Consume
    val receive =
      Receive(Seq("friends"), Seq(setup.cityPattern), "I am the continuation, for now...");
    val receive_buf = receive.toByteArray;
    lib.space_put_once_non_durable_sequential(spacePtr, receive_buf, receive_buf.length);

    // Produce
    val send     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send_buf = send.toByteArray;
    lib.space_get_once_non_durable_sequential(spacePtr, send_buf, send_buf.length);

    assert(lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }

  test("MemSeqProduceNoMatch") {
    // Consume
    val receive =
      Receive(Seq("friends"), Seq(setup.cityPattern), "I am the continuation, for now...");
    val receive_buf = receive.toByteArray;
    lib.space_put_once_non_durable_sequential(spacePtr, receive_buf, receive_buf.length);

    // Produce
    val send     = Send("friends", Some(setup.carol), cityMatchCase(setup.carol));
    val send_buf = send.toByteArray;
    lib.space_get_once_non_durable_sequential(spacePtr, send_buf, send_buf.length);

    assert(!lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }

  test("MemSeqConsumeMatch") {
    // Produce
    val send     = Send("friends", Some(setup.bob), nameMatchCase(setup.bob));
    val send_buf = send.toByteArray;
    lib.space_get_once_non_durable_sequential(spacePtr, send_buf, send_buf.length);

    // Consume
    val receive =
      Receive(Seq("friends"), Seq(setup.namePattern), "I am the continuation, for now...");
    val receive_buf = receive.toByteArray;
    lib.space_put_once_non_durable_sequential(spacePtr, receive_buf, receive_buf.length);

    assert(lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }

  test("MemSeqMultipleChannelsConsumeMatch") {
    // Produce
    val send1     = Send("colleagues", Some(setup.dan), stateMatchCase(setup.dan));
    val send1_buf = send1.toByteArray;
    lib.space_get_once_non_durable_sequential(spacePtr, send1_buf, send1_buf.length);

    // Produce
    val send2     = Send("friends", Some(setup.erin), stateMatchCase(setup.erin));
    val send2_buf = send2.toByteArray;
    lib.space_get_once_non_durable_sequential(spacePtr, send2_buf, send2_buf.length);

    // Consume
    val receive =
      Receive(
        Seq("friends", "colleagues"),
        Seq(setup.statePattern, setup.statePattern),
        "I am the continuation, for now..."
      );
    val receive_buf = receive.toByteArray;
    lib.space_put_once_non_durable_sequential(spacePtr, receive_buf, receive_buf.length);

    assert(lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }

  test("MemSeqConsumePersist") {
    // Consume
    val receive =
      Receive(
        Seq("friends"),
        Seq(setup.cityPattern),
        "I am the continuation, for now..."
      );
    val receive_buf = receive.toByteArray;
    lib.space_put_always_non_durable_sequential(spacePtr, receive_buf, receive_buf.length);

    assert(!lib.is_empty(spacePtr));

    // Produce
    val send1     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send1_buf = send1.toByteArray;
    lib.space_get_once_non_durable_sequential(spacePtr, send1_buf, send1_buf.length);

    assert(!lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }

  test("MemSeqConsumePersistExistingMatches") {
    // Produce
    val send1     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send1_buf = send1.toByteArray;
    lib.space_get_once_non_durable_sequential(spacePtr, send1_buf, send1_buf.length);

    // Produce
    val send2     = Send("friends", Some(setup.bob), cityMatchCase(setup.alice));
    val send2_buf = send2.toByteArray;
    lib.space_get_once_non_durable_sequential(spacePtr, send2_buf, send2_buf.length);

    // Consume
    val receive1 =
      Receive(
        Seq("friends"),
        Seq(setup.cityPattern),
        "I am the continuation, for now..."
      );
    val receive1_buf = receive1.toByteArray;
    lib.space_put_always_non_durable_sequential(spacePtr, receive1_buf, receive1_buf.length);

    assert(!lib.is_empty(spacePtr));

    val receive2 =
      Receive(
        Seq("friends"),
        Seq(setup.cityPattern),
        "I am the continuation, for now..."
      );
    val receive2_buf = receive2.toByteArray;
    lib.space_put_always_non_durable_sequential(spacePtr, receive2_buf, receive2_buf.length);

    assert(lib.is_empty(spacePtr));

    val receive3 =
      Receive(
        Seq("friends"),
        Seq(setup.cityPattern),
        "I am the continuation, for now..."
      );
    val receive3_buf = receive3.toByteArray;
    lib.space_put_always_non_durable_sequential(spacePtr, receive3_buf, receive3_buf.length);

    assert(!lib.is_empty(spacePtr));

    // Produce
    val send3     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send3_buf = send3.toByteArray;
    lib.space_get_once_non_durable_sequential(spacePtr, send3_buf, send3_buf.length);

    assert(!lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }

  test("MemSeqProducePersist") {
    // Produce
    val send     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send_buf = send.toByteArray;
    lib.space_get_always_non_durable_sequential(spacePtr, send_buf, send_buf.length);

    assert(!lib.is_empty(spacePtr));

    // Consume
    val receive =
      Receive(
        Seq("friends"),
        Seq(setup.cityPattern),
        "I am the continuation, for now..."
      );
    val receive_buf = receive.toByteArray;
    lib.space_put_once_non_durable_sequential(spacePtr, receive_buf, receive_buf.length);

    assert(!lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }

  test("MemSeqProducePersistExistingMatches") {
    // Consume
    val receive1 =
      Receive(
        Seq("friends"),
        Seq(setup.cityPattern),
        "I am the continuation, for now..."
      );
    val receive1_buf = receive1.toByteArray;
    lib.space_put_once_non_durable_sequential(spacePtr, receive1_buf, receive1_buf.length);

    assert(!lib.is_empty(spacePtr));

    // Produce
    val send1     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send1_buf = send1.toByteArray;
    lib.space_get_always_non_durable_sequential(spacePtr, send1_buf, send1_buf.length);

    assert(lib.is_empty(spacePtr));

    // Produce
    val send2     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send2_buf = send2.toByteArray;
    lib.space_get_always_non_durable_sequential(spacePtr, send2_buf, send2_buf.length);

    // Consume
    val receive2 =
      Receive(
        Seq("friends"),
        Seq(setup.cityPattern),
        "I am the continuation, for now..."
      );
    val receive2_buf = receive2.toByteArray;
    lib.space_put_once_non_durable_sequential(spacePtr, receive2_buf, receive2_buf.length);

    assert(!lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }
}
