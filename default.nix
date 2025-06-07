{ lib, rustPlatform, }:

let
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
in
  rustPlatform.buildRustPackage {
    pname = cargoToml.name;
    version = cargoToml.version;

    src = ./.;

    useFetchCargoVendor = true;
    cargoHash = "sha256-Se2u40LXKtju8vIlaQYr2pKrtOpGKd5vphnljGa7WN0=";
  }
