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
      hash = "sha256-wJ9aunZ/i6H47w/sZN/ikcRlpOwntJ60DOnJuld2tfU=";
    };

    nativeBuildInputs = [
      pkg-config
    ];

    buildInputs = [
      openssl
    ];

    # cargoHash = lib.fakeHash;
    cargoHash = "sha256-NH7HWBKiK35yhGVVc5v2gTBRSefduZF2Z2c+qS9xDtQ=";

    meta = {
      description = "A nixpkgs tracker with notifications!";
      homepage = "https://github.com/snugnug/big-brother";
      license = lib.licenses.gpl3;
    };
  }
