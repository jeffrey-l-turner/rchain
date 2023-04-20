import com.sun.jna._

object BuildRustLibrary extends Library {
  // Load the Rust library. The name of the library file may differ depending on your platform.
  val _ = System.setProperty("jna.library.path", "./rspace++/target/release")

  // Define interface for Rust functions
  trait RustLib extends Library {
    def space_new(): Pointer

    // Examples
    def MyStruct_new(x: Int, y: Int): Long
    def MyStruct_add(myStructPtr: Long): Int

    def print_types(x: Int, y: Double): Unit
  }

  def main(args: Array[String]): Unit = {
    // Load the Rust library
    val lib = Native.load("rspace_plus_plus", classOf[RustLib]).asInstanceOf[RustLib]

    // @SuppressWarnings(Array("unused"))
    val spacePtr = lib.space_new()
    println(spacePtr)

    // Examples
    val myStructPtr = lib.MyStruct_new(1, 2)
    val result      = lib.MyStruct_add(myStructPtr)
    println(result)

    lib.print_types(42, 3.14);
  }
}

// .dylib on mac os
// .so on linux
