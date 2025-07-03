{
  self,
  lib,
  inputs,
}:

{
  default = lib.composeManyExtensions (
    with self.overlays;
    [
      hermux-packages
    ]
  );

  hermux-packages = lib.composeManyExtensions [
    (final: prev: {
      hermux = final.callPackage ./default.nix {
        version =
          if self ? "shortRev" then
            self.shortRev
          else
            lib.replaceStrings [ "-dirty" ] [ "" ] self.dirtyShortRev;
      };
    })
  ];
}
