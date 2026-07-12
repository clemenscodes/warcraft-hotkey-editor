# The Frame system — `Render`/`Frame` traits, headless frame primitives, app wrappers

Date: 2026-07-12
Status: trait design locked and recorded; component build + migrations pending
Supersedes: `2026-07-12-dialog-rework-design.md` (dialog-only precursor)
Related: the `COMPONENTS.md` "Frame contract" amendment + "Styling a headless
frame primitive" carve-out (applied 2026-07-12)

## What this is

A single reusable primitive for every component whose job is to wrap
caller-supplied content in fixed chrome — dialogs, cards, pages. Instead of each
one re-implementing structure, behavior, and a copied look, they all become
**frames**: a headless primitive supplies structure + behavior, one app wrapper
per frame family supplies the look once, and each concrete instance supplies only
its content regions.

The dialog rework was the seed; the real target is broader. Grounding showed the
app has **no shared content-card** — the help/templates/override/unit-detail
"cards" are ~45 independent surfaces that each re-list the same 3–4 theme tokens
(`border` + `rounded-*` + `bg-panel-*` + a `shadow-*`), and cards nest three deep.
The only genuinely shared card leaf is `conflict_panel` (2 sites). So "generalize
the cards" is a real consolidation, not a mirror of one component.

## The three layers

```
browser-kit  (agnostic Rust; depends on ddd; NO dioxus/web-sys)
  Render, Frame                          the pure contracts

dioxus-kit   (dioxus; Output = Element; app-agnostic, headless)
  Empty                                  the no-op region
  Dialog / Card / Page                   headless frame primitives (Radix-for-dioxus):
                                         structure + behavior, no look, styled via attributes

hotkey-editor (the app)
  WarcraftDialog / WarcraftCard / …      one thin wrapper per family: owns the CLASS,
                                         passes it to the primitive's parts, written once
  PreviewDialog / DownloadDialog / …     per-instance: supply a Frame (its Render regions)
  region leaves (PreviewBody, …)         the app's styled ddd components — the regions
```

External-crate reality: `browser-kit` and `dioxus-kit` are pinned git deps. The
traits and primitives land **in those repos** (edit → publish → retag → bump here),
never via a local `[patch]`. This doc is the design of record; landing it is a
multi-repo task.

## The traits (browser-kit)

```rust
/// The atomic region contract: any real ddd component, droppable into a frame region.
pub trait Render: Clone + PartialEq + Default + 'static {
    type Model: ddd::Model;   // region content is a compiler-checked ddd component
    type Output;              // the frontend's node type (Element for dioxus)
    fn render(&self) -> Self::Output;
}

/// A frame arranges three Render regions; body required, header/footer optional.
pub trait Frame: Clone + PartialEq + Default + 'static {
    type Output;
    type Header: Render<Output = Self::Output>;
    type Body:   Render<Output = Self::Output>;
    type Footer: Render<Output = Self::Output>;
    fn body(&self)   -> Self::Body;                     // required — no default
    fn header(&self) -> Option<Self::Header> { None }   // optional
    fn footer(&self) -> Option<Self::Footer> { None }   // optional
}
```

- **`Render::render` is the single sanctioned `-> Element` outside a
  `#[component]`** (the `COMPONENTS.md` Frame-contract exception). `type Model:
  ddd::Model` forces every region to be a real ddd component (a published
  `View → Model`); `render` returns exactly one named `#[component]` invocation.
- **`type Output` is the agnosticism seam** — the trait names no framework node
  type. `Frame::Output` ties all three regions to one node type, so a frame can't
  mix a dioxus header with an html-string body.
- **Only `body` is required.** `header`/`footer` default to `None`; a frame that
  omits one names `Empty` for that associated type.
- **`type Model: ddd::Model` stays in the agnostic trait** (approved): ddd is pure
  Rust, so `browser-kit → ddd` keeps `browser-kit` dioxus-free while carrying the
  enforcement that motivated the whole design.

## The dioxus side (dioxus-kit)

