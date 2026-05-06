# Warcraft III Hotkey Editor

A web-based editor for Warcraft III: Reforged `CustomKeys.txt` files.
Pick a unit, inspect its command card, drag commands onto new slots,
change hotkeys, and export a file the game can load.

**Live:** <https://clemenscodes.github.io/warcraft-hotkey-editor/>

The app is built with [Dioxus](https://dioxuslabs.com), compiles to
WebAssembly, and runs entirely in the browser. There is no server,
account system, telemetry, or file upload to a backend.

## What It Does

- Inspect units, heroes, buildings, abilities, items, upgrades, and
  system hotkeys from the bundled Warcraft III: Reforged data.
- Edit command-card positions visually on the same 4x3 grid Warcraft III
  uses in-game.
- Override hotkeys and positions for unit commands, hero abilities,
  research levels, build menus, inventory slots, control groups, and hero
  selection.
- Highlight hotkey conflicts on the currently selected unit.
- Start from a built-in QWERTY, QWERTZ (by Neo), or default Warcraft III
  template.
- Export a complete `CustomKeys.txt` file that can be copied into the
  Warcraft III custom-keybindings folder.

## Screenshots

### Editor Overview

![Editor overview with race tabs, unit list, Archmage details, and command-card editing.](docs/screenshots/editor-overview.png)

### Layout Templates

![Layout templates dialog showing Neo QWERTY, Neo QWERTZ, and default Warcraft III templates.](docs/screenshots/layout-templates.png)

### System Hotkeys

![System hotkeys dialog with the full keyboard picker open.](docs/screenshots/system-hotkeys.png)

### Download Flow

![Download dialog explaining where to place CustomKeys.txt.](docs/screenshots/download-customkeys.png)

## Using The Export

1. Open the live editor.
2. Import an existing `CustomKeys.txt`, or start from one of the bundled
   templates.
3. Edit the command cards and system hotkeys.
4. Download `CustomKeys.txt`.
5. Put it in Warcraft III's custom-keybindings directory:

```text
Documents/Warcraft III/CustomKeyBindings/CustomKeys.txt
```

Then enable custom hotkeys in the Warcraft III options menu.

## Local Development

The supported development path is **Nix**. It provides the pinned Rust,
Dioxus CLI, Tailwind CSS, esbuild, binaryen, and wasm-bindgen versions
used by CI and GitHub Pages.

```bash
# Dev server: builds generated assets first, then runs dx serve
nix run .#dev

# Production bundle through the same moon task CI uses
nix run .#bundle

# Fully reproducible static bundle in ./result/
nix build .#warcraft-hotkey-editor
```

For an interactive shell:

```bash
nix develop

moon run :dev        # Tailwind/esbuild + dx serve
moon run :bundle     # production dx build
moon run :ci         # fmt + clippy + tests + wasm build
moon run :nix/bundle # nix build .#warcraft-hotkey-editor
```

The dev server usually prints:

```text
http://127.0.0.1:8080/warcraft-hotkey-editor/
```

### Without Nix

This is unsupported. You need matching local versions of:

- Rust with the `wasm32-unknown-unknown` target
- `dioxus-cli` `0.7.6`
- `wasm-bindgen-cli` `0.2.114`
- Tailwind CSS v4
- esbuild
- binaryen / `wasm-opt`

Version mismatches can fail late in the WASM build, so prefer Nix unless
you are deliberately debugging the toolchain.

## Repository Layout

```text
crates/
├── hotkey-editor/        # Dioxus web app (wasm)
├── warcraft-api/         # Shared Warcraft III data types
├── warcraft-database/    # Pre-extracted Warcraft III object database
├── warcraft-extractor/   # Native CLI: regenerates db.rs from CASC
├── warcraft-keybinds/    # CustomKeys.txt parser and serializer
└── warcraft-slk/         # SLK table parser used by warcraft-extractor

docs/
├── ARCHITECTURE.md       # Runtime and build architecture
└── EXTRACTION.md         # How db.rs is generated and regenerated
```

Generated frontend assets such as
`crates/hotkey-editor/assets/tailwind.css` are not committed. The dev,
bundle, and Nix build paths generate them before Dioxus reads them.

## Generated Data

`crates/warcraft-database/src/db.rs` is **machine-generated** by
`warcraft-extractor` from a Warcraft III: Reforged CASC install. Hand
edits are wiped on the next regeneration. To refresh it after a patch
or fix an extraction bug, see [`docs/EXTRACTION.md`](docs/EXTRACTION.md).

The extractor is native-only (CASC + cmake + zlib) and is intentionally
kept out of the default workspace operations so the wasm build remains
clean. Run it explicitly:

```bash
nix develop --command cargo run -p warcraft-extractor -- --casc "$W3_CASC"
```

## Release Checklist

Before pushing a launch build:

```bash
moon run :ci
nix build .#warcraft-hotkey-editor
```

The GitHub Pages workflow builds the same Nix package and deploys the
resulting static files from `./result`.

## License

Source code: **AGPL-3.0-only**. See [`LICENSE`](LICENSE).

Bundled Warcraft III assets and game data: **property of Blizzard
Entertainment**. See [`DISCLAIMER.md`](DISCLAIMER.md).
