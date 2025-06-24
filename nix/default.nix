{
  version,
  lib,
  installShellFiles,
  rustPlatform,
  openssl,
  pkg-config,
}:

rustPlatform.buildRustPackage {
  pname = "hermes-mux";

  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.difference ../. (
      # don't include in build
      lib.fileset.unions [
        ../README.md
        ../LICENSE
      ]
    );
  };

  inherit version;

  # inject version from nix into the build
  env.NIX_RELEASE_VERSION = version;

  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [
    installShellFiles
    pkg-config
  ];

  buildInputs = [
    openssl
  ];

  meta = with lib; {
    description = "Program acts as a proxy for OpenRouter, allowing the use of multiple free OpenRouter accounts to handle requests. It automatically rotates between the available accounts, prioritizing those that have made the fewest requests today. This helps avoid exceeding daily usage limits for any individual account";
    mainProgram = "hermes-mux";
    homepage = "https://github.com/culxttes/hermes-mux";
    license = licenses.mit;
    maintainers = [ maintainers.culxttes ];
  };
}
