{ pkgs, gitignore }:

let
  rustVersion = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
  rustPlatform = pkgs.makeRustPlatform {
    cargo = rustVersion;
    rustc = rustVersion;
  };
  name = "proto-rust";
  version = "0.1.0";
  problem = 0;
  rustBin = rustPlatform.buildRustPackage {
    pname = name;
    version = version;
    src = gitignore.lib.gitignoreSource ./.;
    cargoLock.lockFile = ./Cargo.lock;
  };
in {
  rust-shell =
    (rustVersion.override { extensions = [ "rust-src" "rust-analyzer" ]; });
  proto-rust = rustBin;
  proto-rust-docker = pkgs.dockerTools.buildImage {
    name = name;
    config = {
      Cmd = [
        "${rustBin}/bin/proto-rust"
        "--host"
        "0.0.0.0"
        "${toString problem}"
      ];
    };
  };
}
