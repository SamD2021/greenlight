{
  description = "A Nix-flake-based Go 1.22 development environment";

  inputs.nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1";

  outputs =
    inputs:
    let
      goVersion = 23; # Change this to update the whole stack

      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        f:
        inputs.nixpkgs.lib.genAttrs supportedSystems (
          system:
          f {
            pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = [ inputs.self.overlays.default ];
            };
          }
        );
    in
    {
      overlays.default = final: prev: {
        go = final."go_1_${toString goVersion}";
        tinygo-bin = final.stdenv.mkDerivation {
          pname = "tinygo";
          version = "0.31.2";
          src = final.fetchzip {
            url = "https://github.com/tinygo-org/tinygo/releases/download/v0.37.0/tinygo0.37.0.linux-amd64.tar.gz";
            hash = "sha256-KV177D7Knqg04jSJBNNeiJvgitK3k3ZyCWLw1qwtznk=";
          };
          installPhase = ''
            mkdir -p $out/bin
            cp -r * $out/
          '';
        };

      };

      devShells = forEachSupportedSystem (
        { pkgs }:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              # go (version is specified by overlay)
              go

              # goimports, godoc, etc.
              gotools
              gopls
              extism-cli
              wasm-tools
              tinygo-bin

              # https://github.com/golangci/golangci-lint
              golangci-lint
            ];
          };
        }
      );
    };
}
