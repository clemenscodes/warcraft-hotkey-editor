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

This rule governs our own components. Library components are exempt: the
`dioxus_primitives` dialog parts (`DialogRoot`, `DialogContent`) are external
building blocks, not project code, so they have no directory here and may appear
in a body directly. A project class we put on a library component is still ours:
it is that element's identity, and its styling is the named utilities beside it,
exactly as for our own elements.

## One class per component

A class is a component boundary. If you give an element a class, that element is
a component. It follows that a component's markup carries exactly one classed
element: its own root. Every other element that needs a class is a child
component, with its own directory, its own one class, and its own `style.rs`.

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

### The one class is an identity, not a style hook

That single class is the component's **identity class**: a stable, semantic,
prefixed name (`help-legend-row`) that matches the directory and component name.
It exists only for selectability — e2e selectors, debugging, finding the element
in the DOM. It is **never a Tailwind utility**, so Tailwind generates no rule for
it and it carries **no styling, ever**. All styling is the named utilities beside
it. The identity is its own const in `style.rs`, listed first, ahead of the band
arrays. e2e decides whether it bothers to select a component, never whether the
component gets an identity — every component is selectable.

## Styling: named Tailwind utilities, never arbitrary values

A component is styled with Tailwind utility classes and nothing else. There are
no per-component CSS files, no `asset!` stylesheets, no `styles/` directories,
no `document::Stylesheet` for project styling.

The hard rule: **no arbitrary values, ever. Not a single bracket in a
component's class list.** No `min-[1100px]:`, no `w-[min(1900px,95vw)]`, no
`[background:linear-gradient(...)]`, no `text-[1.6rem]`, no `!important`. A
bracket in a component class is inline CSS in disguise and is forbidden.

Every value a component references is a named token or a named composite that
lives once in the global layer (`crates/hotkey-editor/tailwind.input.css`):

- **Scalars** — breakpoints, spacing, type sizes — are `@theme` tokens:
  `--breakpoint-*`, `--spacing-*`, `--text-*`. Tailwind turns each into a named
  utility (`gap-section`, `text-heading`) and a named variant (`laptop:`).
- **Composite surfaces** — gradients, multi-layer shadows, an embossed
  text-shadow, a bordered chip — are named `@utility` rules: `surface-callout`,
  `chip-gold`, `text-shadow-emboss`. The raw `rgba(...)`, gradient, and shadow
  literals live inside that one `@utility` definition and nowhere else.

Raw px / rem / rgba / gradients appear **only** in `@theme` and `@utility`. A
component never names a literal value. If a component needs a value that has no
token, add the token to the global layer first, then use it by name.

The global layer is the single source of every value; components are pure
compositions of named utilities. Changing the gold, a gap, or a breakpoint is
one edit in `@theme`/`@utility`, never a hunt through components.

## Responsive bands

Layout uses six named bands plus an always-on `BASE`, defined once in
`tailwind.input.css`. The names are honest device/resolution classes, not
marketing terms — there is no `wide` or `ultrawide` width band, because those are
aspect ratios (21:9 / 32:9), not widths:

| band | width range | what it is |
|------|-------------|------------|
| *(BASE)* | all widths | always-on, unprefixed |
| `mobile` | `< 768px` | phones |
| `tablet` | `768–1279px` | portrait tablets |
| `laptop` | `1280–1919px` | laptops |
| `desktop` | `1920–2559px` | FHD desktops |
| `qhd` | `2560–3839px` | 1440p |
| `uhd` | `≥ 3840px` | 4K |

The six bands are **disjoint width ranges** (`@custom-variant` in
`tailwind.input.css`), not min-width breakpoints: **nothing inherits across
bands**. A `mobile:` style never leaks up to desktop, and a `laptop:` style never
leaks down to a phone — each band paints only its own range.

Because nothing cascades, a style that should apply everywhere goes in **`BASE`**
(unprefixed, always-on): `flex`, `m-0`, `text-warcraft-gold`. The six bands carry
only the **width-specific deltas** — a component sets `BASE` to its common
appearance and overrides per band only where the width genuinely changes
something (`mobile:text-body-sm` vs the `BASE` `text-body`). `BASE` must never
carry a band-prefixed class (the macro rejects it); a band must carry only its
own prefix (the macro rejects `uhd:flex` in `MOBILE`).

A component declares **all seven** lists (`BASE` + the six bands); an unused one
is an explicit empty `&[]`, so a band is never silently missing. Within one list
the property order is layout → sizing → spacing → border → typography → color →
effects → state.

## style.rs and the `classes!` macro

Tailwind's scanner reads source as plain text and never evaluates code, so a
class name assembled at runtime (`format!`, a join, concatenation) is invisible
to it and its CSS is never generated. Every class token must therefore appear as
a literal in the source.

Each component declares a `BASE` and one **`&[&str]` per band** of single-class
literals in its own `style.rs`; `classes!` then derives the identity from the
directory and joins everything at compile time into a
`pub(super) const CLASS: ClassList`:

