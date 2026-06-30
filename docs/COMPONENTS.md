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

The body of a component is a flat list of hook invocations followed by pure
RSX. The hooks come first, one per line, each returning already shaped data.
The RSX comes second and only places that data in the tree.

What is allowed at the top of the body is a flat hook call and nothing more:
`let preview = use_preview_dialog(props);`. What is never allowed is work. No
destructuring that does work, no chaining off a hook result, no conditionals
that build values, no inline domain calls, no `.read()` / `.map()` / `.collect()`
ladders. If a line does anything other than name the result of a hook, it does
not belong in the body.

Everything the markup needs arrives already shaped through a hook. Anything that
is not "call a hook" or "place this value in the tree" lives in a sibling file,
not in the component body.

## Logic composes through hooks, the way markup composes through components

Domain logic, `localStorage`, and web APIs are reached only through hooks, never
inline in a body. The hook layer mirrors the component layer exactly.

- **Primitive hooks are the leaves.** Each owns one concern and is reused across
  components: `use_custom_keys` for the domain facade, `use_dialog_open` for
  shell open state, `use_local_storage` for persistence, `use_upload_picker`
  for the picker web API. A primitive hook is to logic what `HotkeyBadge` is to
  markup.
- **One composed hook per component is the parent.** It calls the primitive
  hooks, wires them together, and hands the body a single already shaped result.
  `use_preview_dialog(props)` calls `use_custom_keys` and `use_dialog_open`
  inside it, the way a parent component composes its leaf children. The body
  sees one flat line, not the wiring.
- A body never reaches a primitive hook directly when a composed result reads
  more clearly. Push the wiring down into the component's own hook so the body
  stays a single declarative line.

Reading a component must tell the story immediately and never require following
a logical calculus. The composed hook is where the calculus lives.

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

This rule governs our own components. Library components are exempt: things like
`document::Stylesheet` and the `dioxus_primitives` dialog parts (`DialogRoot`,
`DialogContent`) are external building blocks, not project code, so they have no
directory here and may appear in a body the same way `Stylesheet` already does.
A project class we put on a library component is still ours and still belongs in
the owning component's stylesheet.

## One class per component

A class is a component boundary. If you give an element a class, that element is
a component. It follows that a component's markup carries exactly one classed
element: its own root. Every other element that needs a class is a child
component, with its own directory, its own one class, and its own stylesheet.

Two classed elements in one markup file is the signal that the file is doing the
work of two components. Split it. Keep splitting until each markup file names
exactly one class.

The root's class string may still carry variant or state modifiers, since those
describe the same component, not another one. `grid-tile dragging-source` is one
component. `button button-primary` is one component. What is not allowed is a
second, structurally distinct classed element in the same file, like a title and
a decoration both classed inside a header. Those become `DialogTitle` and
`DialogHeaderDecoration`, each its own leaf.

Unclassed structural elements are tolerated but suspect. A bare wrapper `div`
with no class is fine as pure layout glue, but the moment it wants styling it
wants a class, and then it wants to be a component.

The `grid_editor` subsystem already embodies this: `grid_tile` holds only
`.grid-tile`, and `tile_figure`, `tile_badge`, `hotkey_badge` are each their own
one-class component.

There is no such thing as too many components. There is only too few. A single
styled paragraph with its own class is a component. When in doubt, split.

## Each component owns its style

Every component that has any styling owns its own stylesheet inside its own
directory. The class it defines is the class it renders. Never define a child's
class in a parent's stylesheet.

The one allowed exception: a parent that is always mounted may keep emitting a
stylesheet that a conditionally rendered child uses, when mounting it with the
child would cause a first paint flicker. The drag follower overlay does this for
the ghost on purpose.

### Nothing global styles a component

The only rules allowed in a global stylesheet are the design system tokens:
font families and the custom color palette, the kind of thing Tailwind's theme
layer holds. Everything else, every layout rule, every size, every border and
shadow that paints a component, lives in that component's own `styles/`
directory and nowhere else.

