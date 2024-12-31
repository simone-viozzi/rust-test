{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  # Define the native build inputs needed for your project
  nativeBuildInputs = with pkgs; [
    rustc       # Rust compiler
    cargo       # Rust package manager
    rustfmt     # Rust code formatter
    clippy      # Rust linter
    gcc         # Required for crates needing C compilers
    pkg-config  # Helps locate libraries like OpenSSL
    openssl     # OpenSSL library for crates like openssl-sys
  ];

  # Set the source path for Rust tooling (e.g., rust-analyzer)
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

  # Optional shellHook to export additional environment variables
  shellHook = ''
    # Export the PKG_CONFIG_PATH to ensure pkg-config works correctly with OpenSSL
    export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"

    echo "Environment setup complete. You are now in a Rust development shell!"
  '';
}
