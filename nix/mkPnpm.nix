{
  nodejs,
  pnpm,
  ...
}: let
  pnpm' = pnpm.override {
    inherit nodejs;
    version = "11.0.9";
    hash = "sha256-TYTXsOMckFT2Flh5VpgHAAfQO3I4SB4hYaViVXqpCDQ=";
  };
in
  pnpm'.overrideAttrs (_old: {
    postInstall = ''
      ln -sf ../libexec/pnpm/bin/pnpm.mjs $out/bin/pnpm
      ln -sf ../libexec/pnpm/bin/pnpx.mjs $out/bin/pnpx
    '';
  })
