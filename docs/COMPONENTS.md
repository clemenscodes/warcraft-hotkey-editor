# Component structure rules

These are the rules for how Dioxus components in `hotkey-editor` are named,
laid out on disk, and written. They are non negotiable and must never require
reminding. Read this file before adding or refactoring any component.

These sit alongside `docs/ARCHITECTURE.md` (the wall: where domain logic lives
versus the renderer) and `docs/RUST_STYLE.md` (how Rust is written). All three
must hold. This file is the reference for the renderer side.

The subsystems approved **end-to-end** — every structural rule below *and* the
full styling / container-query / responsiveness treatment — are
`components/shell/header` and its bottom-of-shell counterpart
`components/shell/footer`. They are the role models: the design-wise perfect
application of this document, verified in the browser at every band. The header is
a full-bleed bar of buttons; the footer is a full-bleed bar of text, converted the
exact same way, so between them they show the model on both axes. When a new
component's shape is in question, the answer is "what does the header do here?" (or,
for text, the footer), and nothing lower than that bar ships. The older
`grid_editor` subsystem still illustrates the structural rules and is referenced
throughout below, but it was never carried through the same styling treatment;
where they differ, **the header and footer win**. The capstone walk-through at the
end of this file breaks the header down quality by quality, and a shorter companion
does the same for the footer.

> Before writing a component: read the `components/shell/header` tree and the
> capstone walk-through at the end of this file, and mirror their shape exactly.

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

---

## `Element` is never a prop and never a value — compose typed components

A corollary of "a component renders, it does not compute," and the single most
important composition rule: **`Element` is produced by exactly one thing, a
`#[component] fn`, and it is never passed around as data.** You compose a UI out
of well-typed components nested by name — never out of blobs of pre-rendered
markup.

This is absolute. There is no "structural wrapper" exception, no "framework
children slot" exception, no "it's just glue" exception. The following are all
**forbidden, everywhere, with no exceptions**:

- **`children: Element`** as a prop — and equally `Option<Element>`,
  `Vec<Element>`, or any other prop field, struct field, or enum variant whose
  type is `Element`. A component never *receives* the markup it wraps. A
  `#[component] fn Foo(children: Element)` inline-children parameter is the same
  violation.
