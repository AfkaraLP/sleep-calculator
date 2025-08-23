{
  description = "Sleep calculator app with runtime deps";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forEachSystem = f:
        nixpkgs.lib.genAttrs systems (system:
          f (import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          }));
    in {
      packages = forEachSystem (pkgs: {
        default = pkgs.rustPlatform.buildRustPackage {
          pname = "sleep-calculator";
          version = "0.1.0";
          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;

          buildInputs = [ pkgs.wayland pkgs.libxkbcommon pkgs.xorg.libX11 pkgs.mesa pkgs.libglvnd ];

          nativeBuildInputs = [ pkgs.pkg-config pkgs.makeWrapper ];

          postInstall = ''
            wrapProgram $out/bin/sleep-calculator \
              --prefix LD_LIBRARY_PATH : ${
                pkgs.lib.makeLibraryPath [
                  pkgs.wayland
                  pkgs.libxkbcommon
                  pkgs.xorg.libX11
                  pkgs.mesa
                  pkgs.libglvnd
                ]
              }
          '';
        };
      });

      devShells = forEachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            rust-bin.stable.latest.default
            pkg-config
            openssl
            wayland
            libxkbcommon
            xorg.libX11
          ];
        };
      });
    };
}