```rust
#[derive(Clone, PartialEq, Default)]
pub struct Empty;
impl Render for Empty {
    type Model = EmptyModel;   // a fieldless ddd model
    type Output = Element;
    fn render(&self) -> Element { rsx! {} }
}
```

The frame **components** (`Dialog`, `Card`, `Page`) are **headless primitives**:
they own structure and behavior — `DialogRoot` wiring, `open`/`on_close`,
`use_body_scroll_lock`, region placement — and carry **no look**. They take styling
through the standard `#[props(extends = GlobalAttributes)] attributes: Vec<Attribute>`
extension (the same shape `dioxus_primitives::DialogRoot` uses), so a consumer
styles them without `dioxus-kit` ever depending on the app's `tw-macro`.

## The app side

- **One styled wrapper per family** (`WarcraftDialog`, `WarcraftCard`, …) owns its
  `classes!` `CLASS` and passes it to the primitive's parts: `Dialog { class: CLASS,
  .. }`, each part (backdrop, panel) addressable separately. This is the single
  owner of the shared look — the backdrop is styled **once** here and every dialog
  composes it, so per-dialog authoring never re-declares it. `CLASS` is a
  `ClassList` (`IntoAttributeValue`), flowing as an ordinary attribute; it lives in
  one app component and leaks to no sibling (the `COMPONENTS.md` "Styling a headless
  frame primitive" carve-out).
- **Each concrete instance** implements its `Frame` (a body, optionally a
  header/footer) and renders `WarcraftDialog { frame }`. No backdrop, no panel, no
  open-plumbing repeated. The body/header/footer **region leaves** are ordinary
  app ddd components — page-renderable, reading their data from context or props.
- **Open state is a plain `bool`.** The app's `OverlayState` (a `Copy` struct with
  one `Signal<bool>` field per dialog, read via `use_overlay_state()`) stays as-is;
  the concrete dialog's wrapper reads its own field, passes `open: bool` +
  `on_close`, and the `Signal` never enters a `View`/`Model`/`Presentation` (fixing
  the current `preview_open: Signal<bool>` violation of `COMPONENTS.md` line 962).

## The frame family

`Frame` is a family, not three fixed members. Each frame *component* owns its own
chrome + style; they share only the `Render`/`Frame` mechanism.

- `Dialog` — modal: backdrop, panel, header (title + close), optional footer, open
  plumbing. Grounded: reuses the existing `DialogHeader`/`Button`/`use_body_scroll_lock`
  leaves.
- `Page` — route-level region stack (the three route pages are the model; no shared
  page-shell exists today — `Page` becomes it).
- Card **types** — multiple, because the looks genuinely differ: a gold
  `ContentCard` (help ≡ templates ≡ hotkey-override share the gold surface, empty
  header), a blue `UnitDetailCard` (card-of-cards), etc. Same `Frame` mechanism,
  different chrome/style. The ~45 duplicated surfaces consolidate onto these.

## Open build-time detail

A frame has several styled surfaces (backdrop **and** panel, plus header/footer
chrome), and one `class` styles one root. So the primitive exposes each surface as
a separately-addressable part (per-part attribute props, or a small Radix-style
compound split). The addressing mechanism is settled at build time; the principle
(each part stylable, shared parts styled once in the family wrapper) is fixed.

## Migration phases

1. **Traits + primitives** land in `browser-kit` (`Render`, `Frame`) and
   `dioxus-kit` (`Empty`, `Dialog`/`Card`/`Page`), published + retagged.
   Fail-fast compile-check: a generic component `#[derive(Props)]` over `F: Frame`
   with the associated-type bounds must build under Dioxus 0.7.9 before proceeding.
2. **Dialogs** migrate onto `Dialog` via `WarcraftDialog` — one instance
   (Preview + Download for the footer path) proven end-to-end under `moon run :ci`
   before the rest.
3. **Cards** consolidate the ~45 surfaces onto the card frame types.
4. **Pages** onto `Page` when wanted.

Every phase ends `moon run :ci` green and the app exercised in the browser; e2e
selectors coupled to dialog/card markup are updated in the same change.
