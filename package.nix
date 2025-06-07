{
  lib,
  pkgs,
  rustPlatform,
}:

rustPlatform.buildRustPackage {
  pname = "ddns-route53";
  version = "0.1.0";

  src = ./.;

  buildInputs = with pkgs; [
    openssl
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
