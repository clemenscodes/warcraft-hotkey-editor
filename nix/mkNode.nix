{
  nixpkgs,
  system,
  ...
}: let
  pkgs = import nixpkgs {inherit system;};
in
  # Build Node.js 24.14.1 from source using the versioned nixpkgs recipe.
  # v24.nix hardcodes the version and carries all required backport patches;
  # the exact build is frozen by the nixpkgs rev in flake.lock.
  pkgs.callPackage "${nixpkgs}/pkgs/development/web/nodejs/v24.nix" {}
