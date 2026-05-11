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
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    playwright.url = "github:pietdevries94/playwright-web-flake";
    moon-tui.url = "github:clemenscodes/moon-tui";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
    crane,
    playwright,
    moon-tui,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            # `dioxus-cli` 0.7.9 strictly checks the wasm-bindgen-cli
            # version against the wasm-bindgen library (transitively
            # resolved to 0.2.121 by `dioxus = =0.7.9`). nixpkgs ships
            # 0.2.117, so we pin our own at 0.2.121 via the in-tree builder.
            (final: prev: {
              wasm-bindgen-cli = final.buildWasmBindgenCli rec {
                src = final.fetchCrate {
                  pname = "wasm-bindgen-cli";
                  version = "0.2.121";
                  hash = "sha256-ZOMgFNOcGkO66Jz/Z83eoIu+DIzo3Z/vq6Z5g6BDY/w=";
                };
                cargoDeps = final.rustPlatform.fetchCargoVendor {
                  inherit src;
                  inherit (src) pname version;
                  hash = "sha256-DPdCDPTAPBrbqLUqnCwQu1dePs9lGg85JCJOCIr9qjU=";
                };
              };
              # nixpkgs ships dioxus-cli 0.7.5 — bump to 0.7.9 to match
              # the workspace pin. Same `no-downloads` + `disable-telemetry`
              # build features that nixpkgs already configures.
              dioxus-cli = prev.dioxus-cli.overrideAttrs (old: rec {
                version = "0.7.9";
                src = final.fetchCrate {
                  pname = "dioxus-cli";
                  inherit version;
                  hash = "sha256-tLMtUlohSJt3okdJh+ARweQNGmzj/vYiNl8iZhDbSAc=";
                };
                cargoDeps = final.rustPlatform.fetchCargoVendor {
                  inherit src;
                  inherit (src) pname version;
                  hash = "sha256-h5wkxHP8ehZLHqcUsro08/dpqSPnPuBbZuUGG8i4nBc=";
                };
              });

              # Pinned upstream CascLib source. `casclib-rs`'s build script
              # builds CascLib from source via cmake; pointing it at this
              # vendored snapshot makes the extractor build reproducible
              # across machines and keeps it offline-friendly inside Nix.
              casclib = prev.fetchFromGitHub {
                owner = "ladislav-zezula";
                repo = "CascLib";
                rev = "07ab5f37ad282cc101d5c17793c550a0a6d4637f";
                hash = "sha256-E1Z4Y1i3KbMuG17M0L3xCLVVcvAGzF5NWWOadAAw3ZQ=";
              };
            })
          ];
        };

        # Rust toolchain — version, targets, and components are all
        # declared in rust-toolchain.toml; fenix reads from there.
        rustToolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-gh/xTkxKHL4eiRXzWv8KP7vfjSk61Iq48x47BEDFgfk=";
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

        # Node.js 24.14.1 and pnpm 11.0.9 — both built from pinned
        # sources so the exact versions survive future nixpkgs updates.
        # The derivation files mirror the pattern used in the unival repo.
        nodejs = import ./nix/mkNode.nix {inherit nixpkgs system;};
        pnpm = import ./nix/mkPnpm.nix {inherit nodejs; inherit (pkgs) pnpm;};

        # Tools every moon task needs on $PATH at runtime. Anything
        # `.moon/tasks.yml` or a per-crate `moon.yml` shells out to has
        # to be in here, otherwise `nix run .#dev` and friends crash
        # with "command not found".
        inherit (playwright.packages.${system}) playwright-test playwright-driver;
        moonTui = moon-tui.packages.${system}.moon-tui;
        moonRuntimeInputs = [
          rustToolchain
          moonCli
          pkgs.dioxus-cli
          pkgs.wasm-bindgen-cli
          pkgs.tailwindcss_4
          pkgs.binaryen
          pkgs.typescript
          nodejs
          pnpm
          playwright-test
          playwright-driver
          moonTui
        ];

        ci-cache-tools = pkgs.buildEnv {
          name = "warcraft-hotkey-editor-ci-cache-tools";
          paths = moonRuntimeInputs;
        };

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
            ./crates/hotkey-editor/assets
            ./crates/hotkey-editor/public
            ./crates/hotkey-editor/templates
            ./crates/warcraft-keybinds/templates
            (pkgs.lib.fileset.fileFilter (file: file.hasExt "css") ./crates/hotkey-editor/src)
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
              dx build --release --platform web --offline --package hotkey-editor
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
          inherit cargoArtifacts ci-cache-tools moonCli warcraft-hotkey-editor;
          dioxus-cli = pkgs.dioxus-cli;
          wasm-bindgen-cli = pkgs.wasm-bindgen-cli;
        };

        # `nix run .#dev` and `nix run .#bundle` are the same thing as
        # `moon run :dev` / `moon run :bundle` — exposed at the flake
        # layer so first-time contributors don't need to learn moon
        # before they can serve or build.
        apps = {
          dev = {
            type = "app";
            program = "${runMoonTask "dev"}/bin/moon-dev";
            meta = {
              description = "Start the Tailwind watcher and dx serve";
              mainProgram = "moon-dev";
            };
          };
          bundle = {
            type = "app";
            program = "${runMoonTask "bundle"}/bin/moon-bundle";
            meta = {
              description = "Build a production WASM bundle via dx";
              mainProgram = "moon-bundle";
            };
          };
          # `nix run .#extract -- --casc /path/to/Warcraft\ III/Data`
          # rebuilds crates/warcraft-database/src/db.rs from CASC. Native
          # only — needs cmake + zlib + the CascLib source pinned in the
          # overlay above. The wrapper delegates to `nix develop` so
          # build-time linker flags (zlib, libstdc++) are wired up the
          # same way they are inside the interactive shell.
          extract = let
            extractApp = pkgs.writeShellApplication {
              name = "warcraft-extract";
              runtimeInputs = [pkgs.nix];
              text = ''
                # Re-enter the project's dev shell so build-time linker
                # flags (zlib, libstdc++) are wired up exactly the way
                # they are inside `nix develop`. Resolves the flake from
                # the user's current working directory, so this works
                # whether invoked as `nix run .#extract` from the repo
                # root or `nix run /path/to/repo#extract` from anywhere.
                exec nix develop . --command \
                  cargo run -p warcraft-extractor -- "$@"
              '';
            };
          in {
            type = "app";
            program = "${extractApp}/bin/warcraft-extract";
            meta = {
              description = "Regenerate db.rs from a Warcraft III CASC archive";
              mainProgram = "warcraft-extract";
            };
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
              # Native build deps for `warcraft-extractor`: casclib-rs's
              # build.rs builds CascLib from source via cmake and links
              # against zlib. None of this is in the wasm graph, so the
              # commonArgs / wasm bundle stay untouched.
              cmake
              pkg-config
              zlib
            ]);

          # `casclib-rs`' build script reads CASCLIB_DIR to locate the
          # CascLib source tree it should compile. Pointing it at the
          # pinned overlay attribute (added above) makes the extractor
          # build reproducible across machines without any network
          # fetches at build time.
          CASCLIB_DIR = pkgs.casclib;

          PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD = "1";
          PLAYWRIGHT_BROWSERS_PATH = "${pkgs.playwright-driver.browsers}";
          MOON_TOOLCHAIN_FORCE_GLOBALS = "true";

          # Runtime linking for the extractor binary: zlib is dlopened
          # by the freshly-built CascLib, gcc.cc.lib provides libstdc++
          # for the C++ portion of CascLib.
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
            gcc.cc.lib
            zlib
          ]);

          shellHook = ''
            export NODE_PATH="${playwright-test}/lib/node_modules''${NODE_PATH:+":$NODE_PATH"}"
            echo ""
            echo "  Warcraft III Hotkey Editor — dev shell"
            echo ""
            echo "  Web app (wasm):"
            echo "    moon run :dev        — Tailwind watcher + dx serve"
            echo "    moon run :bundle     — production build via dx"
            echo "    moon run :ci         — fmt + lint + test + build"
            echo ""
            echo "    nix run .#dev        — same as moon run :dev"
            echo "    nix run .#bundle     — same as moon run :bundle"
            echo "    nix build .#warcraft-hotkey-editor   — reproducible bundle"
            echo ""
            echo "  Native data extraction (regenerates db.rs):"
            echo "    cargo run -p warcraft-extractor -- --casc \"\$W3_CASC\""
            echo "    cargo test -p warcraft-extractor"
            echo "    cargo run -p warcraft-extractor --example inspect_slk -- \"\$W3_CASC\" units/abilitydata.slk"
            echo ""
            echo "  Set W3_CASC to your Warcraft III install's Data/ dir."
            echo "  Typical Wine layout (auto-detected if present):"
            echo "    \$WINEPREFIX/drive_c/Program Files (x86)/Warcraft III/Data"
            echo ""

            # Convenience: if W3_CASC is unset and a likely Wine install
            # is on disk, point at it. Mirrors vk-overlay's discovery
            # behavior without hardcoding a specific WINEPREFIX layout.
            if [ -z "''${W3_CASC:-}" ]; then
              for candidate in \
                "''${WINEPREFIX:-$HOME/.wine}/drive_c/Program Files (x86)/Warcraft III/Data" \
                "$HOME/Games/W3Champions/drive_c/Program Files (x86)/Warcraft III/Data" \
                "$HOME/.wine/drive_c/Program Files (x86)/Warcraft III/Data" \
                "$HOME/Library/Application Support/Blizzard/Warcraft III/Data"; do
                if [ -d "$candidate" ]; then
                  export W3_CASC="$candidate"
                  echo "  W3_CASC auto-detected: $W3_CASC"
                  echo ""
                  break
                fi
              done
            else
              echo "  W3_CASC=$W3_CASC"
              echo ""
            fi
          '';
        };
      }
    );
}