- **`fn … -> Element` that is not itself a `#[component]`** — no trait method
  returning `Element` (`trait XKind { fn tile/cells/scroll/header(…) -> Element }`
  is the classic offender), no free function or `logic.rs`/`hooks.rs` function
  returning `Element`. Logic returns **data** (line 33: *"a component renders, it
  does not compute"*). Only a component renders.
- **Binding markup to a variable and threading it as a prop** —
  `let children = rsx! { … }; Foo { children }`. Building an `Element` in a body
  or a `logic.rs` and handing it to another component is exactly the same sin as
  a `children: Element` prop; it just hides it in a local.

**How you compose instead.** A parent *names* its children; it never *receives*
them. Its body is a flat list of specific typed components — or a `for` loop over
typed **data** that renders one specific typed component per item:

```rust
rsx! {
    section {
        class: CLASS,
        DamageMatchupHeader { unit }          // a specific typed leaf, by name
        for attack_type in ALL_ATTACK_TYPES {
            DefenseMatchup { attack_type, defense_type }   // a for-loop over DATA
        }
    }
}
```

**Sharing a look is not sharing a component.** When two components look the same,
they do not share a wrapper that takes `children` — that wrapper's `children:
Element` is the violation, and the "reuse" it buys is illusory. They each write
the **same utility-class values** in their own `style.rs` (see "Styling: shared
values, individual looks" — *sharing a value is not coupling*). A scaffold `div`
that both a damage row and a defense row want is not a `MatchupGrid { children }`
leaf; it is the same `grid grid-cols-2 …` class list written in each row, each
rendering its own typed cells directly.

**Reusing a piece is nesting a specific typed leaf.** Genuine reuse is a leaf
component with **typed props** that parents nest by name — the way `Grid` drops
in `HotkeyBadge`, or `ToolbarButton` is nested by all nine action buttons. The
leaf takes `HotkeyToken`, `KeyCode`, a `count: usize` — never `children`.

**A generic base selects DATA, never `Element`.** When a base is generic over a
variant (`XProps<B> { behavior: B, config }`, per "Generic components and their
props"), the marker `B` chooses **typed data**, and the base's `#[component]`
body renders a **fixed** typed component from props built by `From`. A
`trait Kind { fn render(…) -> Element }` is not a generic base — it is logic
producing `Element`, forbidden above. Where variants must render *genuinely
different* components, there is no generic base at all: they are separate
components (per "The render tree IS the directory tree"), each composing shared
typed leaves by nesting.

**The test, and it is mechanical:** grep the crate for `-> Element` — every hit
must be on a `#[component]`. Grep for `: Element` in a props/struct/param
position — there must be none. Grep for `let … = rsx!` threaded into a prop —
there must be none. Only `#[component] fn`s make `Element`; everything else makes
data and nests typed components. (The `spec-lint.sh` pre-commit hook enforces the
`children: Element` half of this; the rest is enforced by review the same way the
`super::` test is.)

---

## The render tree IS the directory tree — the law you will be tempted to break

Read this twice. It is the single most-violated rule in this codebase, every
violation has been caught, and every one has cost real time and trust. It is not
a style preference. A wrong placement here is a **structural defect**: the change
gets **reverted, not patched**, and the person who wrote it is asked to do it
again. Treat a misplaced component the way you would treat a compile error.

State it once, absolutely:

> **If a component renders another component, the rendered component is its child
> and lives inside its own `components/` directory. There is no other correct
> location.**

The RSX render tree and the on-disk directory tree are **the same tree** — not
"similar", not "usually aligned", the *same*. If `A` renders `B` in its RSX, then
`B` lives at `A/components/B/`. Full stop. This holds at every depth, forever:
`A` renders `B` renders `C` means `A/components/B/components/C/`.

### The one test that proves a violation — run it on every component you touch

Open the component's `mod.rs`. For each component it renders in RSX, ask: **does
that component's directory sit under my own `components/`?** If it sits anywhere
else — a sibling directory, an ancestor, a cousin — **it is a violation and the
build of trust is broken.**

The mechanical tell, and it is nearly infallible:

> **A `use super::…::TheComponent;` (or `super::super::…`) for a component you
> then render in RSX is a red flag that PROVES the rule is broken.**

`super::` reaches *out* of your directory. A child you render lives *under* you,
so you reach it with `use components::…` — never `super::`. So:

- `use super::header_toolbar::HeaderToolbar;` followed by `HeaderToolbar {}` in
  your RSX ⇒ `header_toolbar/` is misplaced. It is a sibling; it must become your
  child at `<you>/components/header_toolbar/`. **Move it.**
- `use components::header_toolbar::HeaderToolbar;` ⇒ correct: it is already your
  child.

`super::` is legal for exactly two things and nothing else: (1) reaching your own
sibling *files* — `super::props`, `super::state`, `super::logic`, `super::style`,
`super::hooks`, `super::data`; and (2) the two exceptions below (a generic base,
a shared leaf), which are reached by their own explicit paths. `super::` is
**never** the way to reach an ordinary child you render.

### Worked example — the exact violation this rule exists to kill

Wrong. `HeaderActions` renders three components that sit *beside* it, so its
`mod.rs` is full of `super::` render imports — each one is the proof of the crime:

```
header/components/
  header_actions/     renders CollisionsButton, HeaderToolbar, BurgerMenu
  collisions_button/  ✗ rendered by HeaderActions → belongs UNDER it
  header_toolbar/     ✗
  burger_menu/        ✗
```
```rust
// header_actions/mod.rs — every one of these is a red flag
use super::collisions_button::CollisionsButton;   // ✗ super:: for a rendered child
use super::header_toolbar::HeaderToolbar;          // ✗
use super::burger_menu::BurgerMenu;                // ✗
```

Right. They nest under the component that renders them; the imports become
`components::…`:

```
header/components/
  header_actions/
    components/
      collisions_button/
      header_toolbar/     (keeps its own components/ subtree)
      burger_menu/        (keeps its own components/ subtree)
```
```rust
// header_actions/mod.rs
use components::collisions_button::CollisionsButton;   // ✓
use components::header_toolbar::HeaderToolbar;          // ✓
use components::burger_menu::BurgerMenu;                // ✓
```

The same test bites at every depth. `BurgerDrawer` renders `BurgerDrawerBody` ⇒
`burger_drawer/components/burger_drawer_body/`, never beside `burger_drawer/`. A
Host renders its leaf ⇒ the leaf nests under the Host:
`resolve_button_host/components/resolve_button/`, **never** beside the host. Where
older prose in this file says "keep the Host *beside* the leaf", that wording is
wrong and this rule overrides it: the leaf is the Host's child. Nest it.

### The ONLY two escapes — structural, countable, not judgment calls

The default is: a component lives under its single renderer. You may deviate
**only** if one of these two patterns *exactly* applies. You do not get to invent
a third, and "it felt like it belonged there" is not one of them.

1. **A leaf rendered by two or more sibling components** lives in a `shared/`
   grouping directory at their nearest common parent —
   `parent/components/shared/the_leaf/` — reached by each renderer via its full
   path. The decision is a pure **count of render sites**, nothing else:

   - rendered by exactly one component → nest it under that one component;
   - rendered by two or more → `shared/` at their common parent.

   You never flat-dump the leaf beside its renderers, and you never nest a
   shared leaf under just one of several renderers. (See "Shared leaves".)

2. **A generic base and its variant wrappers** are flat siblings inside one
   plural group directory (`grid_editors/grid_editor/` beside
   `grid_editors/command_grid_editor/`), and the variants reach the base with
   `super::grid_editor::…`. This applies **only** when the wrapper binds the
   base's generic **behavior** type parameter (the way `CommandGridEditor` binds
   `GridEditor`'s `GridBehavior`). There is **no** "fills a slot/body the base
   exposes" case — a base that receives a body is `children: Element`, forbidden
   above. It does **not** cover "this wrapper just reuses that leaf" — that is
   exception 1 if the leaf has several renderers, or plain nesting if it has one.
   (See "Base and variants are flat".)

If neither pattern fits exactly, there is no exception — nest it. When unsure
which case you are in, **count the render sites**; that number, never your
intuition about which component "owns" or "is really" the leaf, decides.

### Why it is absolute

The directory tree is how every reader — and the gallery, and you in six months —
navigates the render tree without opening a single file. A flat dump erases who
renders what; a child placed in the wrong subtree silently couples two subtrees
that must stay independent, and that coupling is exactly how bugs we already fixed
came back. So this is enforced socially the way the type checker is enforced
mechanically: **a `super::`-rendered child is a broken build.** Do not ship one,
do not rationalize one, and do not believe any claim that a header, dialog, or
toolbar is "100% compliant" until you have run the `super::` test on every
`mod.rs` yourself.

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

## Presentational leaves, connected wrappers

App-wide state — the `CustomKeys` document, the grid layout, the upload status —
reaches the leaf that renders it as **props**, and never any other way. This splits
every such component into two roles that are never blurred.

**The leaf is presentational: props in, markup out.** A component that renders a
slice of the document (a collisions count, a serialized-keys preview, an editable
grid) takes that data as props and does nothing to *fetch* it. It reads no context
and calls no domain / `localStorage` hook of its own. Hand it its props and it
renders; it is a pure function of them. This is exactly what lets the **gallery**
render it directly — the gallery hands it any value it likes and every state and
variant falls out for free, with no document anywhere in sight. A leaf that reached
for `use_custom_keys` itself could not be showcased and could not be varied.

**The real data comes from a dedicated wrapper whose only job is the seam.** When the
app needs to feed a leaf the live document, it renders a **connected wrapper** — a
component that does *nothing* but call the primitive hook(s), shape the result, and
render the leaf with those props. It owns no markup beyond the single child it wraps.
Name it `<Leaf>Host`. Because the Host renders the leaf, **the leaf is the Host's
child and nests under it** — `<leaf>_host/components/<leaf>/`, never beside the Host
(this is "The render tree IS the directory tree" applied; the `super::` test catches
any slip):

```rust
// collisions_button_host/mod.rs — connected: one hook call, shape, render the leaf.
// The leaf lives at collisions_button_host/components/collisions_button/.
#[component]
pub fn CollisionsButtonHost() -> Element {
    let button = use_collisions_button();   // calls use_custom_keys + use_grid_layout, shapes the count
    rsx! {
        CollisionsButton { ..button }
    }
}
```

```rust
// collisions_button/mod.rs — presentational: props in, markup out. No hook, no context.
#[component]
pub fn CollisionsButton(props: CollisionsButtonProps) -> Element {
    let CollisionsButtonPresentation { count, onclick, .. } = CollisionsButtonPresentation::from(&props);
    rsx! { /* renders the count and badge from props alone */ }
}
```

So the app renders `CollisionsButtonHost`; the gallery renders `CollisionsButton`,
which lives at `collisions_button_host/components/collisions_button/` because the
Host renders it. The hook is called in exactly one place, on behalf of exactly one
leaf.

**The rule this forces: an uninvolved parent threads nothing.** A container that does
not itself use the document must neither receive it nor forward it. `HeaderActions` is
pure layout — it renders `CollisionsButtonHost`, `HeaderToolbar`, and the burger; it
takes no `loaded_keys` prop and calls no document hook, because it has no stake in the
document. Pushing the fetch onto a parent that has no stake is the leak this pattern
exists to forbid: it is the same leak as a threaded god-signal, one layer smaller. The
same holds for the header's overlay dialogs — each dialog is a presentational leaf, and
a `PreviewDialogHost` / `TemplatesDialogHost` supplies its document; the header only
places the hosts.

Three roles, never merged:

- **presentational leaf** — props in, markup out; no hook, no context. The gallery
  renders this directly.
- **connected wrapper (`<Leaf>Host`)** — one hook call, shape, render the leaf. The
  only markup it owns is the single classed container root it wraps the leaf in. The
  app renders this.
- **container** — pure layout of children; fetches nothing, threads nothing.

### The Host doubles as the leaf's container

The connected wrapper is not only the *data* seam — it is also the leaf's **layout
container**. The one piece of markup a Host owns is a single classed root that wraps
its leaf, and that root is the container: it decides how much space the leaf gets,
and — when the leaf sizes itself responsively — establishes the container-query
context the leaf measures against. So a Host, like every component, carries its own
`style.rs` (`classes!`) and `assert_component!`; its identity class *is* the
container.

```rust
// export_button_host/mod.rs — the Host is the seam AND the container.
#[component]
pub fn ExportButtonHost() -> Element {
    let button = use_export_button();        // the seam: shapes the leaf's props
    rsx! {
        div {
            class: CLASS,                    // the container: owns the leaf's space
            ExportButton { ..button }
        }
    }
}
```

Because the container owns the space, the leaf never hard-codes a width or a
breakpoint. The Host's `style.rs` allocates the space per band and, for a leaf that
scales, marks the root a query container with `[container-type:inline-size]`; the
leaf then expresses every size in `cqi` (`text-[13cqi]`, `gap-[1.04cqi]`,
`border-[0.35cqi]`), so it fills whatever the container gives it at every width —
this is how `headed_grid` scales its whole tile grid off one container width. A leaf
that is intrinsically fixed (a single icon button) needs no `cqi`: its own root is
`contents` — a layout-neutral grouping wrapper — and the Host's bands stay empty
until a leaf needs the space carved up.

**`[container-type:inline-size]` is the container marker; `contents` is not.**
`contents` is `display: contents`: the element's own box disappears and its children
lay out as if they were the parent's direct children — a neutral grouping wrapper
that adds an identity class and nothing else. It establishes no query context. Only
`[container-type:inline-size]` does. Do not confuse the two: a Host that must size
its leaf carries `[container-type:inline-size]`, never `contents`.

### A component owns its look; its parent owns its size

This is the exact line the container split draws, and it is absolute:

> **Everything *inside* a component's box — proportions, radius, borders, colors,
> hover and focus treatment, how its glyph is centered — is owned by the component
> and is unreachable from outside. Everything *about* the box — how much space it
> gets, how large or small it is drawn — is owned by the parent that places it.**

A caller in the header can make a finished button 25px or 50px wide (and, when the
button is square, its height follows automatically) with total ease — that is a size
decision, and size belongs to the parent. A caller can **never** reach in and change
the button's radius, its icon centering, its text color, its hover glow, or make a
square button rectangular — that is look, and look is the component's alone. The
opaque `ClassList` already makes this mechanically impossible (no class prop, no
`"{CLASS} …"`), and this rule is why: a size prop or a style hole would re-open
exactly the coupling the opaque class closes. **Size flows through the box the parent
draws, never through a prop.**

Concretely, a self-sizing leaf like `ToolbarButton`:

- **never writes its own width or height** (`w-20`, `h-20`) into its `style.rs` —
  that is the parent stealing-back the size decision baked into the component. It
  **fills the box it is given** (`h-full`) and locks its own shape (`aspect-square`).
- **stays square in every viewport, never stretched.** With `aspect-square` +
  `h-full` + `w-auto` + `max-w-full`, centered by its container, the drawn side is the
  **smaller** side of whatever box it is handed — a wide box yields a square as tall as
  the box, a tall box a square as wide as the box — so no container shape can stretch
  it into a rectangle.
- **scales its interior like an SVG.** The button marks itself
  `[container-type:inline-size]` and expresses **every** interior length in `cqi` —
  border, radius, icon size, glow radii — off its own side. Make the box ten times
  larger and the whole interior — glyph, border, rounding, shadow — scales up in exact
  proportion, as one drawing, because there is not a single absolute length left
  inside. A `px` or `rem` anywhere in the interior is the bug: that part would refuse
  to scale.

The one knob the parent turns is the box. A whole row of buttons is sized uniformly
by one length on the toolbar (its row height); a single button is resized by its own
Host container overriding that box. Neither ever touches what is inside the button.

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
it. The identity is derived by `classes!` from the component directory — never
written by hand and never a const you list. e2e decides whether it bothers to
select a component, never whether the
component gets an identity — every component is selectable.

## Styling: shared values, individual looks

A component is styled with Tailwind utility classes and nothing else. There are
no per-component CSS files, no `asset!` stylesheets, no `styles/` directories,
no `document::Stylesheet` for project styling.

The line that matters is **what is shared and what is individual**. The one-line
rule, from which the rest follows: **share *values*, never *looks*.**

> **History, so the rule is not misread.** An earlier version of this section
> said the inverse — "a value goes in `@theme` only if it is a palette color or a
> font; everything else, inline it; when in doubt inline." That rule existed to
> escape a specific mess: globally interconnected CSS with no design system at
> all, where the only safe move was total component isolation. That north star is
> reached. The failure mode now is the **opposite** one — an archipelago of
> components that are each internally clean but collectively look like different
> applications. So the rule is refined below. Individualism is still allowed, but
> **limited**: the shared visual vocabulary is now mandatory, because a design
> system is the point.

- **The design vocabulary is global, on purpose — and mandatory.** The `@theme`
  block in `crates/hotkey-editor/tailwind.input.css` owns the shared design
  vocabulary: the color palette, the type scale (`--text-*`), the radius scale
  (`--radius-*`), the shadow set (`--shadow-*`), the line-heights
  (`--leading-*`), the recurring surface gradients (`bg-panel-*`), and the font —
  plus the six responsive bands and `kb-focus`. Every component draws its colors,
  font sizes, radii, and shadows **from this vocabulary**, so the whole app reads
  as one product. `text-lg` and `text-warcraft-gold` shared across a hundred
  components is the design system working, not a leak.
- **Sharing a value is not coupling; sharing a look is.** Two components that
  both use `text-lg` are **not** coupled — each still writes its own utility
  list, and editing one's markup cannot touch the other. What stays forbidden is
  promoting a component's **composite look** — a `surface-callout`, `chip-gold`,
  `button-primary` — into a shared `@utility`/class another component could
  *wear*, because then one edit restyles them all. That is the CSS-spaghetti
  coupling this project was built to escape, and it is still banned. **The test:
  share a *value* (a token, consumed through a utility); never share a *rule* (a
  selector or a composite look).**

**Individualism allowed, but limited.** In a tokenized dimension the token is
**mandatory**: a color, font size, radius, shadow, or line-height must be the
design-system token — `text-lg` not `text-[1.4rem]`, `rounded-card` not
`rounded-[8px]`, `text-warcraft-gold` (with `/opacity` modifiers) not a fresh hex
or `rgba`. Arbitrary `[…]` values remain the right tool for the genuinely bespoke
**non-vocabulary** value — a one-off layout dimension (`min-w-[24cqi]`,
`[&_svg]:size-8`, a `cqi`/`cqh` length off the component's container). Those stay
inline and private by construction. The distinction is the *dimension*: **visual
vocabulary (color, type, radius, shadow, gradient) is tokenized and shared;
structure and one-off geometry stay individual.**

**Consolidate — a scale is a constraint.** Tokenizing means *collapsing*, not
renaming: ~92 one-off font sizes become nine scale steps; a dozen shades of gold
become two or three. The count of distinct values in a dimension **going down**
is the design system appearing — a page needs several font sizes, not ninety-two.
A value that nearly matches a token snaps to it; only a genuinely new vocabulary
value earns a new token (and then everyone reuses it), never a one-off.

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

A component keys only the bands it styles (`base` plus whichever responsive bands
it restyles); an unused band is simply omitted, and an unknown band key is a
compile error. Within one band the property order is layout → sizing → spacing →
border → typography → color → effects → state.

## No `clamp()` — responsiveness is bands plus containers

The app has exactly two responsive mechanisms, and `clamp()` is not one of them.

- **Per viewport: the six bands.** A component picks the right token per band —
  `text-2xl` in `BASE`, `mobile:text-xl` where the phone needs it. Different
  widths get different *steps*, chosen explicitly, never interpolated.
- **Per container: `cqi`.** A self-scaling drawing leaf expresses its interior in
  `cqi`/`cqh` off the box its parent hands it, so it scales continuously with the
  container at every width (the header capstone is the worked example).

`clamp()` is a third mechanism that fights both. Its `vw` term interpolates
*across* the band boundaries the band system deliberately makes disjoint
("nothing inherits across bands"), and a `clamp` inside a `cqi` drawing pins the
proportional scaling that `cqi` exists to provide. It reads as "responsive" but
papers over bands that were never designed per width and containers that were
never wired — and it is a real complexity cost, three-part expressions where a
single token would do. **Design the bands and the container scaling correctly and
`clamp` is unnecessary, so it is not used.** A value that seems to want a
floor-and-ceiling is the signal that a band override or a `cqi` context is
missing; add that, not a clamp.

**The one sanctioned `vw` exception — a full-bleed on-screen keyboard.** The
system key-picker renders a complete keyboard whose keys must tile across the
full dialog width; its board is a full-bleed row, so — exactly like a `vw` on the
header *bar's own dimension* — its keys size in `vw` off that full width
(`w-[4.7vw]`, `text-[1.3vw]`). This is the single place a `vw` length is allowed
inside a key/leaf: the keyboard *is* the full-width surface it measures against,
and a `cqi` container would only re-express the same full-width proportion. No
other leaf gets this; a normal drawing leaf still scales in `cqi` off its box.

> **The capstones' `clamp` examples are superseded.** The app is now 100%
> clamp-free — including the role models: the header bar's height is per-band
> (`laptop:min-h-18` … `uhd:min-h-34`) and the footer's font knob is `text-xs`.
> The `shell/header` / `shell/footer` walk-throughs at the end of this file still
> *show* `clamp()` in their prose examples (`min-h-[clamp(…)]`, `text-[clamp(…)]`);
> read them for the structural lessons, but those specific `clamp` values are
> historical — the live rule is per-band steps + `cqi`, never `clamp`.

## Fill the container — a component defines shape, its parent defines scale

Sizing follows one rule: **a component fills the box its parent gives it and
scales like an SVG.** It declares its own *shape and structure* — aspect ratio,
internal proportions (`cqi`), how its parts sit relative to each other — and then
renders at whatever *scale* the parent hands down, preserving those proportions
fluidly. Exactly as an `<svg viewBox>` fills any box you place it in, a component
fills its slot: `w-full` / `h-full` / `size-full`, not a pinned pixel size. Only a
**viewport band** may declare a genuinely *new shape* (a different layout for the
phone than the desktop) — within a band there is no size step, just fill.

So the absolute scale lives *up* the tree, on the container that owns the
structure — the header bar's per-band `min-h`, a page column's per-band width, an
icon slot's per-band `size-*` — and every child below fills it. Push scale up;
never let a leaf pin its own render size.

**`min-*` / `max-*` are mostly the anti-pattern.** A `max-w-[260px]` on a card, a
`w-[72px]` on an icon, a `min-h-[16rem]` on a panel — each pins *scale* onto the
component, fighting the fill model. Replace them: the leaf becomes `size-full` /
`w-full`, and the per-band scale moves to the slot its parent owns. Two kinds of
`min`/`max` are **not** scale-pins and stay:

- **Layout plumbing** — `min-w-0`, `min-h-0`, `minmax(0,1fr)`. These *enable* fill
  and shrink (they let a flex/grid child collapse below its content); keep them.
- **Intrinsic shape** — a component's own form, not its scale. An accessible
  tap-target floor (`min-h-[44px]` on a button) and a readable line-length cap
  (`max-w-[90rem]` on a block of prose) describe *what the component is*, and by
  "the component defines its shape" they belong to the component. Keep these; they
  are the shape side of the line, not the scale side.

The test: does the value say *what proportions this thing has* (shape → keep) or
*how big it renders* (scale → dissolve into fill + a parent-owned per-band size)?

## style.rs and the `classes!` macro

Tailwind's scanner reads source as plain text and never evaluates code, so a
class name assembled at runtime (`format!`, a join, concatenation) is invisible
to it and its CSS is never generated. Every class token must therefore appear as
a literal in the source.

Each component writes one **keyed list per band it styles**, inline, in its own
`style.rs`; `classes!` derives the identity from the directory and joins
everything at compile time into a `pub(super) const CLASS: ClassList`. The keys
are `base` plus the responsive band names; a band the component does not restyle
is simply omitted (no empty placeholder), and the keys may appear in any order:

```rust
// help_top_row/style.rs — wide layout in `base`, the phone override per band.
// Bespoke values are arbitrary and inline (component-local); the gold is a token.
use tw_macro::tw;

classes! {
    base: tw!["flex", "flex-row", "items-start", "gap-[3.2rem]"],
    mobile: tw!["mobile:flex-col", "mobile:gap-[2.6rem]"],
    tablet: tw!["tablet:flex-col", "tablet:gap-[2.6rem]"],
    // laptop/desktop/qhd/uhd omitted — this component does not restyle them
}
// CLASS starts with the derived identity "help-top-row" (from the directory).
```

Each band value is a `tw![…]` list, not a raw array — the Tailwind LSP keys on
`tw![` to scope class completion and validation, and `tw!` re-anchors the
`TailwindClass` type so a stray `&[&str]` can't reach the macro. `classes!` and
`states!` are generated crate-globally by one
`tw_macro::define_styling! { bands: [mobile, tablet, laptop, desktop, qhd, uhd] }`
at the crate root, which declares the band vocabulary once; they are in scope
everywhere and need no `use`. `tw!` and `assert_component!` come from `tw_macro`
directly (`use tw_macro::tw;` / `use tw_macro::assert_component;`).

```rust
// help_top_row/mod.rs — body just names CLASS; assert_component! binds the name
use style::CLASS;
use tw_macro::assert_component;

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
- **Bands are keyed by name; only the ones you style appear.** A component lists
  `base` plus whatever responsive keys it restyles, in any order. There is no
  empty-placeholder ceremony, and `grep 'mobile:'` still lists every component's
  mobile styles because every mobile class is literally `mobile:`-prefixed.
- The macro guards the whole contract at compile time: `base` rejects a class
  carrying any declared band prefix (a width style belongs in that band, not
  `base` — but variant prefixes like `after:`/`hover:` are fine), a non-`base`
  key must name a **declared** band (a typo like `moble:` fails the build), and
  every class in a band must carry that band's prefix — `uhd:flex` under `mobile:`
  fails the build.
- Each utility is a separate literal, so rustfmt-style one-per-line arrays keep
  Tailwind's scanner seeing every token verbatim.
- `classes!` joins them in a `const fn` into one string at **compile time** —
  zero runtime cost; the body only names `CLASS`.
- `CLASS` is a `pub(super)` **`ClassList`**, not a `&str`. `mod style;` is
  private, so no other component can name the path; and `ClassList` implements no
  `Display` and no accessor, so a component cannot interpolate or append to it
  (`class: "{CLASS} other-class"` does not compile). A component can only ever
  wear exactly its own class — styling coupling is impossible to express.

The engine lives in the standalone `tw-macro` crate; `ClassList`/`TailwindClass`
are imported from it directly (`use tw_macro::ClassList;`). `classes!`/`states!`
are generated crate-global by `define_styling!` at the crate root, and their
`const fn` helpers ship with the crate.

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
use tw_macro::tw;

classes! {
    base: tw!["relative", "flex", "items-center"],
    // ... plus the bands that carry the tile's sizing ...
}

states! {
    TileState,
    Idle => tw![],
    DragSource => tw!["opacity-40", "ring-2", "ring-warcraft-gold"],
    DropTarget => tw!["bg-warcraft-gold-dim"],
}
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

Both macros are generated crate-global by `define_styling!` (from the `tw-macro`
crate); their `const fn` helpers ship with that crate. State overlays are keyed
by variant with inline literal lists, exactly like `classes!`.

---

## The directory layout

Every component directory follows the same shape. `grid_tile` is the canonical
example:

```
grid_tile/
  mod.rs            the component function, flat hooks then pure RSX, plus the pub use re-exports
  props.rs          the Props struct and its From conversions
  data.rs           the static content/data this component sources (optional)
  hooks.rs          the component's composed hook, wiring primitive hooks together (optional)
  logic.rs          everything the body is not allowed to do (optional)
  state.rs          component-local enums, e.g. visual state (optional)
  style.rs          the per-band class arrays, via classes!
  components/        child components, each its own directory of this same shape
```

A component with children nests them under `components/` — and **every** component
it renders is such a child, per "The render tree IS the directory tree". A leaf
component omits `components/`. A component with no logic beyond `From` conversions
omits `logic.rs`. A component that reaches the domain, `localStorage`, or a web API
carries a `hooks.rs` with its one composed hook, and omits it otherwise.

## Data and content are props, sourced from `data.rs`

A component renders; it does not own content. Copy, lists, labels, the entries of
a menu — none of it is baked into markup. Data is **a prop, strictly**, threaded
in from the parent, so a renderer is a pure loop over what it is handed and never
hard-codes a sentence or an entry.

The static content itself lives in a **`data.rs` next to the markup** of the
component that sources it — the top of a subtree owns the `data.rs`, builds the
content value, and passes it down as props; every component below renders it.
A `HelpWorkflow` does not contain its fourteen steps; the steps are a
`&'static [&'static [HelpSegment]]` in `help_dialog/data.rs`, handed down and
looped over. This keeps content out of the render path: changing a label is an
edit to `data.rs`, never to a component body, and the same renderer serves any
data of its shape.

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

This is **exception 2** to "The render tree IS the directory tree", and it applies
*only* when a variant wrapper binds the base's generic **behavior** type
parameter. Outside that, a component you render is your child — nest it. There is
no "fills a slot/body the base exposes" variant: a base that receives a body is
`children: Element`, forbidden.

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

The variant pattern above is for a base that is generic over a **behavior**, where
variants bind the type parameter (`CommandGridEditor` binds `GridEditor`'s
`GridBehavior`). Binding a behavior type is the *only* thing a base is for.

**A base is never a shell that receives a body.** There is no "the base owns the
chrome and each variant fills the body" pattern — that is `children: Element` with
extra steps, the exact anti-pattern this document forbids. A reused *frame* — a
dialog shell, a card surface, a scrolling panel — is **not** a base. Each concrete
instance owns its own shell markup, sharing the shell's utility-class **values**
(never a wrapper that swallows a body) and nesting shared **leaves** by name. That
is how `shell/header` and `shell/footer` — the only sanctioned role models — are
built: they receive no markup, they *name* their typed children. When a dialog,
card, or panel tempts you toward a body-slot base, do what the header does instead.

A small shared piece that is not generic over a behavior is not a variant. A
close button, a primary button, an edit panel are plain leaf components. They
live once, own their class and CSS, and parents reuse them by nesting them in
the tree, the way a toolbar drops in `ToolbarButton`. Do not force a behavior
parameter onto a button to make it look like a variant. Extend a base when there
is a behavior to bind, compose a leaf when there is not.

## Shared leaves live in a `shared/` grouping directory

This is **exception 1** to "The render tree IS the directory tree", and the test
for it is a pure count: a leaf with a single renderer nests under that renderer; a
leaf rendered by **two or more** sibling components uses `shared/`. Never flat-dump
a shared leaf beside its renderers, and never nest it under just one of several.

A leaf used verbatim by several sibling components is neither duplicated nor
flat-dumped among those siblings. It lives once in a **`shared/` grouping
subdirectory at their common parent**, and each sibling reaches it by its full
module path.

`shared/` is an organizational module, not a component: its `mod.rs` carries only
`pub mod` entries for the leaves (and any trait) it groups, propagating each on a
public path, and it has no component function, no class, and no `style.rs`. This is
the sanctioned way to keep a genuinely-shared leaf out of the flat component list —
the opposite of the flat-dumping the render-tree rule forbids.

It is distinct from the forbidden `base/` / `extensions/` layers above: those would
split a base and its variants across a grouping level and break
`directory == component`. `shared/` names no component at all; it groups what several
sibling components share, so it does not violate `directory == component == class`.

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

The domain owns the computed, normalized value; the renderer owns how that value is
**presented**. Convert to the display string at the leaf that renders it, with
whatever precision, unit, or sign the UI wants (`{:.0}`, a trailing `%`, a leading
`+`). That formatting is the UI's prerogative and is never required to live in the
domain crate — the UI decides how it draws a normalized domain value.

When one leaf renders several different domain figures and cannot give each its own
`Display` here — the orphan rule forbids implementing `std::fmt::Display` for a
foreign domain type in the renderer crate — a **renderer-local presentation trait**
(one method per figure type returning its display text) is the sanctioned home for
that formatting. It is not a `to_string` anticonvention; it is the UI deciding
presentation for types it does not own. `Display` in the domain crate is allowed but
not required; if a domain type already implements it (as `DefenseType` and
`AttackType` do), the leaf uses that instead of re-wrapping it.

Whether a figure renders muted — a mana of zero, a regeneration of zero — is decided
at the leaf, and where the muted state is "the value is zero" it asks the domain type
(`value.is_zero()`) rather than threading a parallel `is_zero: bool` prop.

See `docs/ARCHITECTURE.md` for why the renderer never computes domain decisions,
and `docs/RUST_STYLE.md` for the `From` and `TryFrom` rules these conversions
follow.

---

## The reference implementation — `shell/header`

`components/shell/header` is the role model. It is the one subsystem taken through
every rule in this file *and* the full styling, container-query, and responsiveness
treatment, and approved end-to-end. Read it before writing any component; when your
shape differs from the header's, the header is right. Each quality below is a rule
you copy, not a suggestion — this is the standard, and lower quality is not accepted.

**The render tree is the file tree, to the leaf.** `Header` renders `BrandHost`,
`GridLayoutButtonHost`, `Toolbar`; each lives under `header/components/`. It holds at
every depth — `Toolbar` → `ToolbarActions` → `InlineActions` → nine action buttons,
and `BurgerMenu` → `BurgerDrawer` → `BurgerDrawerBody` → … — and every `mod.rs`
reaches its children with `use components::…`, never `super::`. Run the `super::`
test on any file in the tree and it passes.

**Connected hosts feed presentational leaves; nothing drills props.** A `<Leaf>Host`
calls one composed hook, shapes the result, and spreads it into a pure leaf.
`CollisionsButtonHost` (`hooks.rs::use_collisions_button`) asks the domain for the
collision count and hands `CollisionsButton { ..button }` its summary and click
handler; `CollisionsButton` is props-in / markup-out and reads no context. The host's
classed `div` is also the leaf's box and its query container. Uninvolved containers
thread nothing: `Toolbar` and `ToolbarActions` are pure layout and pass no document
down. Because the leaf is pure, the gallery renders it directly with any value.

**Static content lives in `data.rs` and arrives as typed props.** `brand/data.rs`
holds `const TITLE: BrandTitleProps = BrandTitleProps { title: "Warcraft III Hotkey
Editor" }`, and `Brand` renders `BrandTitle { ..data::TITLE }`. No sentence is ever
baked into a body — the copy is a typed constant in `data.rs`, spread in as props.

**One shared leaf, many thin connected wrappers.** `ToolbarButton` lives once at
`inline_actions/components/shared/toolbar_button/` and is reused by all nine action
buttons. Each button — `ExportButton`, `UndoButton`, `TemplatesButton`, … — is a thin
wrapper (`hooks.rs`) that sources only its own state and renders the shared
`ToolbarButton`. It is in `shared/` because the count of render sites is ≥ 2; it is
never duplicated and never flat-dumped beside its renderers.

**A component is a drawing that scales off its box — `cqi`, never `px`/`rem`/`vw`
inside a leaf.** The header is a query container (`@container`), and every interior
length in a leaf is a `cqi` fraction of the box its parent hands it. `ToolbarButton`
fills the height it is given (`size-full`), locks itself square (`aspect-square`),
marks itself a container, and its surface expresses border, radius, icon, and glow
entirely in `cqi` — enlarge the box and the whole button scales in exact proportion,
as one drawing, because there is not a single fixed length left inside. A `px`, `rem`,
or `vw` inside a leaf is the bug.

**Size flows through the box: the parent owns it, the child fills it.** The header
owns exactly two knobs — the bar height (`min-h`) and the button-to-bar ratio (`py`) —
and `items-stretch` hands every column the same row height. The layout button and
every toolbar button fill that height and come out identical; no leaf writes its own
width or height. Change one knob and the whole bar rescales together, still centered,
still proportional.

**Per-band proportions: `BASE` is the common truth, the bands are deltas.** `BASE`
carries the shared appearance — here the laptop-and-up default, since it covers four
of the six bands — and the touch bands override only what genuinely differs. A `BASE`
value that every band overrides is dead weight: delete it. `vw` appears only on the
bar's *own* dimensions (`min-h`, `gap`, `py`) — the bar is full-bleed, so `vw` there
is just a fraction of its own width. The bar's `min-height` is an explicit
**per-band step** (`laptop:min-h-18` growing to `uhd:min-h-34`) — chosen per band,
never a `clamp` (see the no-`clamp` rule above). No `clamp` appears anywhere, and
nothing pins the `cqi` drawings with a floor or cap that would break their
proportional scaling.

**Canonical, LSP-validated classes.** Reach for the real Tailwind utility or theme
token before a raw arbitrary value: `@container` (not `[container-type:inline-size]`),
`aspect-39/10`, `grid-cols-[…]`, `z-60`, `bg-warcraft-gold-soft`, `bg-fixed`,
`outline-offset-2`. Arbitrary `[…]` values are for the genuinely bespoke; if the
utility exists and the Tailwind LSP knows it, use it — that is the difference between
a class the tooling can validate and one it cannot.

**Small, single-purpose, composable files.** Each component is one class, one concern,
a handful of lines: `brand_title`, `collisions_button_icon`, `collisions_button_badge`,
`toolbar_button_surface`, `toolbar_button_icon` are each their own leaf. The moment a
file wants a second class or a second responsibility it wants to be two components.
There is no such thing as too many.

Read these files before you write. When in doubt, do what the header does.

---

## The second reference — `shell/footer`

`components/shell/footer` is the header's counterpart, converted the **exact same
way** and approved end-to-end. Where the header is a bar of buttons, the footer is
a full-bleed bar of *text* — the credit line, the outbound links, the trademark
disclaimer — so it proves the same model holds on that axis. Read it beside the
header when your component is text rather than controls; it is a shorter walk
because the same rules produce a smaller tree.

**A full-bleed bar owns its defining dimension as one token.** The header's
defining dimension is its height (a per-band step, `laptop:min-h-18` …
`uhd:min-h-34`); the footer's is its font size (`text-xs`) — the same single-knob
idea, one axis over. The footer is fine print, so that one small type step holds
flat across the whole width range — legible on a phone, never ballooning into a
banner on 4K. That single font token is the footer's one knob.

**Leaves scale in `em` off that knob — never `px`/`rem`/`vw` inside a leaf.** Every
glyph, gap, and icon expresses its length in `em` (`w-[1.15em]` for the heart and
link glyphs, `gap-[0.4em]`, `text-[0.82em]` for the disclaimer), so they track the
bar's font as one drawing — the text counterpart of the header leaves' `cqi`. `em`
is exactly the header's blessed choice for a length that should follow the font
(the brand's `tracking-[0.04em]`), applied here to the whole leaf — down to the
heart's glow radius (`drop-shadow-[0_0_0.3em_…]`), so not a single fixed length is
left inside a leaf. The footer is a query container (`@container`) like the header
bar, so any leaf that later needs to measure the bar in `cqi` still can.

**Size flows through the box: the bar owns it, the leaf fills it.** No leaf writes a
size of its own; each sizes its parts as `em` fractions of the one font the bar
hands down. Change the `text-xs` knob and the credit, heart, link icons,
separators, and disclaimer all rescale together, still centered, still wrapping.

**There are no per-band overrides — the whole footer lives in `BASE`.** Unlike the
header, whose touch bands genuinely differ, the footer renders the same at every
width: one font token, one uniform padding, one vertical rhythm, all in `BASE`, with
`MOBILE` through `UHD` empty. It needs no safe-area insets because the shell drops
`viewport-fit=cover`, so the browser keeps the whole app clear of device edges and no
shell component reaches for `env(safe-area-inset-*)`. This is the lesson the footer
adds to the header's rule that `BASE` is the common truth: a band array earns entries
only when the width *genuinely* changes something — and when nothing changes, `BASE`
is the entire component.

**Every structural rule the header follows, the footer follows too.** The render
tree is the directory tree to the leaf (`footer` → `footer_credit` →
`footer_heart`; the `super::` test passes on every `mod.rs`). The leaves are
presentational, fed typed consts from `data.rs` and spread with `..`, with no
domain state and no context — so the gallery renders `Footer` directly. Each file
is one class, one concern.

---

## Verify every pass

A component change is not done until all of these are green:

```
nix develop -c cargo clippy -p hotkey-editor -p gallery --target wasm32-unknown-unknown
nix develop -c cargo test -p warcraft-keybinds
nix develop -c cargo fmt --check
```

Also confirm the global layer stayed clean: no new `@utility` for a component's
look, and `@theme` gained nothing that is not shared design vocabulary — bespoke
values are arbitrary and inline in the component's `style.rs`. A class assembled
outside a literal is invisible to Tailwind's scanner, so its CSS will silently
never generate — keep every utility a literal in a `style.rs` array.
