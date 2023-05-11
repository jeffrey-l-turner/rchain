## Notes: Rust + Scala

- Using `jna`, `prost`, `heed`, `dashmap`

## Quickstart

1. `cd rspace++` & run `cargo build && cargo build --release`
2. `cd ..` to be in root directory and run `sbt rspacePlusPlus/run`. &nbsp; `rspacePlusPlus/run` if already in sbt shell
3. Run Scala tests: In root directory run `sbt rspacePlusPlus/test`. &nbsp; `rspacePlusPlus/test` if already in sbt shell

## Scala

- Run `sbt rspacePlusPlus/run` to run `example.scala` file in `rspace++/src/main/scala`
- Run `sbt rsapcePlusPlus/compile` to compile rspace++ subproject. Build corresponding `.proto` file for Scala. Outputs to `rspace++/target/scala-2.12/src_managed/`
  
- `scalac <path_to_file>` to compile scala package. Ex: `scalac rspace++/src/main/scala/package.scala` - creates `rspacePlusPlus` directory at root
- `scala <path_to_file>` to run scala file. Ex: `scala rspace++/src/main/scala/example.scala`

- Added CLI arg called `rspace-plus-plus`. When called, like `rnode run --standalone --rspace-plus-plus`, prints message that says using rspace++. When not provided, defaults to using rspace.

- `sbt <project_name>/<command>` to compile, stage, run, clean single project. For example: `node/compile node/stage` will compile and stage only node project directory.

- `sbt compile` will compile entire project, also builds Rust library in `rspace++/target/release/`. This is where JNA pulls library 

- Integrating new rspace++ into rnode setup, I think, will happen in `node/src/main/scala/coop/rchain/node/runtime/Setup.scala`

- `scalafmt <file_path>` to format `.scala` file

## Rust

- Run sample code: `cargo run` within `rspace++` directory
- `rustc <path_to_file>` to compile single rust file
- `cargo build --release` to build `rspace_plus_plus` library. Outputs to `rspace++/target/release/`. Scala code pulls from here.
- `cargo build` to build corresponding `.proto` file for Rust. Outputs to `rspace++/target/debug/`

<br>

- Run tests sequentially: `cargo test -- --test-threads=1` within `rspace++` directory.
- Run specific test file sequentially: `cargo test --test my_test_file -- --test-threads=1` within `rspace++` directory.
- `cargo test --test my_test_file -- --test-threads=1` tests all the functions in a single file

## Backlog

1. Get working correct return types from Rust functions in Scala
2. Create convenient name schema for proto messages throught Rust, Scala and test code
3. Optimize loops marked with TODO: in memory databases
4. Handle continuation data type. Currently string. Might be pointer to function. See original code and tutorial 
5. Revist core database code and reduce cloning? Utilize references? 
6. Rewrite Rust rspace unit tests to match current API
7. Implement common syntax for all crate imports