There is no shared component stylesheet. If two components want the same look,
they do not reach for a common class in a global file, and they do not import
one component's stylesheet into the other. They each carry their own copy of the
rule, or the shared look becomes its own leaf component that both compose.
Duplicating a handful of CSS lines is the correct trade. The thing being bought
is isolation by construction: editing one component's style cannot reach another
component, and fixing a component's style is a focused edit of one `styles/`
directory, never a hunt through the whole CSS codebase.

---

## The directory layout

Every component directory follows the same shape. `grid_tile` is the canonical
example:

```
grid_tile/
  mod.rs            the component function, flat hooks then pure RSX, plus the pub use re-exports
  props.rs          the Props struct and its From conversions
  hooks.rs          the component's composed hook, wiring primitive hooks together (optional)
  logic.rs          everything the body is not allowed to do (optional)
  state.rs          component-local enums, e.g. visual state (optional)
  style.rs          the asset! stylesheet constants
  styles/           the CSS, base.css plus disjoint viewport bands
  components/        child components, each its own directory of this same shape
```

A component with children nests them under `components/`. A leaf component omits
`components/`. A component with no logic beyond `From` conversions omits
`logic.rs`. A component that reaches the domain, `localStorage`, or a web API
carries a `hooks.rs` with its one composed hook, and omits it otherwise.

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

## One attribute per line

Prefer one attribute per line in RSX. When an element or component carries more
than one attribute, each attribute goes on its own line, and the trailing child
or text starts a fresh line too. It costs more lines, but the structure reads at
a glance: the attribute list is a vertical column the eye scans, not a sentence
it has to parse.

```rust
// no
div { class: "dialog-body", role: "group", {body} }

// yes
div {
    class: "dialog-body",
    role: "group",
    {body}
}
```

A single attribute with no child may stay on one line (`img { src }`). The rule
bites the moment there are two attributes, or an attribute plus a child.

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

## Two kinds of reuse: extend a base, or compose a leaf

The variant pattern above is for a base that is generic over a behavior, where
variants bind the type parameter. A `Dialog` base owns the dialog shell and its
chrome CSS, and each concrete dialog is a variant that fills the body, the way
`CommandGridEditor` wraps `GridEditor`. Shell sizing, scrollbar, and backdrop
rules live once on the base, never copied per dialog.

A small shared piece that is not generic over a behavior is not a variant. A
close button, a primary button, an edit panel are plain leaf components. They
live once, own their class and CSS, and parents reuse them by nesting them in
the tree, the way `Grid` drops in `HotkeyBadge`. Do not force a behavior
parameter onto a button to make it look like a variant. Extend a base when there
is a behavior to bind, compose a leaf when there is not.

---

## Modules are public; imports carry the full path

A component is reached by its module path, not by a re-export chain. Make every
module on a public path `pub mod`, and import a component through its full
semantic path: `...::headed_grid::components::grid::Grid`. The path itself tells
you exactly where the component lives.

Do not re-export descendants up the tree. A `mod.rs` that lists
`pub use child::{A, B, C, ...}` to surface a grandchild forces every new layer to
copy that list into yet another file. It does not scale, and it erases where each
name actually lives, the import path loses all ordering. The only re-export a
component may carry is its own public surface from its own sibling files:
`pub use props::XProps;`, and `pub use state::XState;` when it has visual state.
Children are reached by traversing `pub mod`, never through a flattened list.

So a component's `mod.rs` carries exactly: the `pub` component function, its own
`pub use props::XProps` (and `state`), `pub mod components` for its children, and
private `use` imports of the specific children it renders. Nothing is re-exported
that the component does not itself own.

The crate root may still expose a curated public surface for an external consumer
(the `gallery` showcase), but it does so with full-path `pub use`s, and the
intermediate modules stay free of flattening.

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
