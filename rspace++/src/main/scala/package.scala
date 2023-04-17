package rspacePlusPlus

import com.sun.jna._

object RustLibrary extends Library {
  // Load the Rust library. The name of the library file may differ depending on your platform.
  val _ = System.setProperty("jna.library.path", "./rspace++/target/release")

  trait RustLib extends Library {
    def hello_world(): Unit
  }

  // Load the Rust library and call the hello_world function
  def helloWorld(): Unit = {
    val lib = Native.load("rust_library", classOf[RustLib]).asInstanceOf[RustLib]
    lib.hello_world()
  }
}
