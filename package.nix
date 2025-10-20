{
  lib,
  pkgs,
  rustPlatform,
}:

rustPlatform.buildRustPackage {
  pname = "ddns-route53";
  version = "0.2.0";

  src = ./.;

  buildInputs = with pkgs; [
    openssl
  ];

  nativeBuildInputs = with pkgs; [
    pkgconf
  ];

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
