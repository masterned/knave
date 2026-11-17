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
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      imports = with inputs.rust-flake.flakeModules; [
        default
        nixpkgs
      ];
      perSystem =
        { pkgs, self', ... }:
        let
          crateName = (fromTOML (builtins.readFile ./Cargo.toml)).package.name;
          libs = with pkgs; [ ];
        in
        {
          devShells.default = pkgs.mkShell {
            inputsFrom = [ self'.devShells.rust ];
            packages = with pkgs; [
              git
              jujutsu
            ];
          };
          formatter = pkgs.nixfmt;
          packages.default = self'.packages.${crateName};
          rust-project.crates.${crateName}.crane.args.buildInputs = libs;
        };
      systems = [ "x86_64-linux" ];
    };
}
