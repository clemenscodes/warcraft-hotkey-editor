# tw_macro — keyed typed-Tailwind extraction

**Date:** 2026-07-06
**Status:** Design approved, spec under review
**Scope:** Extract the app-agnostic styling subsystem into a reusable crate `tw_macro`, redesigned around a keyed, vocabulary-parameterized macro API. First cut of a larger "reusable Dioxus meta-framework" extraction effort; the other candidates (LocalStorage/codecs, toasts, file/focus helpers, route-sync) are explicitly out of scope here.

## Goal

Today the responsive-styling engine lives in `crates/hotkey-editor/src/styling/` (`tailwind_class.rs`, `class_list.rs`, `compile_checks.rs`, `macros.rs`, `mod.rs`, ~300 LOC of pure `const fn` + `macro_rules!`). It is almost entirely app-agnostic — the only coupling is a **hardcoded six-band vocabulary** (`mobile|tablet|laptop|desktop|qhd|uhd`) baked into the macros and const-fns. Theme tokens already live outside it, in `tailwind.input.css`.

Extract it into a standalone crate `crates/tw_macro` that:

1. Is reusable across future Dioxus (and, in principle, other-framework) projects.
2. Lets each consumer **declare its own responsive band vocabulary** instead of the hardcoded six.
3. Offers a **keyed authoring API** — inline literal lists per band, unused bands simply omitted — the ergonomic payoff, in place of the current seven mandatory positional slices.
4. **Preserves full compile-time enforcement**, including the parts that provably require a declared vocabulary (see Enforcement).
5. Thins `hotkey-editor` by removing the engine and the per-`style.rs` band-const boilerplate.

The crate stays **in the same repo** for now (a workspace member); publishing is a later concern.

## Constraints (project rules that bound this work)

- `tw_macro` must build and unit-test on native (`cargo test`); the browser dependency (`dioxus`) is confined to one file behind a feature.
- Rust style rules (`docs/RUST_STYLE.md`) apply to all new code: full semantic names, no tuples, named-struct fields, `Self` in impls, derive everything applicable, no `as` casts outside `From`/`TryFrom`, etc. Note: `compile_checks.rs` currently returns a tuple from `directory_bounds`/`last_segment_bounds` — the extraction is the moment to replace those with a named struct.
- Component/`style.rs` conventions (`docs/COMPONENTS.md`) are updated as part of this change, since the authoring shape changes.

## Crate structure

A **single** crate, `crates/tw_macro`. No proc-macro, no crate split, **zero external dependencies** except an optional `dioxus`.

```
crates/tw_macro/
  Cargo.toml            # [features] dioxus = ["dep:dioxus"]; dioxus optional, NOT default
  src/
    lib.rs              # re-exports: TailwindClass, ClassList, tw!, assert_component!, define_styling!
    tailwind_class.rs   # TailwindClass (named-field newtype over &'static str) — pure
    class_list.rs       # ClassList (opaque &'static str); IntoAttributeValue impl + to_library_class
                        #   are #[cfg(feature = "dioxus")]
    internal.rs         # #[doc(hidden)] pub mod __internal: all const fn validators/joiners
    macros.rs           # tw!, assert_component! (fixed); define_styling! (generator)
```

- **`TailwindClass`, `ClassList`** — moved verbatim (modulo style fixes). `ClassList` keeps its opacity guarantee (only `IntoAttributeValue`, no `Display`/accessor) so class strings can never be interpolated or appended.
- **`__internal`** — the const-fn machinery (`assert_base`, `assert_band`, `assert_flat`, `assert_declared_band`, `identity_len`, `build_identity`, `joined_len`, `join_into`, plus a named `DirectoryBounds { start, end }` struct replacing the current tuples). `pub` so generated macros can call it, `#[doc(hidden)]` so it is not public surface.
- **`dioxus` feature** — off by default. Only `class_list.rs`'s `IntoAttributeValue` impl and the `to_library_class` bridge (for `dioxus_primitives::DialogContent`) depend on it. Non-dioxus consumers get the pure engine. `hotkey-editor` enables `features = ["dioxus"]`.

