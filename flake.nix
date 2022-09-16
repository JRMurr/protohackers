{
  description = "protohackers";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils = { url = "github:numtide/flake-utils"; };
    gitignore = {
      url = "github:hercules-ci/gitignore.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, gitignore, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustAttrs = import ./rust { inherit pkgs gitignore; };
      in with pkgs; {
        defaultPackage = rustAttrs.proto-rust;
        packages = {
          rust-bin = rustAttrs.proto-rust;
          rust-docker = rustAttrs.proto-rust-docker;
        };
        devShell = mkShell {
          buildInputs = [
            rustAttrs.rust-shell
            cargo-expand
            # common
            watchexec

            nixfmt

            just
          ];
        };
      });
}
