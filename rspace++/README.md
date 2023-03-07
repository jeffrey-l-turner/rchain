## Rust + Scala

- Using `jna`

## Steps to run (in nix-shell)

1. Under `rspace++` directory, run `cargo build --release`. Should create `target/release` directory
2. `cd scala_using_rust` to get into temp scala directory and run `sbt compile run`. Executes `src/main/scala/Main.scala` which has `jna` configured to use `target/release` path along with Rust function defined and then calls function that prints "hello world". Uses file `rspace++/src/lib.rs`

