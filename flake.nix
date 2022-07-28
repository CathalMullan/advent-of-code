{
  description = "Advent of Code";

  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-22.05";
    };

    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let

        inherit (pkgs) lib stdenv mkShell fetchFromGitHub;
        inherit (pkgs.darwin.apple_sdk.frameworks) SystemConfiguration;

        pkgs = import nixpkgs {
          inherit system;
        };

        rust-toolchain = fenix.packages."${system}".fromToolchainFile {
          file = "${self}/rust-toolchain.toml";
          sha256 = "sha256-T3ZIxjAX81wEq1SMnieh60c6xFvJ4rBZexeBS/3zGF4=";
        };

        naerskPlatform = naersk.lib.${system}.override {
          cargo = rust-toolchain;
          rustc = rust-toolchain;
        };

        cargo-cranky = naerskPlatform.buildPackage {
          pname = "cargo-cranky";
          version = "0.3.0";

          src = fetchFromGitHub {
            owner = "ericseppanen";
            repo = "cargo-cranky";
            rev = "v0.3.0";
            sha256 = "sha256-3ARl3z+2nz05UaKf8ChN6mvPY2qMjUNxGnGJ1P0xkas=";
          };
        };
      in
      rec {
        # nix develop
        devShell = mkShell {
          name = "advent-of-code-shell";

          buildInputs = with pkgs;
            [
              # Build
              pkg-config
              libiconv

              # Rust
              rust-toolchain
              cargo-cranky
              cargo-nextest

              # Nix
              nixpkgs-fmt
              rnix-lsp
            ] ++ lib.optional (stdenv.isDarwin) [
              # Build
              SystemConfiguration
            ];
        };
      });
}
