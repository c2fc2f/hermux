{
  description = "HerMux acts as a proxy for OpenRouter, allowing the use of multiple free OpenRouter accounts to handle requests. It automatically rotates between the available accounts, prioritizing those that have made the fewest requests today. This helps avoid exceeding daily usage limits for any individual account";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      systems,
      ...
    }:
    let
      inherit (nixpkgs) lib;
      eachSystem = lib.genAttrs (import systems);

      pkgsFor = eachSystem (
        system:
        import nixpkgs {
          localSystem = system;
          overlays = with self.overlays; [
            hermux-packages
          ];
        }
      );
    in
    {
      overlays = import ./nix/overlays.nix { inherit self lib inputs; };

      packages = eachSystem (system: {
        default = self.packages.${system}.hermux;
        inherit (pkgsFor.${system})
          hermux
          ;
      });

      devShells = eachSystem (system: {
        default =
          pkgsFor.${system}.mkShell.override
            {
              inherit (self.packages.${system}.default) stdenv;
            }
            {
              env = {
                # Required by rust-analyzer
                RUST_SRC_PATH = "${pkgsFor.${system}.rustPlatform.rustLibSrc}";
              };

              nativeBuildInputs = with pkgsFor.${system}; [
                cargo
                rustc
                rust-analyzer
                rustfmt
                clippy

                pkg-config
              ];

              buildInputs = with pkgsFor.${system}; [
                openssl
              ];
            };
      });
    };
}