```rust
// help_top_row/style.rs — wide layout in BASE, the phone override per band
use crate::classes;

const BASE: &[&str] = &["flex", "flex-row", "items-start", "gap-columns"];
const MOBILE: &[&str] = &["mobile:flex-col", "mobile:gap-section"];
const TABLET: &[&str] = &["tablet:flex-col", "tablet:gap-section"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
// CLASS starts with the derived identity "help-top-row" (from the directory).
```

```rust
// help_top_row/mod.rs — body just names CLASS; assert_component! binds the name
use crate::assert_component;
use style::CLASS;

assert_component!(HelpTopRow);

#[component]
pub fn HelpTopRow() -> Element {
    rsx! {
        div {
            class: CLASS,
            HelpWorkflowSection {}
            HelpLegendSection {}
        }
    }
}
```

Why this shape:

- **The identity is derived, not written.** `classes!` reads the component
  directory from `module_path!()` and emits the kebab identity (`help_top_row` →
  `help-top-row`), so the class always equals the directory and the caller only
  thinks about band styles.
- **The component name is bound too.** `assert_component!(HelpTopRow)` in
  `mod.rs` asserts the PascalCase function name equals the directory
  (capitalization included), closing the triangle `component == directory ==
  class` at compile time. A `HelpTopRow` living in `top_row/` fails the build.
- **`BASE` plus all six bands are mandatory and named.** Every component spells
  out `BASE MOBILE TABLET LAPTOP DESKTOP QHD UHD`, an unused one being an explicit
  `&[]`. A band is never silently missing, and `grep MOBILE` lists every
  component's mobile styles.
- The macro guards the whole contract at compile time: its fixed arity rejects a
  missing list, `assert_named` rejects a misnamed const, `assert_base` rejects a
  band-prefixed class in `BASE`, and `assert_band` rejects a class whose prefix
  does not match its band — `uhd:flex` inside `MOBILE` fails the build.
- Each utility is a separate literal in a `&[&str]`, so rustfmt lays them one per
  line (no line-width fights) and Tailwind's scanner sees every token verbatim.
- `classes!` joins them in a `const fn` into one string at **compile time** —
  zero runtime cost; the body only names `CLASS`.
- `CLASS` is a `pub(super)` **`ClassList`**, not a `&str`. `mod style;` is
  private, so no other component can name the path; and `ClassList` implements no
  `Display` and no accessor, so a component cannot interpolate or append to it
  (`class: "{CLASS} other-class"` does not compile). A component can only ever
  wear exactly its own class — styling coupling is impossible to express.

The macro lives once at the crate root (`crate::classes!`); its `const fn`
helpers are in `src/styling.rs`.

## Stateful components and the `states!` macro

Some components have mutually-exclusive visual states on one element: a grid tile
is idle **xor** the drag source **xor** a drop target. A `ClassList` is opaque,
so the body cannot conditionally swap classes — and wrappers do not fit, because
the state is runtime and N-way on the *same* element (wrapping would re-render
the whole subtree per state). These use `states!` alongside `classes!`.

`classes!` produces the base look; `states!` layers one **flat (non-responsive)
overlay** per state on top of it. State overlays carry no band prefix (the
macro rejects one): a state's appearance is the same at every width, while the
element's *sizing* lives in the base bands. The component's state enum lives in
`state.rs`; the match is exhaustive, so every state must be styled.

```rust
// grid_tile/state.rs
#[derive(Clone, Copy, PartialEq)]
pub enum TileState { Idle, DragSource, DropTarget }

// grid_tile/style.rs
use crate::{classes, states};

const BASE: &[&str] = &["relative", "flex", "items-center"];
// ... the six bands (the tile's sizing) ...
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }

const IDLE: &[&str] = &[];
const DRAG_SOURCE: &[&str] = &["opacity-40", "ring-2", "ring-warcraft-gold"];
const DROP_TARGET: &[&str] = &["bg-warcraft-gold-dim"];

states! { TileState, Idle => IDLE, DragSource => DRAG_SOURCE, DropTarget => DROP_TARGET }
// → pub(super) fn class(state: TileState) -> ClassList
```

The state is chosen in `From<&Props>` (logic stays out of the body); the body
just places the result:

```rust
// grid_tile/mod.rs
let presentation = GridTilePresentation::from(&props);  // computes style::class(state)
rsx! { div { class: presentation.class, /* ... */ } }
```

The joined class per state is built at compile time, so the selector is a plain
match returning a precomputed `ClassList`. The body never branches, `ClassList`
stays opaque, and every token stays a literal for the scanner — the same
guarantees as `classes!`.

Both macros live at the crate root (`crate::classes!`, `crate::states!`); their
`const fn` helpers are in `src/styling.rs`.

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
  style.rs          the identity const + per-band class arrays, via classes!
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

Also confirm no component class list contains a bracket (no arbitrary values and
no `!important`), every utility resolves to a `@theme` token or an `@utility`
composite, and any value with no token was added to the global layer first. A
class assembled outside a literal is invisible to Tailwind's scanner, so its CSS
will silently never generate — keep every token a literal in a `style.rs` array.