### `hotkey-editor` integration

- Depends on `tw_macro` with `features = ["dioxus"]`.
- Invokes `tw_macro::define_styling! { bands: [mobile, tablet, laptop, desktop, qhd, uhd] }` **once** at crate root, generating crate-global `classes!`/`states!` bound to those six bands.
- A thin re-export keeps existing `use crate::{tw, classes, styling::{TailwindClass, ClassList}, ...}` import lines compiling (module `styling` re-exports the two types; `tw!`/`assert_component!`/generated `classes!`/`states!` are crate-global). Only the macro **invocation bodies** change, not the import lines (and even those shrink where band-consts disappear).

## Public API

### Fixed macros (vocabulary-independent, exported directly)

- `tw!["a", "b"]` → `&[TailwindClass]`. Unchanged. Retained for standalone class arrays.
- `assert_component!(PascalName)` → compile guard that the component fn name equals its directory (`component == directory == class`). Unchanged.

### Generator macro

- `define_styling! { bands: [<ident>, <ident>, ...] }` — invoked once per consumer crate. Emits:
  - `const __BANDS: &[&str] = &[stringify!(<ident>), ...];` — the baked, closed vocabulary.
  - `#[macro_export] macro_rules! classes { ... }` and `#[macro_export] macro_rules! states { ... }`, both bound to `__BANDS`.

  Mechanism: `macro_rules!` generating `macro_rules!` on stable via the **dollar-passing trick** — `define_styling!` expands to a helper macro that matches a literal `$` token as `$dol:tt`, then uses `$dol` to write the inner macros' metavariables. The band idents are interpolated as literal data (into `__BANDS` and the canonical join order), not re-parameterized. This is the one intentionally-hairy construct in the crate; it is covered by both success and `compile_fail` tests.

### Generated: keyed `classes!`

```rust
classes! {
    base:   ["@container", "relative", "z-50", "after:content-['']", "after:absolute"],
    mobile: ["mobile:flex", "mobile:flex-row"],
    laptop: ["laptop:grid", "laptop:items-stretch"],
    // tablet/desktop/qhd/uhd omitted — an unused band is simply absent
}
```

- Grammar: `$key:ident : [ $($class:literal),* $(,)? ]`, entries comma-separated, **order-independent**, all optional, duplicate keys rejected at compile time.
- Reserved key `base` → always-on band (`assert_base`).
- Any other key must be a **declared band** (`assert_declared_band(key, __BANDS)`); each class must carry the `key:` prefix (`assert_band(key, ...)`).
- The kebab **identity** derived from `module_path!()` (directory, `_`→`-`) is always prepended.
- Emits `pub(super) const CLASS: ClassList` and an internal `CLASS_STR` (consumed by `states!`).
- **Join order:** the joined string is deterministic and independent of call-site key order: identity first, then `base`, then each declared band in `__BANDS` vocabulary order, skipping absent keys. This matches the old positional order (`identity, BASE, MOBILE, TABLET, …`). The migration script writes keys in this same canonical order, so migrated output is byte-identical to the old output.

### Generated: keyed `states!`

```rust
states! {
    TileState,
    Idle       => [],
    DragSource => ["opacity-40", "ring-2", "ring-warcraft-gold"],
}
```

- Grammar unchanged in spirit: a state enum `$ty`, then `Variant => [ $($class:literal),* ]` arms, exhaustive.
- Each overlay is validated **flat** (`assert_flat` against `__BANDS`: no declared band prefix).
- Emits `pub(super) fn class(state: $ty) -> ClassList`, each arm a precomputed `ClassList` (base `CLASS_STR` + overlay).

### Dropped

- `assert_named` (the "a band const must be named exactly BASE/MOBILE/…" guard) is **removed** — there are no band consts anymore; the keyed key *is* the band. This was the app-specific naming coupling.

## Enforcement (the reason a declared vocabulary is required)

Per-band, non-`base` enforcement is **local** to the key and needs no vocabulary. `base`, `states!` overlays, and typo-band-key detection **provably require** the declared band set, because `base`/overlays legitimately contain non-band variant prefixes (`after:`, `hover:`) and must reject only *band* prefixes.

