{
  description = "Rust devShell with stable rust-analyzer support";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };

        rustAnalyzer = pkgs.rust-bin.stable.latest.rust-analyzer;

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rust
            rustAnalyzer
            pkgs.openssl
            pkgs.pkg-config
            pkgs.eza
            pkgs.fd
          ];

          shellHook = ''
            alias ls=eza
            alias find=fd
            export RUST_SRC_PATH="${rust}/lib/rustlib/src/rust/library"
          '';
        };
      }
    );
}
