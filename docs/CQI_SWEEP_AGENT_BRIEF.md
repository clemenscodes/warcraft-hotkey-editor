# CQI sweep — Phase 1 brief: CREATE THE HOSTS (mechanical, copy-driven)

You are one agent in a swarm. **This brief has exactly one job: create Host
components.** Not "convert a subsystem." Not "change spacing." Not "decide `cqi`
vs `em`." You wrap a fixed list of named leaves in Host wrappers by **copying an
existing Host and renaming it**. That is the entire task. It is `cp` + rename +
`git mv` + rewire three lines. There is no judgment in it and no code to invent.

> **Why this brief exists.** Earlier briefs bundled host-creation together with
> the `px → cqi/em` interior conversion. Every agent then satisfied "done" with
> the cheap class edits and quietly skipped the hosts — the exact thing the sweep
> needs. So host-creation is now its own pass with **nothing cheap to hide behind**:
> your deliverable is *new `*_host/` directories that exist and compile*, and
> nothing else counts. Interior `cqi`/`em` conversion is a **separate later brief**
> (§7) — do NOT do any of it here.

---

## 1. What you are handed (and what you are NOT)

You are handed, by the coordinator, an **explicit list of leaves to wrap**, each
as an absolute component directory path. Example:

```
LEAVES TO WRAP THIS TASK:
  - crates/.../unit_stats_panel/components/stat_value
  - crates/.../unit_stats_panel/components/stat_label
```

You are **NOT** handed "a subsystem to survey." You do not decide *which* leaves
get hosts — that decision was already made by the coordinator (§6). You do not
decide `cqi` vs `em` — that is not part of host-creation at all. If your task list
is empty or a path does not exist, STOP and report it; do not go looking for work.

There is **no `em` option and no `// CQI-FLAG` in this brief.** Those belong to the
interior-conversion pass. Here, every listed leaf gets a Host. Full stop.

## 2. The done-condition — grep-checkable, no self-grading

Your work is done when, and only when, ALL of these are literally true:

1. For every leaf `<parent>/components/<leaf>` in your list, a directory
   `<parent>/components/<leaf>_host/` now exists, and the leaf now lives at
   `<parent>/components/<leaf>_host/components/<leaf>/` (moved via `git mv`).
2. `moon run :check` is **green**.
3. `grep -rn "super::.*::<Leaf>" ` for each moved leaf returns nothing — the
   `super::` test passes (the leaf is reached via `components::…`, never `super::`).

A report with zero new `*_host/` directories is a **failure**, not a status update.
"I analyzed the subsystem and it mostly needs `em`" is a failure. The only success
is directories that exist and compile.

---

## 3. THE REFERENCE YOU COPY — `toast_close_host` (props-carrying)

This is a real, in-tree, compiling Host with props. **You copy it.** Its host-level
directory is:

```
crates/hotkey-editor/src/components/app/components/shell/components/toasts/components/toast_overlay/components/toast_list/components/toast_list_item/components/toast/components/shared/toast_card/components/toast_close_host/
```

Its five host-level files, verbatim — this is the shape you reproduce:

```rust
// toast_close_host/mod.rs
pub mod components;
mod model;
mod view;
pub use view::ToastCloseHostView;
mod style;

use components::toast_close::ToastClose;
use dioxus::prelude::*;
use model::ToastCloseHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ToastCloseHost(props: ToastCloseHostModel) -> Element {
    let id = props.id;                 // one line per field, copied from the leaf's model
    let on_remove = props.on_remove;
    rsx! {
        div {
            class: CLASS,
            ToastClose { id, on_remove }   // forward every field to the leaf, named
        }
    }
}
assert_component!(ToastCloseHost);
```
```rust
// toast_close_host/style/mod.rs   — THE DEFAULT HOST BOX for this pass
use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "size-full",
        "min-w-0",
    ],
}
```
```rust
// toast_close_host/view/mod.rs    — fields are a VERBATIM COPY of the leaf's own view fields
use dioxus::prelude::*;
#[derive(Clone, PartialEq)]
pub struct ToastCloseHostView {
    pub id: usize,
    pub on_remove: Callback<usize>,
}
impl ddd::View for ToastCloseHostView {}
```
```rust
// toast_close_host/model/mod.rs   — fields verbatim from the leaf's model; From<&View>; ddd::Model
use super::view::ToastCloseHostView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ToastCloseHostModel {
    pub id: usize,
    pub on_remove: Callback<usize>,
}
impl From<&ToastCloseHostView> for ToastCloseHostModel {
    fn from(view: &ToastCloseHostView) -> Self {
        let ToastCloseHostView { id, on_remove } = view.clone();
        Self { id, on_remove }
    }
}
impl ddd::Model for ToastCloseHostModel { type View = ToastCloseHostView; }
```
```rust
// toast_close_host/components/mod.rs
pub mod toast_close;
```

