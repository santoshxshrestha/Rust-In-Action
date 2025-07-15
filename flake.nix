{
  description = "Flake for Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        rustc
        cargo
        clippy
        rustfmt
        rust-analyzer
        gcc
        pkg-config
      ];

      shellHook = ''
        echo "🦀 Welcome to the Rust dev shell!"
        cargo --version
        rustc --version
      '';
    };
  };
}
