import com.sun.jna._

object HelloWorld extends Library {
  // Load the Rust library. The name of the library file may differ depending on your platform.
  System.setProperty("jna.library.path", "../target/release")

  trait RustLib extends Library {
    def hello_world(): Unit
  }

  def main(args: Array[String]): Unit = {
    // Load the Rust library
    val lib = Native.load("my_library", classOf[RustLib]).asInstanceOf[RustLib]

    // Call the hello_world function
    lib.hello_world()
  }
}

// .dylib on mac os
// .so on linux
