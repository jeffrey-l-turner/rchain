Notes on building RNode on Windows
Install Windows Subsystem for Linux -> https://learn.microsoft.com/en-us/windows/wsl/install#install-wsl-command
Install Ubuntu -> wsl --install Ubuntu
Open Ubuntu terminal app or restart windows terminal app and add ubuntu profile
In ubuntu terminal, install nix -> https://nixos.org/download.html#nix-install-linux
Check Nix was installed correctly, run nix-shell -p hello then:
[nix-shell:~]$ hello
Hello, world!

[nix-shell:~]$ exit

$ hello
hello: command not found
More info on Nix -> https://nixos.org/manual/nix/stable/introduction.html

Check Git is installed: git --version if not then: sudo apt-get install git
Configure .gitconfig
Copy ssh keys to Ubuntu: cp -r /mnt/c/Users/<username>/.ssh ~/.ssh or cp -r /mnt/c/Users/<username>/.ssh/<file-name> ~/.ssh
If bad permission, then change for specefied file: chmod 600 ~/.ssh/<file-name>
Test connection: ssh -T git@github.com
Create src directory: mkdir src && cd src and clone project git clone git@github.com:rchain/rchain.git
Using Readme -> https://github.com/rchain/rchain/tree/dev/nix#rchain-project-setupbuild-using-nix
Install Java: sudo apt install default-jdk and check: java --version Currently using 11.0.17
Install Scala build tool -> https://www.scala-sbt.org/1.x/docs/Installing-sbt-on-Linux.html#Ubuntu+and+other+Debian-based+distributions
Install jflex and bnfc with nix: nix-env -i jflex -iA haskellPackages.BNFC --file https://github.com/NixOS/nixpkgs-channels/archive/nixos-20.03.tar.gz
Uninstall: nix-env -e jflex BNFC
Install in case of error (Ubuntu): sudo apt-get install libgmp3-dev
Compile: sbt compile
Default memory limits may not be sufficient so additional options for sbt can be specified. They can be added to .bashrc file. Increase heap memory and thread stack size. Disable supershell if empty lines are printed in sbt output -> Add to .bashrc: export SBT_OPTS="-Xmx4g -Xss2m -Dsbt.supershell=false"
Compile with tests: sbt test:compile
