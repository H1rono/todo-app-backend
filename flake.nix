{
  description = "server side implementation of my todo app";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, flake-utils, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-riZUc+R9V35c/9e8KJUE+8pzpXyl0lRXt3ZkKlxoY0g=";
        };
        rustPlatform = pkgs.makeRustPlatform {
          rustc = toolchain;
          cargo = toolchain;
        };
        nativeBuildInputs = [ pkgs.libiconv ];
        buildInputs = pkgs.lib.optionals pkgs.stdenvNoCC.isDarwin [ pkgs.darwin.Security ];
      in
      {
        devShells.default = pkgs.mkShell {
          name = "todo-app-backend";
          packages = [ toolchain ] ++ nativeBuildInputs ++ buildInputs;
        };
        packages.default = rustPlatform.buildRustPackage {
          pname = "todo-app-backend";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          inherit nativeBuildInputs buildInputs;
          doCheck = false;
        };
      }
    );
}
