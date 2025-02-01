{
  lib,
  rustPlatform,
  fetchFromGitHub,
  pkg-config,
  openssl,
}: let
  version = "1.0.0";
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
      hash = "sha256-iJF2fIcTcUGpDlMV5SBx9zBUFVeCETtfabuuJTqSDz8=";
    };

    nativeBuildInputs = [
      pkg-config
    ];

    buildInputs = [
      openssl
    ];

    # cargoHash = lib.fakeHash;
    cargoHash = "sha256-8dZmCK0d15yf1mOzEmLhFL6XGmsabMn7bVfj5wS1rEM=";

    meta = {
      description = "A nixpkgs tracker with notifications!";
      homepage = "https://github.com/snugnug/big-brother";
      license = lib.licenses.gpl3;
    };
  }
