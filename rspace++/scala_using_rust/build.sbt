import scala.sys.process._

val scala3Version = "3.2.2"

lazy val root = project
  .in(file("."))
  .settings(
    name := "hello_world",
    version := "0.1.0-SNAPSHOT",
    scalaVersion := scala3Version,
    libraryDependencies ++= Seq(
      "org.scalameta"    %% "munit" % "0.7.29" % Test,
      "net.java.dev.jna" % "jna"    % "5.7.0"
    )
  )

lazy val runCargoBuild = taskKey[Unit]("Builds Rust library for rspace++")

runCargoBuild := {
  val log = new ProcessLogger {
    override def out(s: => String): Unit = println(s)
    override def err(s: => String): Unit = println(s)
    override def buffer[T](f: => T): T   = f
  }

  println("Running command: cargo build --release")
  val result = Process("cargo build --release").run(log)
  if (result.exitValue() != 0) {
    sys.error("Cargo build failed!")
  }
}

(compile in Compile) := ((compile in Compile) dependsOn runCargoBuild).value
