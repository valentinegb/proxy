let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/archive/f02fddb8acef29a8b32f10a335d44828d7825b78.tar.gz";
  pkgs = import nixpkgs {
    config = { };
    overlays = [ ];
  };
in
{
  proxy = pkgs.callPackage ./proxy.nix { };
}
