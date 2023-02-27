with (import <nixpkgs> {});

let
    jdk = pkgs.jdk11;
    sbt = pkgs.sbt.override { jre = pkgs.jdk11; };
    jflex = import (fetchTarball https://github.com/NixOS/nixpkgs/archive/fcc8660d359d2c582b0b148739a72cec476cfef5.tar.gz) { }; 

in mkShell {
  buildInputs = with pkgs; [
    ammonite
    coursier
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    git
    ps
    bash
    which
    less
    haskellPackages.BNFC
    git 
    neovim
    jflex.jflex
    scala
    coursier
    scalafmt
    sbt 
    jdk
    zsh
    vscode
  ];

    shellHook = ''
      export SBT_OPTS="-Xmx4g -Xss2m -Dsbt.supershell=false"
            export PATH="/usr/local/opt/openjdk/bin:$PATH"
  '';

  RUST_BACKTRACE = 1;
}
