{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    devshell.url = "github:numtide/devshell";
  };

  outputs = { nixpkgs, rust-overlay, devshell, flake-utils, ... }: 
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) devshell.overlays.default ];
        };

        toolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        });

        radix-colors-json = pkgs.callPackage (import ./radix-colors-to-json.nix) {};
      in {
        devShell = pkgs.devshell.mkShell {
          packages = [ toolchain pkgs.yarn pkgs.clang pkgs.cargo-expand ];
          motd = "\n  Welcome to the {2}radix-colors-rs{reset} shell.\n";
        };
        packages = {
          inherit radix-colors-json;
          default = radix-colors-json;
        };
      });
}
