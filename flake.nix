{
  description = "ddns-route53";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      binary = pkgs.callPackage ./package.nix { };
    in
    {
      packages.${system}.default = binary;

      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          openssl
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];
      };
    };
}
