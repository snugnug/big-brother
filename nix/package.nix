{
  lib,
  rustPlatform,
  pkg-config,
  openssl,
}:
rustPlatform.buildRustPackage {
  pname = "big-brother";
  version = "1.0.0";

  src = ../.;

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    openssl
  ];

  # cargoHash = lib.fakeHash;
  cargoHash = "sha256-GiZh+/C1s9S8QT5zjbat+LQ806ka+NQO7eoAgI/Ff4E=";

  meta = {
    description = "A nixpkgs tracker with notifications!";
    homepage = "https://github.com/snugnug/big-brother";
    license = lib.licenses.gpl3;
  };
}
