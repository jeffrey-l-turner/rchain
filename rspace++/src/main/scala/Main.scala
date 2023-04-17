import com.sun.jna._

object BuildRustLibrary extends Library {
  // Load the Rust library. The name of the library file may differ depending on your platform.
  val _ = System.setProperty("jna.library.path", "./rspace++/target/release")

  trait RustLib extends Library {
    def get_once_durable_concurrent(): Unit
  }

  def main(args: Array[String]): Unit = {
    // Load the Rust library
    val rspace = Native.load("rspace_plus_plus", classOf[RustLib]).asInstanceOf[RustLib]

    // Call the hello_world function
    rspace.get_once_durable_concurrent();
  }
}

// .dylib on mac os
// .so on linux
