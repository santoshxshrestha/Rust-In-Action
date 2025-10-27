{
  description = "Flake for Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          nodejs
          rustc
          cargo
          cargo-watch

          # clippy
          rustfmt
          openssl
          wasm-pack
          rocmPackages.llvm.lld
          python3
          # rust-analyzer
          # gcc
          # pkg-config
        ];
        env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    };
}
