#{
#  description = "A flake to build a Rust project to WASM";
#
#  inputs = {
#    nixpkgs.url = "github:nixos/nixpkgs";
#    utils.url = "github:numtide/flake-utils";
#
#    rust-overlay.url = "github:oxalica/rust-overlay";
#  };
#
#  outputs = { self, nixpkgs, utils, rust-overlay }:
#    utils.lib.eachDefaultSystem (system:
#      let
#        buildTarget = "wasm32-unknown-unknown";
#
#        pkgs = import nixpkgs {
#          inherit system;
#          overlays = [ rust-overlay.overlays.default ];
#        };
#
#        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
#          targets = [ buildTarget ];
#        };
#
#        rustPlatform = pkgs.makeRustPlatform {
#          cargo = rustToolchain;
#          rustc = rustToolchain;
#        };
#      in {
#        #packages = {
#          #x-oracle-passthrough = rustPlatform.buildRustPackage {
#          #  name = "x-oracle-passthrough";
#          #  src = "./contracts/x-oracle-passthrough";
#          #  cargoLock.lockFile = ./Cargo.lock;
#          #};
#        #};
#        packages.default = rustPlatform.buildRustPackage {
#          name = "connect-sdk";
#          src = ./.;
#
#          cargoLock.lockFile = ./Cargo.lock;
#
#          buildPhase = ''
#            cargo build --release -p connect-sdk --target=${buildTarget}
#          '';
#  
#          installPhase = ''
#            mkdir -p $out/lib
#            cp target/${buildTarget}/release/*.wasm $out/lib/
#          '';
#  
#          # Disable checks if they only work for WASM
#          # doCheck = false;
#      };
#      }
#  );
#}
{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            rust
          ];
        };
      }
    );
}