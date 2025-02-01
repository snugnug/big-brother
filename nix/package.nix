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
  cargoHash = "sha256-8dZmCK0d15yf1mOzEmLhFL6XGmsabMn7bVfj5wS1rEM=";

  meta = {
    description = "A nixpkgs tracker with notifications!";
    homepage = "https://github.com/snugnug/big-brother";
    license = lib.licenses.gpl3;
  };
}
