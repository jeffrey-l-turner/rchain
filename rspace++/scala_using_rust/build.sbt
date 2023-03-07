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
