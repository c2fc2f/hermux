{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default-linux";
  };

  outputs =
    inputs@{
      flake-parts,
      nixpkgs,
      self,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      perSystem =
        { pkgs, ... }:
        {
          devShells = {
            default = pkgs.mkShell {
              env = {
                # Required by rust-analyzer
                RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
              };

              nativeBuildInputs = with pkgs; [
                cargo
                rustc
                rust-analyzer
                rustfmt
                clippy

                pkg-config
              ];

              buildInputs = with pkgs; [
                openssl
              ];
            };
          };

          packages = rec {
            hermes-mux = pkgs.callPackage ./package.nix {
              version =
                if self ? "shortRev" then
                  self.shortRev
                else
                  nixpkgs.lib.replaceStrings [ "-dirty" ] [ "" ] self.dirtyShortRev;
            };
            default = hermes-mux;
          };
        };
    };
}
