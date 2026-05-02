{
  description = "Warcraft III Hotkey Editor — web-based CustomKeys.txt editor";

  nixConfig = {
    extra-substituters = ["https://clemenscodes.cachix.org"];
    extra-trusted-public-keys = [
      "clemenscodes.cachix.org-1:yEwW1YgttL2xdsyfFDz/vv8zZRhRGMeDQsKKmtV1N18="
    ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    crane,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
            # `dioxus-cli` 0.7.6 strictly checks the wasm-bindgen-cli
            # version against the wasm-bindgen library (transitively
            # pinned to 0.2.114 by `dioxus = =0.7.6`). nixpkgs ships a
            # newer 0.2.117, so we pin our own at 0.2.114 via the
            # in-tree builder.
            (final: prev: {
              wasm-bindgen-cli = final.buildWasmBindgenCli rec {
                src = final.fetchCrate {
                  pname = "wasm-bindgen-cli";
                  version = "0.2.114";
                  hash = "sha256-xrCym+rFY6EUQFWyWl6OPA+LtftpUAE5pIaElAIVqW0=";
                };
                cargoDeps = final.rustPlatform.fetchCargoVendor {
                  inherit src;
                  inherit (src) pname version;
                  hash = "sha256-Z8+dUXPQq7S+Q7DWNr2Y9d8GMuEdSnq00quUR0wDNPM=";
                };
              };
              # nixpkgs ships dioxus-cli 0.7.5 — bump to 0.7.6 to match
              # the workspace pin. Same `no-downloads` + `disable-telemetry`
              # build features that nixpkgs already configures.
              dioxus-cli = prev.dioxus-cli.overrideAttrs (old: rec {
                version = "0.7.6";
                src = final.fetchCrate {
                  pname = "dioxus-cli";
                  inherit version;
                  hash = "sha256-PKidohK85wv/ZN9WcNS+HTlVGgR5o07gWLshZhzyg5k=";
                };
                cargoDeps = final.rustPlatform.fetchCargoVendor {
                  inherit src;
                  inherit (src) pname version;
                  hash = "sha256-T6xLlu8XeJPm+ULgpTALTT93X55ExJhDMuhpal2QLhg=";
                };
              });
            })
          ];
        };

        # Stable Rust with the WASM target the Dioxus build needs.
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = ["wasm32-unknown-unknown"];
          extensions = ["rust-src" "rust-analyzer" "clippy" "rustfmt"];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # nixpkgs's `moon` 1.41.8 currently fails to build on
        # nixos-unstable, so we vendor the upstream 2.0.3 release the
        # same way the private warcraft-vk-overlay repo does.
        moonCli = pkgs.rustPlatform.buildRustPackage (finalAttrs: {
          pname = "moon";
          version = "2.0.3";

          src = pkgs.fetchFromGitHub {
            owner = "moonrepo";
            repo = "moon";
            tag = "v${finalAttrs.version}";
            hash = "sha256-I19rScY2cZYuFaVUYTO5XEb6s/GWaC+TFgSmhYn9Egw=";
          };

          cargoHash = "sha256-ciaXO47SYCkDDJO0k4Jk7D7z9/rQ+UxBsXyBlD5zTtc=";

          env = {
            RUSTFLAGS = "-C strip=symbols";
            OPENSSL_NO_VENDOR = 1;
          };

          buildInputs = [pkgs.openssl];
          nativeBuildInputs = with pkgs; [
            pkg-config
            installShellFiles
            writableTmpDirAsHomeHook
          ];

          postInstall =
            pkgs.lib.optionalString
            (pkgs.stdenv.hostPlatform.emulatorAvailable pkgs.buildPackages)
            (
              let
                emulator = pkgs.stdenv.hostPlatform.emulator pkgs.buildPackages;
              in ''
                installShellCompletion --cmd moon \
                  --bash <(${emulator} $out/bin/moon completions --shell bash) \
                  --fish <(${emulator} $out/bin/moon completions --shell fish) \
                  --zsh <(${emulator} $out/bin/moon completions --shell zsh)
              ''
            );

          doCheck = false;
          doInstallCheck = true;
          nativeInstallCheckInputs = [pkgs.versionCheckHook];

          meta = {
            description = "Task runner and repo management tool for the web ecosystem, written in Rust";
            mainProgram = "moon";
            homepage = "https://github.com/moonrepo/moon";
            license = pkgs.lib.licenses.mit;
          };
        });

        # Tools every moon task needs on $PATH at runtime. Anything
        # `.moon/tasks.yml` or a per-crate `moon.yml` shells out to has
        # to be in here, otherwise `nix run .#dev` and friends crash
        # with "command not found".
        moonRuntimeInputs = [
          rustToolchain
          moonCli
          pkgs.dioxus-cli
          pkgs.wasm-bindgen-cli
          pkgs.tailwindcss_4
          pkgs.esbuild
          pkgs.binaryen
        ];

        # Wraps `moon run :<task>` (workspace-default project, which is
        # `hotkey-editor` per `.moon/workspace.yml`) in a shell app so
        # we can expose it as `nix run .#<task>` — no need to enter the
        # devshell first.
        runMoonTask = task:
          pkgs.writeShellApplication {
            name = "moon-${task}";
            runtimeInputs = moonRuntimeInputs;
            text = ''
              exec moon run ":${task}" "$@"
            '';
          };

        # The build needs more than `cargo` sees: the static asset trees
        # under `crates/hotkey-editor/{assets,public}` are inlined by
        # Dioxus's `asset!()` macro and `include_str!` pulls in the
        # baseline CustomKeys.txt at the repo root.
        src = pkgs.lib.fileset.toSource {
          root = ./.;
          fileset = pkgs.lib.fileset.unions [
            (craneLib.fileset.commonCargoSources ./.)
            ./crates/hotkey-editor/Dioxus.toml
            ./crates/hotkey-editor/tailwind.input.css
            ./crates/hotkey-editor/styles
            ./crates/hotkey-editor/scripts
            ./crates/hotkey-editor/assets
            ./crates/hotkey-editor/public
            ./crates/hotkey-editor/templates
          ];
        };

        commonArgs = {
          inherit src;
          pname = "warcraft-hotkey-editor";
          version = "0.1.0";
          strictDeps = true;
          doCheck = false;
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
          # `dioxus-cli` in nixpkgs is built with the `no-downloads`
          # feature, so it expects `wasm-bindgen-cli` and `wasm-opt`
          # (binaryen) to already be on $PATH at build time.
          nativeBuildInputs = with pkgs; [
            dioxus-cli
            wasm-bindgen-cli
            tailwindcss_4
            esbuild
            binaryen
          ];
        };

        # Cache cargo dependencies separately so a code-only change
        # doesn't re-download the world.
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # The final static bundle: index.html + hashed JS/WASM + assets +
        # `.nojekyll` and `404.html` for GitHub Pages compatibility.
        # The output directory is exactly what `actions/upload-pages-artifact`
        # wants — no post-processing needed in CI.
        warcraft-hotkey-editor = craneLib.mkCargoDerivation (commonArgs
          // {
            inherit cargoArtifacts;
            pnameSuffix = "-bundle";
            buildPhaseCargoCommand = ''
              cd crates/hotkey-editor
              tailwindcss -i tailwind.input.css -o assets/tailwind.css --minify
              esbuild scripts/keyboard-navigation.ts \
                --bundle --format=esm --target=es2022 --minify \
                --outfile=assets/keyboard-navigation.js
              dx build --release --platform web --offline
            '';
            installPhaseCommand = ''
              mkdir -p $out
              cp -r ../../target/dx/hotkey-editor/release/web/public/. $out/

              # GitHub Pages runs Jekyll by default and silently strips
              # any path beginning with `_` (interpreted as a partial).
              # `.nojekyll` opts the site out of Jekyll entirely so dx's
              # hashed filenames and any `_`-prefixed directories survive.
              touch $out/.nojekyll

              # Deep-link fallback for the Dioxus client-side router:
              # when GH Pages can't find an exact file match it serves
              # `404.html`, which here is just a copy of `index.html`.
              # The SPA boots, reads the requested path, and routes
              # client-side — `/warcraft-hotkey-editor/templates`,
              # `/warcraft-hotkey-editor/Hamg`, etc. all "just work"
              # without server-side rewrites.
              cp $out/index.html $out/404.html
            '';
          });
      in {
        formatter = pkgs.alejandra;

        packages = {
          default = warcraft-hotkey-editor;
          inherit warcraft-hotkey-editor;
        };

        # `nix run .#dev` and `nix run .#bundle` are the same thing as
        # `moon run :dev` / `moon run :bundle` — exposed at the flake
        # layer so first-time contributors don't need to learn moon
        # before they can serve or build.
        apps = {
          dev = {
            type = "app";
            program = "${runMoonTask "dev"}/bin/moon-dev";
          };
          bundle = {
            type = "app";
            program = "${runMoonTask "bundle"}/bin/moon-bundle";
          };
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [warcraft-hotkey-editor];
          packages =
            moonRuntimeInputs
            ++ (with pkgs; [
              cargo-watch
              cargo-edit
              taplo
              alejandra
              nil
            ]);
          shellHook = ''
            echo ""
            echo "  Warcraft III Hotkey Editor — dev shell"
            echo ""
            echo "  Tasks (moon-driven):"
            echo "    moon run :dev        — Tailwind watcher + dx serve"
            echo "    moon run :bundle     — production build via dx"
            echo "    moon run :ci         — fmt + lint + test + build"
            echo ""
            echo "  Or skip the shell entirely:"
            echo "    nix run .#dev        — same as moon run :dev"
            echo "    nix run .#bundle     — same as moon run :bundle"
            echo "    nix build .#warcraft-hotkey-editor   — fully reproducible"
            echo ""
          '';
        };
      }
    );
}
