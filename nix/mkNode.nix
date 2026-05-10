{
  nixpkgs,
  system,
  ...
}: let
  pkgs = import nixpkgs {inherit system;};
  nodeDir = "${nixpkgs}/pkgs/development/web/nodejs";

  buildNodejs = pkgs.callPackage "${nodeDir}/nodejs.nix" {
    openssl = pkgs.openssl;
    python = pkgs.python3;
  };

  gypPatches =
    if pkgs.stdenv.buildPlatform.isDarwin
    then ["${nodeDir}/gyp-patches-set-fallback-value-for-CLT-darwin.patch"]
    else [];
in
  buildNodejs {
    version = "24.15.0";
    sha256 = "1j51m1ild7ihyqq403dqvb7qbjl5nnij530yjanslh6ikvbm7xm4";
    patches =
      (
        if pkgs.stdenv.hostPlatform.emulatorAvailable pkgs.buildPackages
        then ["${nodeDir}/configure-emulator.patch"]
        else [
          (pkgs.fetchpatch2 {
            url = "https://raw.githubusercontent.com/buildroot/buildroot/2f0c31bffdb59fb224387e35134a6d5e09a81d57/package/nodejs/nodejs-src/0003-include-obj-name-in-shared-intermediate.patch";
            hash = "sha256-3g4aS+NmmUYNOYRNc6UMJKYoaTlpP5Knt9UHegx+o0Y=";
          })
        ]
      )
      ++ pkgs.lib.optionals
        (pkgs.stdenv.hostPlatform != pkgs.stdenv.buildPlatform && pkgs.stdenv.hostPlatform.isFreeBSD)
        [
          (pkgs.fetchpatch2 {
            url = "https://raw.githubusercontent.com/rubyjs/libv8-node/62476a398d4c9c1a670240a3b070d69544be3761/patch/v8-no-assert-trivially-copyable.patch";
            hash = "sha256-hSTLljmVzYmc3WAVeRq9EPYluXGXFeWVXkykufGQPVw=";
          })
        ]
      ++ [
        "${nodeDir}/configure-armv6-vfpv2.patch"
        "${nodeDir}/node-npm-build-npm-package-logic.patch"
        "${nodeDir}/use-correct-env-in-tests.patch"
        "${nodeDir}/bin-sh-node-run-v22.patch"
        "${nodeDir}/use-nix-codesign.patch"
      ]
      ++ gypPatches
      ++ pkgs.lib.optionals (!pkgs.stdenv.buildPlatform.isDarwin) [
        (pkgs.fetchpatch2 {
          url = "https://github.com/nodejs/node/commit/869d0cbca3b0b5e594b3254869a34d549664e089.patch?full_index=1";
          hash = "sha256-BBBShQwU20TSY8GtPehQ9i3AH4ZKUGIr8O0bRsgrpNo=";
          revert = true;
        })
      ]
      ++ pkgs.lib.optionals pkgs.stdenv.is32bit [
        "${nodeDir}/v24-32bit.patch"
        "${nodeDir}/sab-test-32bit.patch"
      ];
  }
