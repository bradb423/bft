{ pkgs ? import <nixpkgs> { } }:

let
  unstableTarball =
    fetchTarball
      https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz;
  pkgs = import <nixpkgs> { };
  unstable = import unstableTarball { };
in

pkgs.mkShell {
  name = "rust-shell";
  nativeBuildInputs = with pkgs; [ unstable.rustc unstable.cargo unstable.gcc ];
  buildInputs = with pkgs; [ unstable.rustfmt unstable.clippy ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
