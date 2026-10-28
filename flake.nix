{
  description = "A very naïve way to match host names to IPs";

  inputs = {
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-flake = {
      url = "github:juspay/rust-flake";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } (
      { ... }: {
        imports = [
          inputs.rust-flake.flakeModules.default
          inputs.rust-flake.flakeModules.nixpkgs
        ];
        perSystem =
          { pkgs, self', ... }:
          let
            buildInputs = with pkgs; [ git ];
          in
          {
            devShells.default = pkgs.mkShell {
              inherit buildInputs;
              inputsFrom = [
                self'.devShells.rust
              ];
              nativeBuildInputs = with pkgs; [ pkg-config ];
              packages = with pkgs; [ bacon ];
              LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
            };
            formatter = pkgs.nixfmt;
            packages.default = self'.packages.knave;
          };
        systems = [ "x86_64-linux" ];
      }
    );
}
