with (import <nixpkgs> {});

let 
	pkgs = import (builtins.fetchTarball {
        url = "https://github.com/NixOS/nixpkgs/archive/fcc8660d359d2c582b0b148739a72cec476cfef5.tar.gz";
    }) {};
		
	jdk = pkgs.jdk11;
	sbt = pkgs.sbt.override { jre = pkgs.jdk11; };
	jflex = pkgs.jflex;

in mkShell {
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
    haskellPackages.BNFC
    git 
    jflex
    sbt 
    jdk
  ];

	shellHook = ''
      export SBT_OPTS="-Xmx4g -Xss2m -Dsbt.supershell=false"
			export PATH="/usr/local/opt/openjdk/bin:$PATH"
  '';

  RUST_BACKTRACE = 1;
}
