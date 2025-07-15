{
  description = "Flake for rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: {
    devShells.x86_64-linux.default = let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
      };
    in pkgs.mkShell {
      buildInputs = with pkgs; [
      clippy
      rustfmt
        rustc
          cargo 
          rust-analyzer
      ];

      shellHook = ''
        cargo --version
        '';
    };

  };
}
