// import com.sun.jna._

// object MyRustLib extends Library {
//   // Load the Rust library. The name of the library file may differ depending on your platform.
//   System.setProperty("jna.library.path", "../../../target/debug/librspace_plus_plus.rlib")
//   Native.register("rspace_plus_plus")

//   // Define the Rust functions that you want to call from Scala
//   def printHelloWorld(): Int = nativeMethod()
//   // def printHelloWorld(s: String): Unit = {
//   //   val string = new NativeString(s, "UTF-8")
//   //   nativeMethod(string)
//   // }

//   // Declare the native methods that you want to call from Scala
//   @native def nativeMethod(): Int
//   // @native def nativeMethod(s: NativeString): Unit
// }

// @main def helloWorld: Unit = 
//   MyRustLib.printHelloWorld();

import com.sun.jna._

object HelloWorld extends Library {
	// Load the Rust library. The name of the library file may differ depending on your platform.
	System.setProperty("jna.library.path", "../target/release/libmy_library.dylib")

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
