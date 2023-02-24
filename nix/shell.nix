{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    git
    ps
    bash
    which
  ];

  RUST_BACKTRACE = 1;
}
