{
  pkgs ?
    import
      (fetchTarball "https://github.com/NixOS/nixpkgs/archive/f02fddb8acef29a8b32f10a335d44828d7825b78.tar.gz")
      { },
}:
pkgs.rustPlatform.buildRustPackage {
  pname = "proxy";
  version = "0.1.0";

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;
}
