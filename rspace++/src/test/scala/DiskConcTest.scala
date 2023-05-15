import org.scalatest.funsuite.AnyFunSuite
import rspacePlusPlus.{RustLibrary, Setup}
import firefly.rtypes.{Address, Entry, Name, Receive, Send}
import com.sun.jna._
import java.io.File

class DiskConcTest extends AnyFunSuite {
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
    val cres        = lib.space_put_once_durable_concurrent(spacePtr, receive_buf, receive_buf.length);

    // Produce
    val send     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send_buf = send.toByteArray;
    val pres     = lib.space_get_once_durable_concurrent(spacePtr, send_buf, send_buf.length);

    assert(cres == null)
    assert(!pres.isEmpty())
    assert(lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }

  test("DiskConcProduceNoMatch") {
    // Consume
    val receive =
      Receive(Seq("friends"), Seq(setup.cityPattern), "I am the continuation, for now...");
    val receive_buf = receive.toByteArray;
    val cres        = lib.space_put_once_durable_concurrent(spacePtr, receive_buf, receive_buf.length);

    // Produce
    val send     = Send("friends", Some(setup.carol), cityMatchCase(setup.carol));
    val send_buf = send.toByteArray;
    val pres     = lib.space_get_once_durable_concurrent(spacePtr, send_buf, send_buf.length);

    assert(cres == null)
    assert(pres == null)
    assert(!lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }

  test("DiskConcConsumeMatch") {
    // Produce
    val send     = Send("friends", Some(setup.bob), nameMatchCase(setup.bob));
    val send_buf = send.toByteArray;
    val pres     = lib.space_get_once_durable_concurrent(spacePtr, send_buf, send_buf.length);

    // Consume
    val receive =
      Receive(Seq("friends"), Seq(setup.namePattern), "I am the continuation, for now...");
    val receive_buf = receive.toByteArray;
    val cres        = lib.space_put_once_durable_concurrent(spacePtr, receive_buf, receive_buf.length);

    assert(pres == null)
    assert(!cres.isEmpty)
    assert(lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }

  test("DiskConcMultipleChannelsConsumeMatch") {
    // Produce
    val send1     = Send("colleagues", Some(setup.dan), stateMatchCase(setup.dan));
    val send1_buf = send1.toByteArray;
    val pres1     = lib.space_get_once_durable_concurrent(spacePtr, send1_buf, send1_buf.length);

    // Produce
    val send2     = Send("friends", Some(setup.erin), stateMatchCase(setup.erin));
    val send2_buf = send2.toByteArray;
    val pres2     = lib.space_get_once_durable_concurrent(spacePtr, send2_buf, send2_buf.length);

    // Consume
    val receive =
      Receive(
        Seq("friends", "colleagues"),
        Seq(setup.statePattern, setup.statePattern),
        "I am the continuation, for now..."
      );
    val receive_buf = receive.toByteArray;
    val cres        = lib.space_put_once_durable_concurrent(spacePtr, receive_buf, receive_buf.length);

    assert(pres1 == null)
    assert(pres2 == null)
    assert(!cres.isEmpty)
    assert(cres.length == 2)
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
    val cres        = lib.space_put_always_durable_concurrent(spacePtr, receive_buf, receive_buf.length);

    assert(cres == null)
    assert(!lib.is_empty(spacePtr));

    // Produce
    val send1     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send1_buf = send1.toByteArray;
    val pres      = lib.space_get_once_durable_concurrent(spacePtr, send1_buf, send1_buf.length);

    assert(!pres.isEmpty())
    assert(!lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }

  test("DiskConcConsumePersistExistingMatches") {
    // Produce
    val send1     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send1_buf = send1.toByteArray;
    val pres1     = lib.space_get_once_durable_concurrent(spacePtr, send1_buf, send1_buf.length);

    assert(pres1 == null)

    // Produce
    val send2     = Send("friends", Some(setup.bob), cityMatchCase(setup.alice));
    val send2_buf = send2.toByteArray;
    val pres2     = lib.space_get_once_durable_concurrent(spacePtr, send2_buf, send2_buf.length);

    assert(pres2 == null)

    // Consume
    val receive1 =
      Receive(
        Seq("friends"),
        Seq(setup.cityPattern),
        "I am the continuation, for now..."
      );
    val receive1_buf = receive1.toByteArray;
    val cres1        = lib.space_put_always_durable_concurrent(spacePtr, receive1_buf, receive1_buf.length);

    assert(cres1.length == 1)
    assert(!lib.is_empty(spacePtr));

    val receive2 =
      Receive(
        Seq("friends"),
        Seq(setup.cityPattern),
        "I am the continuation, for now..."
      );
    val receive2_buf = receive2.toByteArray;
    val cres2        = lib.space_put_always_durable_concurrent(spacePtr, receive2_buf, receive2_buf.length);

    assert(cres2.length == 1)
    assert(lib.is_empty(spacePtr));

    val receive3 =
      Receive(
        Seq("friends"),
        Seq(setup.cityPattern),
        "I am the continuation, for now..."
      );
    val receive3_buf = receive3.toByteArray;
    val cres3        = lib.space_put_always_durable_concurrent(spacePtr, receive3_buf, receive3_buf.length);

    assert(cres3 == null)
    assert(!lib.is_empty(spacePtr));

    // Produce
    val send3     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send3_buf = send3.toByteArray;
    val pres3     = lib.space_get_once_durable_concurrent(spacePtr, send3_buf, send3_buf.length);

    assert(!pres3.isEmpty())
    assert(!lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }

  test("DiskConcProducePersist") {
    // Produce
    val send     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send_buf = send.toByteArray;
    val pres     = lib.space_get_always_durable_concurrent(spacePtr, send_buf, send_buf.length);

    assert(pres == null)
    assert(!lib.is_empty(spacePtr));

    // Consume
    val receive =
      Receive(
        Seq("friends"),
        Seq(setup.cityPattern),
        "I am the continuation, for now..."
      );
    val receive_buf = receive.toByteArray;
    val cres        = lib.space_put_once_durable_concurrent(spacePtr, receive_buf, receive_buf.length);

    assert(!cres.isEmpty)
    assert(cres.length == 1)
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
    val cres1        = lib.space_put_once_durable_concurrent(spacePtr, receive1_buf, receive1_buf.length);

    assert(cres1 == null)
    assert(!lib.is_empty(spacePtr));

    // Produce
    val send1     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send1_buf = send1.toByteArray;
    val pres1     = lib.space_get_always_durable_concurrent(spacePtr, send1_buf, send1_buf.length);

    assert(!pres1.isEmpty())
    assert(lib.is_empty(spacePtr));

    // Produce
    val send2     = Send("friends", Some(setup.alice), cityMatchCase(setup.alice));
    val send2_buf = send2.toByteArray;
    val pres2     = lib.space_get_always_durable_concurrent(spacePtr, send2_buf, send2_buf.length);

    // Consume
    val receive2 =
      Receive(
        Seq("friends"),
        Seq(setup.cityPattern),
        "I am the continuation, for now..."
      );
    val receive2_buf = receive2.toByteArray;
    val cres2        = lib.space_put_once_durable_concurrent(spacePtr, receive2_buf, receive2_buf.length);

    assert(pres2 == null)
    assert(!cres2.isEmpty)
    assert(!lib.is_empty(spacePtr));
    lib.space_clear(spacePtr);
  }
}
