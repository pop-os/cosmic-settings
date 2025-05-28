{
  description = "Settings application for the COSMIC desktop environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; };

        runtimeDependencies = with pkgs; [
        wayland
        # libglvnd # For libEGL
        vulkan-loader
        ];

      in {
        devShells.default = with pkgs; mkShell rec {
          nativeBuildInputs = with pkgs; [
            just
            pkg-config
          ];

          buildInputs = with pkgs; [
            clang
            cargo
            rustc
            rustfmt
            systemdMinimal
            bashInteractive
            libxkbcommon
            freetype
            fontconfig
            expat
            lld
            desktop-file-utils
            stdenv.cc.cc.lib
            libinput
            libpulseaudio.dev
            pipewire.dev
          ];

          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          RUSTFLAGS = "-C link-arg=-Wl,-rpath,${pkgs.lib.makeLibraryPath runtimeDependencies}";
          LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
      }
    );
}
