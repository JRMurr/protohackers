{ pkgs, gitignore }:

let
  rustVersion = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
  rustPlatform = pkgs.makeRustPlatform {
    cargo = rustVersion;
    rustc = rustVersion;
  };

in {
  rust-shell =
    (rustVersion.override { extensions = [ "rust-src" "rust-analyzer" ]; });
  proto-rust = rustPlatform.buildRustPackage {
    pname = "bang";
    version = "0.1.0";
    src = gitignore.lib.gitignoreSource ./.;
    cargoLock.lockFile = ./Cargo.lock;
  };
}
