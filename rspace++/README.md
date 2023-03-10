## Rust + Scala

- Using `jna`

## Notes

- Run `sbt rspacePlusPlus/run` to run `Main.scala` file in `rspace++/src/main/scala`
- `scalac <path_to_file>` to compile scala package. Ex: `scalac rspace++/src/main/scala/package.scala` - creates `rspacePlusPlus` directory at root
- `scala <path_to_file>` to run scala file. Ex: `scala rspace++/src/main/scala/example.scala`

- `rustc <path_to_file>` to compile single rust file

- Added CLI arg called `rspace-plus-plus`. When called, like `rnode run --standalone --rspace-plus-plus`, prints message that says using rspace++. When not provided, defaults to using rspace.

- `sbt <project_name>/<command>` to compile, stage, run, clean single project. For example: `node/compile node/stage` will compile and stage only node project directory.

- `sbt compile` will build Rust library in `rspace++/target/release/`. This is where JNA pulls library 

- Integrating new rspace++ into rnode setup, I think, will happen in `node/src/main/scala/coop/rchain/node/runtime/Setup.scala`