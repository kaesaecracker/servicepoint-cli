{
  description = "Flake for command line interface of the ServicePoint display.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-26.05";
    nix-filter.url = "github:numtide/nix-filter";
    crane.url = "github:ipetkov/crane";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      nix-filter,
      treefmt-nix,
    }:
    let
      lib = nixpkgs.lib;
      treefmt-config = {
        projectRootFile = "flake.nix";
        programs = {
          nixfmt.enable = true;
          keep-sorted.enable = true;
          rustfmt.enable = true;
        };
      };
      supported-systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      forAllSystems =
        f:
        lib.genAttrs supported-systems (
          system:
          f rec {
            pkgs = nixpkgs.legacyPackages.${system};
            treefmt-eval = treefmt-nix.lib.evalModule pkgs treefmt-config;
            inherit system;
          }
        );
    in
    rec {
      packages = forAllSystems (
        { pkgs, ... }:
        let
          craneLib = crane.mkLib pkgs;
          commonArgs = {
            src = nix-filter.lib.filter {
              root = ./.;
              include = [
                ./Cargo.toml
                ./Cargo.lock
                ./src
                ./README.md
                ./LICENSE
              ];
            };
            strictDeps = true;
            nativeBuildInputs = with pkgs; [
              pkg-config
              rustPlatform.bindgenHook
            ];
            buildInputs =
              with pkgs;
              [
                xe
                xz
                ffmpeg-headless.dev
              ]
              ++ lib.optionals pkgs.stdenv.isLinux (
                with pkgs;
                [
                  dbus
                  pipewire
                ]
              );
          };
        in
        rec {
          servicepoint-cli = craneLib.buildPackage (
            commonArgs
            // {
              cargoArtifacts = craneLib.buildDepsOnly commonArgs;
            }
          );

          default = servicepoint-cli;
        }
      );

      legacyPackages = packages;

      nixosModules.default = {
        nixpkgs.overlays = [ self.overlays.default ];
      };

      overlays.default = final: prev: {
        servicepoint-cli = self.legacyPackages."${prev.system}".servicepoint-cli;
      };

      devShells = forAllSystems (
        {
          pkgs,
          system,
          ...
        }:
        {
          default = pkgs.mkShell rec {
            inputsFrom = [ self.packages.${system}.default ];
            packages = with pkgs; [
              (pkgs.symlinkJoin {
                name = "rust-toolchain";
                paths = with pkgs; [
                  rustc
                  cargo
                  rustPlatform.rustcSrc
                  rustfmt
                  clippy
                  cargo-expand
                ];
              })

              cargo-flamegraph
              gdb
            ];
            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath (builtins.concatMap (d: d.buildInputs) inputsFrom)}";
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            RUST_LOG = "all";
            RUST_BACKTRACE = "1";
          };
        }
      );

      formatter = forAllSystems ({ treefmt-eval, ... }: treefmt-eval.config.build.wrapper);
    };
}
