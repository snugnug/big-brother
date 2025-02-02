{
  lib,
  rustPlatform,
  fetchFromGitHub,
  pkg-config,
  openssl,
}: let
  version = "1.0.2";
in
  rustPlatform.buildRustPackage {
    pname = "big-brother";
    version = version;

    # src = ../.;
    # I dont know if theres a better way to do this
    src = fetchFromGitHub {
      owner = "snugnug";
      repo = "big-brother";
      tag = "v${version}";
      # hash = lib.fakeHash;
      hash = "sha256-RdGcNscZG4AkCqumjpX8Sp8wgKwP6OvLR6R9J7sIkUk=";
    };

    nativeBuildInputs = [
      pkg-config
    ];

    buildInputs = [
      openssl
    ];

    # cargoHash = lib.fakeHash;
    cargoHash = "sha256-PcG7boF+FINiWzsQfrdVzpSXOg9ni6ElolRGrB+bPPE=";

    meta = {
      description = "A nixpkgs tracker with notifications!";
      homepage = "https://github.com/snugnug/big-brother";
      license = lib.licenses.gpl3;
    };
  }
