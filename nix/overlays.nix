{
  self,
  lib,
  inputs,
}:

{
  default = lib.composeManyExtensions (
    with self.overlays;
    [
      hermes-mux-packages
    ]
  );

  hermes-mux-packages = lib.composeManyExtensions [
    (final: prev: {
      hermes-mux = final.callPackage ./default.nix {
        version =
          if self ? "shortRev" then
            self.shortRev
          else
            lib.replaceStrings [ "-dirty" ] [ "" ] self.dirtyShortRev;
      };
    })
  ];
}
