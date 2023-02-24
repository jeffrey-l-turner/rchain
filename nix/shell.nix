with (import <nixpkgs> {});

let 
	jdk = pkgs.jdk11;
	sbt = pkgs.sbt.override { jre = pkgs.jdk11; };
	# jflex = pkgs.jflex

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
		jflex-1.7.0
		sbt 
		jdk
  ];

	shellHook = ''
      export SBT_OPTS="-Xmx4g -Xss2m -Dsbt.supershell=false"
			export PATH="/usr/local/opt/openjdk/bin:$PATH"
  '';

  RUST_BACKTRACE = 1;
}
