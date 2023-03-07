val scala3Version = "3.2.2"

lazy val root = project
  .in(file("."))
  .settings(
    name := "hello_world",
    version := "0.1.0-SNAPSHOT",

    scalaVersion := scala3Version,

		libraryDependencies ++= Seq(
			"org.scalameta" %% "munit" % "0.7.29" % Test,
			"net.java.dev.jna" % "jna" % "5.7.0"
    )
  )

// name := "MyScalaProject"
// version := "0.1"
// scalaVersion := "2.13.6"

// libraryDependencies += "com.github.jnr" % "jnr-ffi" % "2.1.10"

// resolvers += Resolver.mavenLocal

// lazy val root = (project in file("."))
//   .settings(
//     libraryDependencies ++= Seq(
//       "com.github.jnr" % "jnr-ffi" % "2.1.10",
//       "com.github.jnr" % "jnr-constants" % "0.9.12",
//       "com.github.jnr" % "jnr-posix" % "3.0.60",
//       "com.github.jnr" % "jnr-x86asm" % "1.0.2",
//       "org.scalaj" % "scalaj-http_2.12" % "2.4.2"
//     ),
//     Compile / sourceDirectories += baseDirectory.value / "../src"
//   )

// Compile / run := {
//   System.setProperty("jna.library.path", baseDirectory.value / "../target/debug")
//   val MyRustLib = Native.load("rspace_plus_plus", classOf[MyRustLib]).asInstanceOf[MyRustLib]
//   MyRustLib.main()
// }