**The host is a pass-through wrapper: its `View`/`Model` fields are an exact copy
of the leaf's own `View`/`Model` fields, and its `mod.rs` forwards every field to
the leaf unchanged.** The Host adds nothing but a classed `@container` `div`.

**If the leaf is parameterless** (no `view/`, no fields), copy the empty form from
`shell/header/components/brand_host` instead: an empty `FooHostView;` / `FooHostModel;`
and a body that renders `Foo {}` with no fields. Everything else is identical.

## 4. THE RECIPE — per leaf, exactly these steps

For a leaf at `<parent>/components/<leaf>` whose component is `<Leaf>` (PascalCase):

1. **Copy the reference host dir** to `<parent>/components/<leaf>_host/` (host-level
   files only: `mod.rs`, `style/mod.rs`, `view/mod.rs`, `model/mod.rs`,
   `components/mod.rs`). Do NOT copy `toast_close`'s own leaf.
2. **Rename identifiers** in those five files: `ToastCloseHost` → `<Leaf>Host`,
   `toast_close` → `<leaf>`, `ToastClose` → `<Leaf>`. (`sed -i` is fine.)
3. **Replace the field lists** in the new `view/mod.rs` and `model/mod.rs` with a
   **verbatim copy of the leaf's own `view/mod.rs` / `model/mod.rs` fields**, and
   update the `mod.rs` body to bind one `let` per field and forward them all to
   `<Leaf> { ..fields }`. Parameterless leaf → empty form (§3).
4. **`git mv` the leaf under the host:** `git mv <parent>/components/<leaf>
   <parent>/components/<leaf>_host/components/<leaf>`.
5. **Rewire the parent** (`<parent>/mod.rs` + `<parent>/components/mod.rs`) — three
   lines, copied from how `toast_card` renders `ToastCloseHost`:
   - `<parent>/components/mod.rs`: `pub mod <leaf>;` → `pub mod <leaf>_host;`
   - `<parent>/mod.rs` import: `use components::<leaf>::<Leaf>;` →
     `use components::<leaf>_host::<Leaf>Host;`
   - `<parent>/mod.rs` RSX call: `<Leaf> { …fields }` → `<Leaf>Host { …fields }`
     (the same named fields — the Host forwards them).
6. **Leave the leaf's own code UNCHANGED.** No class edits, no `cqi`, no `em`. The
   leaf renders exactly as before, one `div` deeper. (Interior conversion is §7.)

Then `moon run :check`. Fix compile errors, re-check until green. Move to the next
leaf. Do not stop until every listed leaf has its host and the build is green.

## 5. The four ways this breaks a compile (and the fix)

1. `assert_component!(<Leaf>Host)` fails → the dir name, the `fn` name, and the
   kebab class must all agree: dir `foo_host`, fn `FooHost`. Rename the one that's off.
2. A `super::` import for the moved leaf → you rewired the parent wrong; the leaf is
   now the host's child, reached `use components::<leaf>::<Leaf>;` *inside the host*,
   and the host is reached `use components::<leaf>_host::<Leaf>Host;` *inside the parent*.
3. Missing `pub mod <leaf>_host;` in `<parent>/components/mod.rs` → module not found.
4. Field mismatch → the Host's `Model` fields must match the leaf's `Model` fields
   exactly, and the body must forward each one. Copy them; do not improvise names.

## 6. FOR THE COORDINATOR — how to dispatch so agents can't dodge

The dodge you are preventing is "agent does cheap class edits, skips the hosts."
You prevent it structurally:

- **Pre-decide the leaf list yourself.** Walk the target subsystem, list the leaves
  that will get hosts (a leaf that sits in a definite-width box: a page column, a
  grid track, a dialog panel, a fill slot). That host-vs-`em` judgment happens ONCE,
  here, by you — never inside the swarm. Hand each agent a flat list of leaf paths.
- **One agent = a handful of named hosts**, never "a subsystem." The unit is
  countable and its done-state is `ls`-checkable.
- **Verify agents mechanically**, not by their prose: after each agent,
  `find <subsystem> -type d -name '*_host'` must have grown by exactly the leaves you
  assigned, and `moon run :check` is green. If the count didn't grow, the agent
  failed regardless of what its report says.

## 7. Phase 2 (NOT this brief): interior conversion

Only after the hosts for a subsystem exist and compile does the interior pass run —
a **separate brief**: move each leaf to `size-full` and express its interior in `cqi`
off its host (rule 4 of the old decision tree), or, for a genuinely content-sized
leaf, `em` off a font knob (the footer model). That pass is where `cqi` magnitudes
and the `em`-vs-`cqi` call live. **Do none of it here.** Mixing it back in is exactly
what let agents skip the hosts. Keep this pass pure: hosts exist, build green, done.
