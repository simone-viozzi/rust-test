{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustc     # Rust compiler
    cargo     # Rust package manager
    rustfmt   # Code formatter
    clippy    # Linter for Rust
    gcc       # Required for crates needing C compilers
  ];

  # Set the source path for Rust tooling (e.g., rust-analyzer)
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