| Author writes | Result |
|---|---|
| `mobile: ["mobile:flex"]` | ok |
| `mobile: ["flex"]` | compile error — missing `mobile:` prefix |
| `mobile: ["tablet:flex"]` | compile error — not `mobile:`-prefixed (caught locally) |
| `base: ["after:absolute", "flex"]` | ok — `after:` is a variant, not a band |
| `base: ["mobile:flex"]` | compile error — a declared band prefix in `base` |
| `moble: ["moble:flex"]` | compile error — `moble` not in the declared vocabulary |
| overlay `["mobile:opacity-40"]` in `states!` | compile error — band prefix in a flat overlay |

## Migration of the ~337 `style.rs`

Big-bang, script-driven. The source shape is rigid and uniform (every file: `const BASE/MOBILE/…/UHD: &[TailwindClass] = tw![…];` blocks + one `classes!{ … }`, optionally `states!{ … }` over local overlay consts), which makes a targeted transform reliable.

Transform per file:
1. Parse the band consts feeding `classes!`; emit keyed inline `classes!` in **canonical vocabulary order**, **omitting** empty (`tw![]`) bands. `BASE`→`base`, `MOBILE`→`mobile`, etc.
2. Parse `states!` overlay consts; inline each `Variant => CONST` as `Variant => [literals]`.
3. Remove now-unused `tw!` / `TailwindClass` imports (leave `tw!` where still used for standalone arrays).
4. Leave unrelated `tw!` consts and non-style files untouched.

One-time: add `define_styling! { bands: [...] }` at `hotkey-editor` crate root; wire the `tw_macro` dependency and re-export shim; update `docs/COMPONENTS.md` and `docs/RUST_STYLE.md` (the positional-seven-band role model becomes the keyed model).

### Verification

- **Byte-identical output:** before the rewrite, snapshot every component's `CLASS` string (and every `states!` arm's string) to a golden file. After the rewrite, assert unchanged. Because the script emits canonical order, output must be byte-for-byte identical — the strongest possible "the migration changed nothing" check. This golden snapshot/test is **temporary** — removed once CI is green, not checked in permanently.
- `moon run :ci` green: clippy, fmt, native tests, and the Playwright e2e gate.
- Because `ClassList` is compile-time-built, any mis-migrated file fails to compile immediately.

## Implementation order (de-risk first)

1. **Proof-of-concept:** scaffold `crates/tw_macro`; move `TailwindClass`/`ClassList`/const-fns; implement `define_styling!` + generated keyed `classes!`/`states!`; prove it on 1–2 hand-migrated components. Add `compile_fail` tests for every row of the Enforcement table. **Do not touch the other ~335 files until this is solid.**
2. **Migration script:** write + dry-run the transform; diff a sample; add the golden snapshot.
3. **Big-bang migrate**, run the golden check + `moon run :ci`.
4. **Docs + cleanup:** update `COMPONENTS.md`/`RUST_STYLE.md`; remove the temporary golden test; delete the old `hotkey-editor/src/styling/` engine (leaving only the re-export shim).

## Out of scope

- The other extraction candidates (LocalStorage/codecs, toasts, overlay-state, file/focus helpers, route-sync) — separate later cuts.
- A closed vocabulary beyond a flat list of band prefixes (no per-band metadata, no breakpoint widths in Rust — those stay in `tailwind.input.css`).
- The `module_path!()`-derived identity and `component == directory == class` convention — unchanged.
- Publishing to crates.io.

## Risks

- **The nested-macro generator** is the main technical risk; mitigated by building it first in isolation with compile-fail coverage before any mass migration.
- **Migration-script edge cases** (a `style.rs` that deviates from the uniform shape) — surfaced immediately as compile errors; the golden byte-identical check catches any silent semantic drift.
- **Band join order** — resolved by canonical vocabulary-order emission; class-attribute order is semantically irrelevant to Tailwind anyway, but byte-identity gives a clean verification signal.
