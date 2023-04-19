import com.sun.jna._

object BuildRustLibrary extends Library {
  // Load the Rust library. The name of the library file may differ depending on your platform.
  val _ = System.setProperty("jna.library.path", "./rspace++/target/release")

  // Define interface for Rust functions
  trait RustLib extends Library {
    // def create(): Unit
    def create_my_struct(a: Int, b: Int): Pointer
    def print_types(x: Int, y: Double): Unit
  }

  def main(args: Array[String]): Unit = {
    // Load the Rust library
    val lib = Native.load("rspace_plus_plus", classOf[RustLib]).asInstanceOf[RustLib]

    // rspace.create();
    lib.print_types(42, 3.14);

  }
}

// .dylib on mac os
// .so on linux
