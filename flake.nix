{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nixpkgs-mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };
  outputs = { self, flake-utils, naersk, nixpkgs, nixpkgs-mozilla }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [
            (import nixpkgs-mozilla)
          ];
        };
        toolchain = (pkgs.rustChannelOf {
          rustToolchain = ./rust-toolchain.toml;
          sha256 = "sha256-xaVCskjfBe+XQHSbPi9OwGzLCc9BaWvi5wA/BZQGu0Q=";
          #        ^ After you run `nix build`, replace this with the actual
          #          hash from the error message
        }).rust;
        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };
      in {
        packages = {
          default = naersk'.buildPackage {
            src = ./.;
          };
          check = naersk'.buildPackage {
            src = ./.;
            mode = "check";
          };
          clippy = naersk'.buildPackage {
            src = ./.;
            mode = "clippy";
          };
        };

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
              toolchain
              sqlx-cli          
              bashInteractive
              sqlite
              rust-analyzer
              bun
              nodePackages.typescript-language-server
          ];
        };
      }
    );
}
