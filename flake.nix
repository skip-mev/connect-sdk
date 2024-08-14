{
  description = "Skip Connect SDK";

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
            binaryen
          ];
        };
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "connect-sdk-examples";
          version = "0.1.0";

          src = ./.;

          buildInputs = with pkgs; [
            rust
            binaryen
          ];

          buildPhase = ''
            cargo wasm
          '';

          installPhase = ''
            mkdir -p $out/artifacts
            for wasm in ./target/wasm32-unknown-unknown/release/*.wasm; do
              out_file=$(basename $wasm)
              wasm-opt -Os --signext-lowering $wasm -o "$out/artifacts/$out_file"
            done
          '';
        };
      }
    );
}