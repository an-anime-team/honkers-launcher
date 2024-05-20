let
    nixpkgs-unstable = builtins.fetchTarball "https://github.com/NixOS/nixpkgs/archive/refs/heads/nixos-unstable.tar.gz";

    pkgs = import <nixpkgs> {};
    pkgs-unstable = import nixpkgs-unstable {};

in

pkgs.mkShell {
    nativeBuildInputs = [
        pkgs-unstable.rustc
        pkgs-unstable.cargo
        pkgs-unstable.clippy

        pkgs.gcc
        pkgs.cmake
        pkgs.pkg-config
    ];

    buildInputs = [
        pkgs.gtk4
        pkgs.glib
        pkgs.gdk-pixbuf
        pkgs.gobject-introspection

        pkgs.libadwaita
    ];

    RUST_SRC_PATH = "${pkgs-unstable.rust.packages.stable.rustPlatform.rustLibSrc}";
    RUST_BACKTRACE = 1;
}
