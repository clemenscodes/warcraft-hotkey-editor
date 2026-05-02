# Architecture

Warcraft III Hotkey Editor is a static Dioxus web app. The browser loads
the full object database, keeps the edited `CustomKeys.txt` state in
client-side signals, and downloads the final file without talking to a
server.

## Workspace Crates

| Crate | Role |
| --- | --- |
| `hotkey-editor` | Dioxus app, UI components, browser import/export glue, focus handling, and command-card editing logic. |
| `warcraft-api` | Shared Warcraft III value types and object metadata used by the database and app. |
| `warcraft-database` | Generated Rust database extracted from Warcraft III data files. Treat `src/db.rs` as generated output. |
| `warcraft-keybinds` | Parser, in-memory model, and serializer for `CustomKeys.txt`. |

## Runtime Flow

The app starts with the bundled baseline `CustomKeys.txt` template. A user
can replace it by importing a local file; imported values are parsed by
`warcraft-keybinds` and overlaid onto the baseline so missing sections
still have complete game defaults.

Most UI state lives in `hotkey-editor/src/app.rs` as Dioxus signals:
selected race, mode, unit, selected command-card slot, active grid
layout, import status, and dialog visibility. Components receive those
signals directly and delegate non-visual decisions into `domain/`.

The central editing path is:

1. `unit_catalog` and `command_catalog` pick the objects and command-card
   rows to show from `WARCRAFT_DATABASE`.
2. `positions` resolves current button positions by combining defaults,
   explicit `CustomKeys.txt` overrides, generated grid layouts, and
   special WC3 cases such as build menus and uprooted buildings.
3. `hotkey_override` writes user edits back into the mutable
   `CustomKeysFile`.
4. `explicit_export` materializes implicit defaults and serializes a full
   `CustomKeys.txt` for download.

## Frontend Structure

`components/` contains the Dioxus UI. Larger components are split around
user-visible surfaces: header, unit list, unit detail, command grid,
override panel, preview dialog, templates dialog, and system-hotkeys
dialog.

`domain/` contains app-specific rules that are easier to test and reason
about outside RSX markup: races, unit modes, object lookup, grid slots,
grid templates, resolved positions, display labels, and icon URLs.

`customkeys/` contains browser-facing import/export helpers and the
baseline/template overlay logic.

`text/` normalizes Warcraft III tooltip text by handling color codes,
placeholder substitution, level markers, and command labels.

`focus/` and `scripts/keyboard-navigation.ts` handle keyboard modality and
desktop spatial navigation. This lives partly in TypeScript because some
focus events need capture-phase DOM handling outside Dioxus' delegated
event system.

## Assets

There are two asset paths on purpose:

- `assets/` is processed by Dioxus `asset!()` calls. It contains app
  assets, generated CSS/JS, and UI textures referenced from Rust.
- `public/` is copied as stable static files. It contains files that the
  app addresses by URL at runtime, such as command-button icons, favicons,
  Open Graph images, and fonts.

`Dioxus.toml` sets `base_path = "warcraft-hotkey-editor"` for GitHub
Pages. Any hand-built public URL must include the same path prefix.

The Tailwind output is intentionally ignored:

- `crates/hotkey-editor/assets/tailwind.css`

The TypeScript bundle at
`crates/hotkey-editor/assets/keyboard-navigation.js` is generated from
`scripts/keyboard-navigation.ts` and versioned as the browser asset that
Dioxus references. The Moon `dev` and `bundle` tasks rebuild both assets
before `dx serve` or `dx build` runs. The Nix package runs the same
Tailwind and esbuild steps inside the derivation.

## Build And Release

Moon is the local task runner:

```bash
moon run :dev
moon run :bundle
moon run :ci
```

Nix is the reproducible release path:

```bash
nix build .#warcraft-hotkey-editor
```

The GitHub Actions workflow builds that flake package and uploads the
resulting static bundle to GitHub Pages.
