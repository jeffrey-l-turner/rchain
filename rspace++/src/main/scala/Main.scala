import com.sun.jna._

object BuildRustLibrary extends Library {
  // Load the Rust library. The name of the library file may differ depending on your platform.
  val _ = System.setProperty("jna.library.path", "./rspace++/target/release")

  trait RustLib extends Library {
    def hello_world(): Unit
  }

  def main(args: Array[String]): Unit = {
    // Load the Rust library
    val lib = Native.load("rust_library", classOf[RustLib]).asInstanceOf[RustLib]

    // Call the hello_world function
    lib.hello_world()
  }
}

// .dylib on mac os
// .so on linux
