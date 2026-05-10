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
  pnpm'
