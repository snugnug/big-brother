{
  description = "Big Brother Nix Flake";

  inputs = {nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";};

  outputs = inputs @ {
    self,
    nixpkgs,
  }: let
    inherit (nixpkgs) lib;
    inherit (builtins) attrValues;
    eachSystem = f:
      lib.genAttrs ["x86_64-linux"]
      (system: f nixpkgs.legacyPackages.${system});
  in {
    packages = eachSystem (pkgs: rec {
      big-brother = pkgs.callPackage ./nix/package.nix {};
      default = big-brother;
    });

    nixosModules = eachSystem (pkgs: rec {
      big-brother = import ./nix/module.nix inputs;
      default = big-brother;
    });

    devShells = eachSystem (pkgs: {
      default = pkgs.mkShell {
        packages = attrValues {
          inherit
            (pkgs)
            cargo
            rustc
            rust-analyzer
            rustfmt
            pkg-config
            openssl
            sqlx-cli
            ;
        };
      };
    });
  };
}
