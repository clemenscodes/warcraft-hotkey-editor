# Component structure rules

These are the rules for how Dioxus components in `hotkey-editor` are named,
laid out on disk, and written. They are non negotiable and must never require
reminding. Read this file before adding or refactoring any component.

These sit alongside `docs/ARCHITECTURE.md` (the wall: where domain logic lives
versus the renderer) and `docs/RUST_STYLE.md` (how Rust is written). All three
must hold. This file is the reference for the renderer side. It was distilled
from the `grid_editor` subsystem, which is the worked example for every rule
below.

> Before writing a component: read the relevant `grid_editor` files and mirror
> their shape exactly.

---

## The one rule that drives the rest

**A component renders. It does not compute.**

The body of a component is pure RSX. No `let` bindings, no destructuring of
work, no conditionals that build values, no domain calls. Everything the markup
needs arrives already shaped, and the body only places it.

Anything that is not "place this value in the tree" lives in a sibling file, not
in the component body.

---

## Directory equals component equals class

There is exactly one component per directory, and the names line up three ways
with no divergence allowed:

- the directory name in `snake_case`
- the component function in `PascalCase`
- the root CSS class the component renders in `kebab-case`

So `GridHeading` lives in `grid_heading/`, and its markup carries
`class: "grid-heading"`. If you find a directory whose component has a different
name, fix it. Prefer renaming the directory when the component name is public
API, otherwise rename the component. Fix the CSS class to match in the same pass.

## Each component owns its style

Every component that has any styling owns its own stylesheet inside its own
directory. The class it defines is the class it renders. Never define a child's
class in a parent's stylesheet.

The one allowed exception: a parent that is always mounted may keep emitting a
stylesheet that a conditionally rendered child uses, when mounting it with the
child would cause a first paint flicker. The drag follower overlay does this for
the ghost on purpose.

---

## The directory layout

Every component directory follows the same shape. `grid_tile` is the canonical
example:

```
grid_tile/
  mod.rs            the component function, pure RSX, plus the pub use re-exports
  props.rs          the Props struct and its From conversions
  logic.rs          everything the body is not allowed to do (optional)
  state.rs          component-local enums, e.g. visual state (optional)
  style.rs          the asset! stylesheet constants
  styles/           the CSS, base.css plus disjoint viewport bands
  components/        child components, each its own directory of this same shape
```

A component with children nests them under `components/`. A leaf component omits
`components/`. A component with no logic beyond `From` conversions omits
`logic.rs`.

---

## Props flow by conversion, never by hand

Each child's props are produced by a `From<&ParentProps>` impl, and the parent
spreads them in. This is the only sanctioned way to pass props down.

```rust
#[component]
pub fn GridTile(props: GridTileProps) -> Element {
    let figure = TileFigureProps::from(&props);
    let badge = TileBadgeProps::from(&props);
    let GridTilePresentation { class, tabindex, row, column, onclick, .. } =
        GridTilePresentation::from(&props);
    rsx! {
        for href in GRID_TILE_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        div {
            class, 
            tabindex, 
            "data-grid-row": row, 
            "data-grid-col": column, 
            onclick,
            TileFigure { ..figure }
            TileBadge { ..badge }
        }
    }
}
```

Rules inside this pattern:

- Conversions take `&ParentProps` by reference. `EventHandler` and `Signal` are
  `Copy`, so the parent props is never moved and one borrow feeds every child.
- The `From` impl is the home for the work the body may not do: building class
  strings, resolving attributes, wiring event handlers, calling the domain.
  Put it in `props.rs` for plain prop mapping, or in `logic.rs` when it carries
  real work.
- Spread the result: `Child { ..ChildProps::from(&props) }`. Do not list fields
  by hand at the call site.
- Use RSX shorthand. Name the local exactly as the prop so `Foo { bar }` works
  instead of `Foo { bar: bar }`. Name a struct field after the attribute or
  child prop it feeds (for example an image alt text field is named `alt`, not
  `label`, so `img { src, alt }` is all shorthand).

## Markup has no branches that build values

No `if let` in the body to decide what to render. Push the optional or branching
piece into its own leaf component that early returns, or render a guarded child.
The body stays a flat list of children with no nesting beyond the natural tree.
Loops over a collection in the markup are fine. Reaching for a deep `if/else`
ladder is the signal to extract a child.

---

## Naming the components

Drop redundant subsystem prefixes, keep real variant words.

- The engine grid is `Grid`, not `CommandGrid`. The subsystem being named
  `grid_editor` does not justify stamping `GridEditor` onto every part inside it.
- Variant discriminators stay, because they carry meaning. `CommandGridEditor`,
  `ResearchGridEditor`, and `UprootedGridEditor` keep Command, Research, and
  Uprooted because those name genuine variants, not noise.
- Name a thing for what it is. A struct of fields that configure an editable grid
  is not a "section". The component that is the editor implementation is the
  `GridEditor`, not a `GridSection`.

## Generic components and their props

When one component is generic over a behavior or variant type, there are two
distinct prop structs:

- the caller facing input is `XConfig`. It is flat, it carries no type
  parameter, and callers build it without knowing which variant they target.
  Example: `GridEditorConfig`.
- the component's own props is `XProps<B>`. It carries the type parameter the
  generic component needs, wrapping the config plus the behavior marker.
  Example: `GridEditorProps<B>` is `{ behavior: B, config: GridEditorConfig }`,
  built with `From<&GridEditorConfig>`.

Two structs are required because Dioxus needs the type parameter to appear in
the props, while callers must stay agnostic to the behavior. The thin variant
wrappers bind the behavior:

```rust
#[component]
pub fn CommandGridEditor(props: GridEditorConfig) -> Element {
    rsx! {
        GridEditor { ..GridEditorProps::<CommandBehavior>::from(&props) }
    }
}
```

## Base and variants are flat, not nested

A generic base and its variant wrappers are siblings under one plural group
directory. Variants are never subcomponents of the base, since they depend on
it, not the other way around.

```
grid_editors/
  grid_editor/            the base GridEditor<B>, with its own components/
  command_grid_editor/    CommandGridEditor
  research_grid_editor/   ResearchGridEditor
  uprooted_grid_editor/   UprootedGridEditor
```

Variants reach the base with `super::grid_editor::...`. The group `mod.rs`
re-exports the public surface. Prefer the flattest layout that still keeps
directory equals component. Do not add grouping layers like `base/` or
`extensions/` that would break that equality.

---

## Types at the props boundary

Props carry domain types, never stringly typed stand ins, and never an `Option`
for a value that is always present.

- A hotkey is `HotkeyToken`, a key is the domain `KeyCode`, a grid address is
  `GridCoordinate`. Convert to a display string at the leaf that renders text,
  not before.
- If a value always exists, the type is `T`, not `Option<T>`. A tile always has
  a hotkey, a draggable follower always has an icon. Make the invariant true at
  the domain source and propagate it, rather than threading an `Option` that can
  never be `None`.

See `docs/ARCHITECTURE.md` for why the renderer never computes domain decisions,
and `docs/RUST_STYLE.md` for the `From` and `TryFrom` rules these conversions
follow.

---

## Verify every pass

A component change is not done until all of these are green:

```
nix develop -c cargo clippy -p hotkey-editor -p gallery --target wasm32-unknown-unknown
nix develop -c cargo test -p warcraft-keybinds
nix develop -c cargo fmt --check
```

Also confirm every `asset!` path resolves to a real file, since a renamed or
moved directory silently breaks them and clippy alone may not catch it.
