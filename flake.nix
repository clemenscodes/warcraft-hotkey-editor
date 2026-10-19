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
    playwright.url = "github:pietdevries94/playwright-web-flake";
    moon-tui.url = "github:clemenscodes/moon-tui";
    tw-lint = {
      url = "github:clemenscodes/tw-lint";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    crane,
    playwright,
    moon-tui,
    tw-lint,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            # oxalica's rust-overlay exposes `pkgs.rust-bin`, from which we
            # build the toolchain declared in rust-toolchain.toml. It ships
            # pre-generated version manifests, so this stays IFD-free — no
            # toolchain hash to pin or churn.
            (import rust-overlay)
          ];
        };

        # Rust toolchain — version, targets, and components are all
        # declared in rust-toolchain.toml; rust-overlay reads from there.
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # nixpkgs ships moon, dioxus-cli and wasm-bindgen-cli at the versions
        # the workspace pins (dioxus 0.7.10 checks wasm-bindgen-cli against the
        # 0.2.129 library strictly), so they come prebuilt from cache.nixos.org.
        moonCli = pkgs.moon;

        # MOON_TOOLCHAIN_FORCE_GLOBALS=true bypasses Moon's version check,
        # so any nixpkgs-provided nodejs_24/pnpm works. Building from source
        # took 2h+ in CI; the pre-built binaries from cache.nixos.org are instant.
        nodejs = pkgs.nodejs_24;
        pnpm = pkgs.pnpm;

        # Tools every moon task needs on $PATH at runtime. Anything
        # `.moon/tasks.yml` or a per-crate `moon.yml` shells out to has
        # to be in here, otherwise `nix run .#dev` and friends crash
        # with "command not found".
        inherit (playwright.packages.${system}) playwright-test playwright-driver;
        moonTui = moon-tui.packages.${system}.moon-tui;
        twLint = tw-lint.packages.${system}.default;

        # Packages needed to run moon tasks in CI and dev. moonTui is a
        # TUI wrapper for the interactive dev experience — not needed in CI.
        ciRuntimeInputs = [
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
          twLint
        ];
        moonRuntimeInputs = ciRuntimeInputs ++ [moonTui];

        ci-cache-tools = pkgs.buildEnv {
          name = "warcraft-hotkey-editor-ci-cache-tools";
          paths = ciRuntimeInputs;
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
            ./crates/hotkey-editor/tailwind.css
            ./crates/hotkey-editor/assets
            ./crates/hotkey-editor/public
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

        # Native build args: the host target. The domain crates this once had to
        # single out now come from warcraft-data, so the workspace is the editor
        # alone, which builds and tests natively as well.
        commonArgsNative = {
          inherit src;
          pname = "warcraft-hotkey-editor";
          version = "0.1.0";
          strictDeps = true;
          doCheck = false;
          cargoExtraArgs = "--workspace";
        };

        # Cache cargo dependencies separately so a code-only change
        # doesn't re-download the world.
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        cargoArtifactsNative = craneLib.buildDepsOnly commonArgsNative;

        # `asset!("/assets/tailwind.css")` requires the generated stylesheet at
        # compile time, so every derivation that compiles the editor itself
        # generates it first, exactly as the bundle does.
        generateTailwind = {
          nativeBuildInputs = [pkgs.tailwindcss_4];
          preBuild = ''
            (cd crates/hotkey-editor && tailwindcss -i tailwind.css -o assets/tailwind.css --minify)
          '';
        };

        # Nix-native check derivations — all reuse the cached artifacts above
        # so they never recompile deps. These replace the raw cargo calls that
        # used to run in the `rust-checks` CI job.
        cargoFmt = craneLib.cargoFmt {inherit src;};

        cargoClippyNative = craneLib.cargoClippy (
          commonArgsNative
          // generateTailwind
          // {
            cargoArtifacts = cargoArtifactsNative;
            cargoClippyExtraArgs = "-- -D warnings";
          }
        );

        cargoClippyWasm = craneLib.cargoClippy (
          commonArgs
          // {
            inherit cargoArtifacts;
            inherit (generateTailwind) preBuild;
            pnameSuffix = "-wasm";
            cargoExtraArgs = "-p hotkey-editor";
            cargoClippyExtraArgs = "-- -D warnings";
          }
        );

        cargoTestNative = craneLib.cargoTest (
          commonArgsNative
          // generateTailwind
          // {
            cargoArtifacts = cargoArtifactsNative;
          }
        );

        # The final static bundle: index.html + hashed JS/WASM + assets +
        # `.nojekyll` and `404.html` for GitHub Pages compatibility.
        # The output directory is exactly what `actions/upload-pages-artifact`
        # wants — no post-processing needed in CI.
        warcraft-hotkey-editor = craneLib.mkCargoDerivation (
          commonArgs
          // {
            inherit cargoArtifacts;
            pnameSuffix = "-bundle";
            buildPhaseCargoCommand = ''
              cd crates/hotkey-editor
              tailwindcss -i tailwind.css -o assets/tailwind.css --minify
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
          }
        );
      in {
        formatter = pkgs.nixfmt;

        packages = {
          default = warcraft-hotkey-editor;
          inherit
            cargoArtifacts
            cargoArtifactsNative
            ci-cache-tools
            moonCli
            warcraft-hotkey-editor
            ;
          inherit
            cargoFmt
            cargoClippyNative
            cargoClippyWasm
            cargoTestNative
            ;
          inherit (pkgs) dioxus-cli wasm-bindgen-cli;
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
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [warcraft-hotkey-editor];
          packages =
            moonRuntimeInputs
            ++ (with pkgs; [
              cargo-watch
              cargo-edit
              taplo
              nil
              hadolint
            ]);

          PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD = "1";
          # Use the pietdevries94 flake input's driver, NOT `pkgs.playwright-driver`
          # (nixpkgs): the browsers must match the `playwright-test` runner on PATH,
          # and only the flake input keeps that pair version-locked. Sourcing the
          # browsers from nixpkgs instead lets them drift out of sync on a bump.
          PLAYWRIGHT_BROWSERS_PATH = "${playwright-driver.browsers}";
          MOON_TOOLCHAIN_FORCE_GLOBALS = "true";

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
            echo "  Game data (warcraft-api/database) is pinned from"
            echo "  https://github.com/clemenscodes/warcraft-data — regenerate"
            echo "  db.rs there and bump the tag in Cargo.toml to update it."
            echo ""
          '';
        };
      }
    );
}
