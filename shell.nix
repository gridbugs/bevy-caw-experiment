{ pkgs ? import <nixpkgs> {
  overlays = [
    (import (builtins.fetchTarball
      "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
  ];
} }:

pkgs.mkShell rec {
  packages = with pkgs; [
    (rust-bin.stable.latest.default.override {
      extensions = [ "rust-src" "rust-analysis" ];
      targets = [ "wasm32-unknown-unknown" ];
    })
    llvmPackages.bintools
    rustPlatform.rustLibSrc
    rust-analyzer
    cargo-watch
    cargo-outdated
    rustfmt
    pkg-config
    udev
    alsa-lib
    nodejs
    wasm-pack
    openssl
    SDL2
    SDL2_ttf
    libxkbcommon
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    vulkan-loader
    vulkan-tools
  ];

  # Allows rust-analyzer to find the rust source
  RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

  # Without this graphical frontends can't find the GPU adapters
  LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath packages}";

  # Needed to build bevy in debug profile
  RUST_MIN_STACK = 33554432;
}
