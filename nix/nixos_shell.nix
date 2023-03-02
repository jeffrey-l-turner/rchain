{ pkgs ? import <nixpkgs> {
  config = {
    packageOverrides = pkgs: {
      sbt = pkgs.sbt.override { jre = pkgs.openjdk11; };
      buildFHSUserEnv = pkgs.buildFHSUserEnv.override {
        name = "akka-grpc";
        targetPkgs = pkgs: [pkgs.sbt pkgs.glibc ];
      };
    };
  };
}}:

let 
  pkgs = import (builtins.fetchTarball {
        url = "https://github.com/NixOS/nixpkgs/archive/fcc8660d359d2c582b0b148739a72cec476cfef5.tar.gz";
    }) {};

  myPkg = pkgs.jflex;

in (pkgs.buildFHSUserEnv {
  name = "rchain";

  targetPkgs = pkgs: with pkgs; [ sbt glibc haskellPackages.BNFC git myPkg jdk11 which rustc cargo rustfmt zsh ];

  profile = ''
    export SBT_OPTS="-Xmx4g -Xss2m -Dsbt.supershell=false"
    alias rnode="./node/target/universal/stage/bin/rnode"
    export PATH="$PATH:/usr/bin"
  '';  
# runScript = "sbt";
}).env
