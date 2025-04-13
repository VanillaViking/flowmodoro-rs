{
description = "my project description";

inputs.flake-utils.url = "github:numtide/flake-utils";

outputs = { self, nixpkgs, flake-utils }:
flake-utils.lib.eachDefaultSystem
(system:
  let pkgs = nixpkgs.legacyPackages.${system}; in
  {
    devShells.default = import ./shell.nix { inherit pkgs; };
    packages.default = pkgs.rustPackages.rustPlatform.buildRustPackage {
      pname = "flowmodoro-rs";
      version = "0.1.0";
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;
    };
  }
  );
}
