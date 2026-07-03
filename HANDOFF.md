# Handoff — prop-drilling kill (Host pattern)

## Rules of engagement — read first, non-negotiable

1. **The spec is 100% clear and is law.** `docs/COMPONENTS.md`, `docs/RUST_STYLE.md`,
   `docs/ARCHITECTURE.md`, and the two `CLAUDE.md` files define everything. You have
   **no right to interpret**. Apply the spec **literally**. Do not build your own
   theory of "what is really the leaf / what the gallery shows / which role a
   component plays" over text that already says exactly what to do.
2. **No questions.** There is no ambiguity to clarify. Asking around a clear rule is
   itself the failure mode. Just do what is written.
3. **Render tree = directory tree.** The component you render in RSX is your **child**
   and nests under your own `components/`. A `<Leaf>Host` that renders `<Leaf>` puts
   `<leaf>/` at `<leaf>_host/components/<leaf>/` — **never** as a sibling directory.
   (COMPONENTS.md prose says "beside the leaf"; the authoritative rule is render-tree
   nesting. Nest it.)
4. **Painter leaves get props.** A presentational leaf takes its data as **props** and
   reads no context / no domain hook. Never strip props off a painter and force a
   provider onto it. The context read lives in the **Host** (connected wrapper).
5. **Never revert / restore the uncommitted working tree.** The whole changeset is
   in-progress and valuable. Fix forward. Never `git restore` / `git checkout HEAD --`
   / `git revert` his work.

## The task

Kill `loaded_keys` **read** prop-drilling in the header, subtree by subtree, using the
COMPONENTS.md "Presentational leaves, connected wrappers" split. An uninvolved parent
threads nothing: once every button below it reads the document itself (via its Host),
`HeaderToolbar` / `HeaderActions` / `Header` stop taking and forwarding `loaded_keys`.

## Done — ResolveButton (the reference for the rest)

Structure now in place under
`crates/hotkey-editor/src/components/shell/header/components/header_toolbar/components/`:

```
resolve_button_host/
  mod.rs            ResolveButtonHost — one hook call, renders the leaf child
  hooks.rs          use_resolve_button() -> ResolveButtonProps { disabled, onclick }
                    reads CustomKeysService + ViewNavigationContext from context
  components/
    mod.rs          pub mod resolve_button;
    resolve_button/
      mod.rs        ResolveButton(props) -> renders ToolbarButton { ..From::from(&props) }
      props.rs      ResolveButtonProps { disabled, onclick }
                    + From<&ResolveButtonProps> for ToolbarButtonProps
                      (icon ICON_RESOLVE, aria "Resolve conflicts", data_action "view-resolve")
```

- `ResolveButton` (leaf) reads **no** context — pure props → markup.
- `ResolveButtonHost` (wrapper) reads the document via `CustomKeysService.keys()`
  (`disabled = keys.read().is_none()`) and navigation, shapes the leaf props.
- `header_toolbar/mod.rs` renders `ResolveButtonHost {}` — **no `loaded_keys`** to it.
- `header_toolbar/components/mod.rs` lists `pub mod resolve_button_host;` (the leaf is
  no longer a top-level module — it's the host's child).
- Gallery: `crates/gallery/src/stories/buttons.rs::resolve_button()` renders the pure
  leaf `ResolveButton { disabled, onclick: move |_| {} }` — **no provider, no
  CustomKeysMount**. Import path is the full nested path
  `...::resolve_button_host::components::resolve_button::ResolveButton`.
- **Verified green:** `cargo clippy -p hotkey-editor -p gallery --all-targets` → exit 0.

## Per-button recipe (repeat exactly)

For each button that reads `loaded_keys` (and/or `grid_layout` / `upload_status`):

1. Create `<button>_host/` beside where the button module is registered.
2. `<button>_host/hooks.rs`: the composed hook reads `CustomKeysService` (and other
   context) and returns the leaf's `Props`.
3. `<button>_host/mod.rs`: `<Button>Host` — `pub mod components; mod hooks;`, one hook
   call, render the leaf child.
4. Move the existing leaf under `<button>_host/components/<button>/` and make it
   **presentational**: props in, no context, no domain hook. Keep its props.
5. Parent renders `<Button>Host {}` and stops passing the document to it.
6. Gallery story renders the **pure leaf** with plain values (no provider). For thin
   configurators of `ToolbarButton`, showcase via the shared `ToolbarButton` leaf.
7. Remove the now-orphaned top-level `pub mod <button>;` and fix import paths.

## Remaining

- `export_button` — reads `loaded_keys` to serialize on download; has `hooks.rs`
  (`ExportButtonModel`, a visibility guard, `From` impls) and a `DownloadInfoDialog`
  child. Split into `export_button_host/` + nested leaf.
- `upload_button` — reads `loaded_keys` + `upload_status`.
- `collisions_button` (under `header/components/collisions_button/`) — reads
  `loaded_keys` + `grid_layout`; `use_collisions_button` already exists; richer leaf
  (icon + badge children, `state.rs`, `logic.rs`). Doc's canonical example.
- Burger-drawer copies of the same buttons.
- After all header buttons read their own document: strip `loaded_keys` from
  `HeaderToolbar`, `HeaderActions`, `Header` (uninvolved parents thread nothing).

## Verify every pass (COMPONENTS.md "Verify every pass")

```
nix develop -c cargo clippy -p hotkey-editor -p gallery --target wasm32-unknown-unknown
nix develop -c cargo test -p warcraft-keybinds
nix develop -c cargo fmt --check
```

`moon run :ci` additionally runs the Playwright e2e gate. Visual verification is
available via the Playwright MCP (user runs the dev server; drive Playwright over it).

## Context you must not relearn the hard way

- `CustomKeysService` (`services/customkeys/service.rs`) is provided via context in
  `use_workbench` and is the **only** mutation path (commit re-normalizes +
  write-through). Reads: `service.keys()` -> `ReadSignal<Option<CustomKeys>>`.
- `CustomKeys` is always-normalized by construction: `from_text` (public, normalizes),
  `parse_raw` (pub(crate), raw). No public raw constructor. No `Normalized<T>`.
- The gallery **lib** is a domain-agnostic Dioxus preview framework; app/domain types
  never enter it. App-specific decorators (`stories/keys_mount.rs::CustomKeysMount`,
  like `ToastMount`) live with the stories, only for behavior stories that need a
  provider.
- A spec-review hook blocks edits and requires a visible review. Following the spec
  literally is how you pass it.
