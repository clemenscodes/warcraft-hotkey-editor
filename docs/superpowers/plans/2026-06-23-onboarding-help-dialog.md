# Onboarding and Help dialog Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development (recommended) or executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Give first time visitors a Help dialog that explains the editor's intended flow and labels every header control, opened automatically on the first visit and reachable forever after from a Help button.

**Architecture:** Pure renderer work. A new persistence helper owns a single localStorage flag that records whether the user opted out of the auto open. The app declares a `help_open` signal that starts true only when the flag is absent. A new `HelpDialog` component, built on the same dialog primitives as `PreviewDialog`, renders the content. A Help button is added to both the desktop toolbar and the mobile burger menu, since the toolbar is hidden below 1500px.

**Tech Stack:** Rust, Dioxus, `dioxus_primitives::dialog`, Tailwind utility classes, hand written component CSS loaded through `document::Stylesheet`.

## Global Constraints

- The wall: this feature is renderer only. No calls into `warcraft_keybinds`, no access to `CustomKeysFile`, no `binding.set_*`. See `docs/ARCHITECTURE.md`.
- localStorage is read and written only through the `LocalStorage` wrapper in `crates/hotkey-editor/src/services/storage/local_storage.rs`.
- Rust style from `docs/RUST_STYLE.md`: full semantic names, no tuples, no `as` casts, no numeric suffixes, private fields accessed only within their module, `Self` inside impl blocks, derive every trait the type qualifies for, no section header comments.
- Prose rule for every user facing string: no hyphens, no semicolons, no em dashes. Plain human sentences. This applies to all dialog copy and legend text exactly as written in this plan.
- Run all tooling through the dev shell: `nix develop -c <command>`.
- After editing any `.css` file, run `nix develop -c moon run hotkey-editor:tailwind/build`, then reload the browser. `dx serve` does not compile CSS.
- Finish state: `nix develop -c moon run :ci` is green, including the Playwright smoke gate.

---

### Task 1: Onboarding persistence helper

Adds the localStorage flag helper next to `CustomKeysPersistence`. The only branching logic, mapping a stored value to a seen boolean, is a pure associated function so it can be unit tested on the native target where `LocalStorage` is a no-op.

**Files:**
- Modify: `crates/hotkey-editor/src/services/customkeys/persistence.rs`

**Interfaces:**
- Produces:
  - `OnboardingPersistence` (unit struct)
  - `OnboardingPersistence::has_been_seen() -> bool`
  - `OnboardingPersistence::mark_seen()`
  - private `OnboardingPersistence::seen_from_stored(stored: Option<String>) -> bool`

- [ ] **Step 1: Write the failing test**

Append this test module to the end of `crates/hotkey-editor/src/services/customkeys/persistence.rs`:

```rust
#[cfg(test)]
mod onboarding_tests {
    use super::OnboardingPersistence;

    #[test]
    fn absent_value_is_not_seen() {
        let stored = None;
        let result = OnboardingPersistence::seen_from_stored(stored);
        assert!(!result);
    }

    #[test]
    fn exact_true_value_is_seen() {
        let stored = Some(String::from("true"));
        let result = OnboardingPersistence::seen_from_stored(stored);
        assert!(result);
    }

    #[test]
    fn other_values_are_not_seen() {
        let stored = Some(String::from("false"));
        let result = OnboardingPersistence::seen_from_stored(stored);
        assert!(!result);
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `nix develop -c cargo test -p hotkey-editor onboarding_tests`
Expected: FAIL to compile with an error that `OnboardingPersistence` is not found in `super`.

- [ ] **Step 3: Write minimal implementation**

In the same file, after the existing `GRID_LAYOUT_STORAGE` constant add a new constant pair, and after the `CustomKeysPersistence` impl block add the new helper. The full file becomes:

```rust
use crate::model::grid::GridLayout;
use crate::services::storage::local_storage::LocalStorage;

const CUSTOM_KEYS_STORAGE: LocalStorage = LocalStorage::new("warcraft-hotkey-editor.custom-keys");
const GRID_LAYOUT_STORAGE: LocalStorage = LocalStorage::new("warcraft-hotkey-editor.grid-layout");
const ONBOARDING_SEEN_STORAGE: LocalStorage =
    LocalStorage::new("warcraft-hotkey-editor.onboarding-seen");
const ONBOARDING_SEEN_VALUE: &str = "true";

pub(crate) struct CustomKeysPersistence;

impl CustomKeysPersistence {
    pub(crate) fn load_text() -> Option<String> {
        CUSTOM_KEYS_STORAGE.get()
    }

    pub(crate) fn save_text(text: &str) {
        CUSTOM_KEYS_STORAGE.set(text);
    }

    pub(crate) fn load_grid_layout() -> Option<GridLayout> {
        let raw_value = GRID_LAYOUT_STORAGE.get()?;
        GridLayout::try_from(raw_value.as_str()).ok()
    }

    pub(crate) fn save_grid_layout(layout: GridLayout) {
        let contents = layout.to_storage_string();
        GRID_LAYOUT_STORAGE.set(&contents);
    }
}

pub(crate) struct OnboardingPersistence;

impl OnboardingPersistence {
    pub(crate) fn has_been_seen() -> bool {
        let stored = ONBOARDING_SEEN_STORAGE.get();
        Self::seen_from_stored(stored)
    }

    pub(crate) fn mark_seen() {
        ONBOARDING_SEEN_STORAGE.set(ONBOARDING_SEEN_VALUE);
    }

    fn seen_from_stored(stored: Option<String>) -> bool {
        let stored_value = stored.as_deref();
        stored_value == Some(ONBOARDING_SEEN_VALUE)
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `nix develop -c cargo test -p hotkey-editor onboarding_tests`
Expected: PASS, three tests pass.

- [ ] **Step 5: Stage and report for signing**

Per the project rule, do not commit. Stage the file and tell the user it is ready to sign.

```bash
git add crates/hotkey-editor/src/services/customkeys/persistence.rs
git status --short
```

---

### Task 2: Help icon and HelpDialog component

Adds the question mark icon and the dialog component. The component compiles on its own. It is wired into the app in Task 3.

**Files:**
- Modify: `crates/hotkey-editor/src/components/shared/icons/mod.rs`
- Create: `crates/hotkey-editor/src/components/dialogs/help_dialog/mod.rs`
- Create: `crates/hotkey-editor/src/components/dialogs/help_dialog/help_dialog.css`
- Modify: `crates/hotkey-editor/src/components/dialogs/mod.rs`
- Modify: `crates/hotkey-editor/src/components/dialogs/dialogs.css`
- Modify: `crates/hotkey-editor/tailwind.input.css`

**Interfaces:**
- Consumes from Task 1: `OnboardingPersistence::mark_seen()`.
- Produces: `HelpDialog` component with `HelpDialogProps { help_open: Signal<bool> }`.

- [ ] **Step 1: Add the Help icon**

Append to `crates/hotkey-editor/src/components/shared/icons/mod.rs`:

```rust
pub(crate) const ICON_HELP: &str = r##"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="10"/><path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>"##;
```

- [ ] **Step 2: Register the new module**

Add this line to `crates/hotkey-editor/src/components/dialogs/mod.rs`, keeping the list alphabetical:

```rust
pub(crate) mod help_dialog;
```

The file becomes:

```rust
pub(crate) mod dialog_header;
pub(crate) mod dialog_stack;
pub(crate) mod download_info_dialog;
pub(crate) mod help_dialog;
pub(crate) mod layout_editor;
pub(crate) mod preview_dialog;
pub(crate) mod templates_dialog;
pub(crate) mod upload_info_dialog;
```

- [ ] **Step 3: Create the component**

Create `crates/hotkey-editor/src/components/dialogs/help_dialog/mod.rs`:

```rust
use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};

use crate::components::dialogs::dialog_header::DialogHeader;
use crate::components::shared::icons::{
    ICON_COG, ICON_COLLISIONS, ICON_DOWNLOAD, ICON_GRID, ICON_HELP, ICON_PREVIEW, ICON_RESOLVE,
    ICON_TEMPLATES, ICON_UNDO, ICON_UPLOAD,
};
use crate::services::customkeys::persistence::OnboardingPersistence;

const HELP_DIALOG_STYLES: Asset = asset!("/src/components/dialogs/help_dialog/help_dialog.css");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct TaskBlock {
    heading: &'static str,
    body: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct LegendEntry {
    icon: &'static str,
    label: &'static str,
    description: &'static str,
}

const WORKFLOW_STEPS: [&str; 6] = [
    "The editor loads the Warcraft III default keybinds, or a template if you applied one.",
    "It automatically finds every hotkey and position collision.",
    "You open the Collisions and Resolve pages to settle each conflict.",
    "You define a global grid layout and apply it to all units.",
    "You search for a unit and drag abilities onto the grid to rebind them.",
    "You export your CustomKeys.txt file.",
];

const TASK_BLOCKS: [TaskBlock; 4] = [
    TaskBlock {
        heading: "Applying a template",
        body: "A template is a complete keybind set that someone prepared in advance. Applying one replaces all of your current keys with the keys from the template. The editor then normalizes the result, which means it resolves the cascades and settles the collisions on its own. Reach for a template when you want a solid starting point instead of the Warcraft III defaults.",
    },
    TaskBlock {
        heading: "Resolving conflicts",
        body: "After you load the defaults or a template, some commands will clash. Two abilities might ask for the same hotkey. Two units might want the same grid cell. The editor finds all of these for you. The Collisions page shows every clash so you can see what is wrong. The Resolve page settles them in bulk by shifting and swapping bindings along the cascade until nothing conflicts. Keep going until the editor reports a clean state.",
    },
    TaskBlock {
        heading: "Applying the grid to all units",
        body: "You do not have to bind each unit by hand. Instead you define one global grid layout, which decides which command lives in which cell of the command card. You then apply that layout to every unit at once. The editor rewrites the positions and hotkeys across all units, so the whole game shares one consistent layout that is easy to learn.",
    },
    TaskBlock {
        heading: "Fine tuning with drag and drop",
        body: "Use this for small adjustments after the global grid is applied. Search for a unit, then drag an ability from one grid cell to another. When you drop it, the editor rewrites that ability's hotkey to match its new cell. This is how you handle the units that need a layout slightly different from the global one.",
    },
];

const LEGEND_ENTRIES: [LegendEntry; 10] = [
    LegendEntry {
        icon: ICON_GRID,
        label: "Grid Layout",
        description: "Edit the global grid template that every unit can share.",
    },
    LegendEntry {
        icon: ICON_COLLISIONS,
        label: "Collisions",
        description: "Jump to the conflicts the editor found.",
    },
    LegendEntry {
        icon: ICON_TEMPLATES,
        label: "Templates",
        description: "Apply a prepared keybind set.",
    },
    LegendEntry {
        icon: ICON_UPLOAD,
        label: "Upload",
        description: "Import a CustomKeys.txt file from your computer.",
    },
    LegendEntry {
        icon: ICON_COG,
        label: "System Hotkeys",
        description: "Edit the system and menu hotkeys.",
    },
    LegendEntry {
        icon: ICON_RESOLVE,
        label: "Resolve",
        description: "Open the conflict resolver.",
    },
    LegendEntry {
        icon: ICON_PREVIEW,
        label: "Preview",
        description: "See the text the editor will export.",
    },
    LegendEntry {
        icon: ICON_DOWNLOAD,
        label: "Export",
        description: "Download your CustomKeys.txt file.",
    },
    LegendEntry {
        icon: ICON_UNDO,
        label: "Undo and Redo",
        description: "Step backward and forward through your changes.",
    },
    LegendEntry {
        icon: ICON_HELP,
        label: "Help",
        description: "Reopen this guide at any time.",
    },
];

#[derive(Props, Clone, PartialEq)]
pub(crate) struct HelpDialogProps {
    pub(crate) help_open: Signal<bool>,
}

#[component]
pub(crate) fn HelpDialog(props: HelpDialogProps) -> Element {
    let mut help_open = props.help_open;
    let handle_open_change = move |is_open| help_open.set(is_open);
    let handle_close = move |_| help_open.set(false);
    let dismiss_for_good = move |_| {
        OnboardingPersistence::mark_seen();
        help_open.set(false);
    };
    rsx! {
        document::Stylesheet { href: HELP_DIALOG_STYLES }
        DialogRoot {
            class: "dialog-overlay",
            open: help_open(),
            on_open_change: handle_open_change,
            DialogContent { class: "dialog-shell wc3-dialog help-dialog".to_string(),
                DialogHeader {
                    title: "How to use this editor".to_string(),
                    on_close: handle_close,
                }
                div { class: "wc3-dialog-body help-dialog-body flex flex-col gap-[2.6rem] \
                        max-[1099px]:[flex:1_1_0] max-[1099px]:min-h-0 max-[1099px]:overflow-y-auto \
                        max-[1099px]:[-webkit-overflow-scrolling:touch] max-[1099px]:[overscroll-behavior:contain]",
                    section { class: "flex flex-col gap-[1.2rem]",
                        h3 { class: "help-section-title", "The workflow" }
                        ol { class: "help-workflow flex flex-col gap-[0.9rem] m-0 p-0",
                            for step in WORKFLOW_STEPS.iter() {
                                li { class: "help-workflow-step", "{step}" }
                            }
                        }
                    }
                    section { class: "flex flex-col gap-[1.4rem]",
                        h3 { class: "help-section-title", "Key tasks, explained" }
                        for block in TASK_BLOCKS.iter() {
                            div { class: "flex flex-col gap-[0.5rem]",
                                h4 { class: "help-task-heading", "{block.heading}" }
                                p { class: "help-task-body m-0", "{block.body}" }
                            }
                        }
                    }
                    section { class: "flex flex-col gap-[1.2rem]",
                        h3 { class: "help-section-title", "Button legend" }
                        ul { class: "help-legend flex flex-col gap-[0.9rem] m-0 p-0",
                            for entry in LEGEND_ENTRIES.iter() {
                                li { class: "help-legend-row flex items-center gap-[1.2rem] \
                                        max-[640px]:flex-col max-[640px]:items-start max-[640px]:gap-[0.4rem]",
                                    span {
                                        class: "help-legend-icon inline-flex items-center justify-center \
                                                shrink-0 w-[3rem] h-[3rem] [&_svg]:w-[2rem] [&_svg]:h-[2rem]",
                                        aria_hidden: "true",
                                        dangerous_inner_html: entry.icon,
                                    }
                                    span { class: "help-legend-text",
                                        span { class: "help-legend-label", "{entry.label}" }
                                        span { class: "help-legend-description", " {entry.description}" }
                                    }
                                }
                            }
                        }
                    }
                }
                footer { class: "flex items-center justify-end flex-none gap-4 pt-[1.4rem] px-[4.5rem] pb-[1.8rem] \
                        [border-top:1px_solid_rgba(255,206,99,0.4)] max-[480px]:px-6",
                    button {
                        class: "help-dismiss-button inline-flex items-center justify-center min-h-12 \
                                px-[1.8rem] py-[0.7rem] \
                                [background:linear-gradient(135deg,rgba(40,30,8,0.85)_0%,rgba(15,12,4,0.85)_100%)] \
                                border border-warcraft-gold rounded-[10px] text-warcraft-gold \
                                font-friz-quadrata text-[1.4rem] tracking-[0.08em] uppercase cursor-pointer \
                                [box-shadow:0_0_22px_rgba(255,206,99,0.22)] \
                                [transition:background_0.12s_ease,box-shadow_0.12s_ease] \
                                [@media(hover:hover)]:hover:[background:linear-gradient(135deg,rgba(255,206,99,0.22)_0%,rgba(60,45,14,0.95)_100%)] \
                                [@media(hover:hover)]:hover:[box-shadow:0_0_26px_rgba(255,206,99,0.55)] \
                                focus:outline-none \
                                focus-visible:border-white focus-visible:text-white \
                                focus-visible:[box-shadow:0_0_0_3px_#fff,0_0_18px_rgba(255,255,255,0.55)]",
                        r#type: "button",
                        onclick: dismiss_for_good,
                        "Got it, don't show this again"
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 4: Create the component stylesheet**

Create `crates/hotkey-editor/src/components/dialogs/help_dialog/help_dialog.css`:

```css
.help-dialog {
    max-width: min(960px, 94vw);
    width: 100%;
}
.help-dialog-body {
    padding: 2.6rem 4.5rem 2.2rem;
    text-align: left;
}
.help-section-title {
    margin: 0;
    font-family: "Friz Quadrata", serif;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    font-size: 2.2rem;
    line-height: 1.2;
    color: #ffce63;
    text-shadow: 1px 1px 0 #000;
}
.help-workflow {
    list-style: decimal;
    padding-left: 2.4rem;
}
.help-workflow-step {
    font-size: 1.7rem;
    line-height: 1.5;
    color: #e0d8c8;
}
.help-task-heading {
    margin: 0;
    font-family: "Friz Quadrata", serif;
    font-size: 1.8rem;
    line-height: 1.3;
    color: rgba(255, 206, 99, 0.9);
    text-shadow: 1px 1px 0 #000;
}
.help-task-body {
    font-size: 1.6rem;
    line-height: 1.55;
    color: #e0d8c8;
}
.help-legend-label {
    font-family: "Friz Quadrata", serif;
    color: #ffce63;
    text-shadow: 1px 1px 0 #000;
    font-size: 1.6rem;
}
.help-legend-description {
    font-size: 1.6rem;
    line-height: 1.5;
    color: #c0c8da;
}
.help-legend-icon {
    color: #ffce63;
    border: 1px solid rgba(255, 206, 99, 0.4);
    border-radius: 8px;
    background: linear-gradient(180deg, rgba(40, 30, 8, 0.55) 0%, rgba(15, 12, 4, 0.55) 100%);
}
@media (max-width: 480px) {
    .help-dialog-body {
        padding: 1.6rem 1.4rem 1.4rem;
    }
}
```

- [ ] **Step 5: Add the phone full screen sheet rule**

In `crates/hotkey-editor/src/components/dialogs/dialogs.css`, inside the existing `@media (max-width: 480px)` block, add `.dialog-shell.help-dialog,` to the list of selectors that the full screen sheet rule targets. Find the selector group that begins with `.dialog-shell,` and ends with `.dialog-shell.resolve-info-dialog {` and insert the new line so the group reads:

```css
    .dialog-shell,
    .dialog-shell.wc3-dialog,
    .dialog-shell.system-hotkeys-dialog,
    .dialog-shell.preview-dialog,
    .dialog-shell.help-dialog,
    .dialog-shell.key-picker-shell,
    .dialog-shell.sys-key-picker-shell,
    .dialog-shell.upload-info-dialog,
    .dialog-shell.download-info-dialog,
    .dialog-shell.resolve-info-dialog {
```

- [ ] **Step 6: Register help_dialog.css in the Tailwind build (content scan)**

`help_dialog.css` is loaded directly through `document::Stylesheet`, so it does not need a Tailwind import. No change to `tailwind.input.css` is required for the stylesheet itself, because the Tailwind `@source "./src/**/*.rs"` directive already scans the component's `.rs` file for the utility classes it uses. Confirm `tailwind.input.css` already contains the line `@source "./src/**/*.rs";` and make no edit if it does.

- [ ] **Step 7: Build the CSS and the wasm target**

Run: `nix develop -c moon run hotkey-editor:tailwind/build`
Expected: completes, writes `assets/tailwind.css`.

Run: `nix develop -c cargo build -p hotkey-editor --target wasm32-unknown-unknown`
Expected: builds with no errors.

- [ ] **Step 8: Lint**

Run: `nix develop -c cargo clippy -p hotkey-editor --all-targets --target wasm32-unknown-unknown -- -D warnings`
Expected: no warnings.

- [ ] **Step 9: Stage and report for signing**

```bash
git add crates/hotkey-editor/src/components/shared/icons/mod.rs \
        crates/hotkey-editor/src/components/dialogs/mod.rs \
        crates/hotkey-editor/src/components/dialogs/help_dialog/ \
        crates/hotkey-editor/src/components/dialogs/dialogs.css \
        crates/hotkey-editor/assets/tailwind.css
git status --short
```

---

### Task 3: Wire the signal, auto open, and Help buttons

Declares `help_open` in the app with the first visit initial value, renders the dialog, threads the signal to the header, and adds the Help button to both the desktop toolbar and the mobile burger menu.

**Files:**
- Modify: `crates/hotkey-editor/src/app.rs`
- Modify: `crates/hotkey-editor/src/components/shell/header/mod.rs`
- Modify: `crates/hotkey-editor/src/components/shell/header/toolbar/mod.rs`
- Modify: `crates/hotkey-editor/src/components/shell/header/burger/mod.rs`

**Interfaces:**
- Consumes from Task 1: `OnboardingPersistence::has_been_seen()`.
- Consumes from Task 2: `HelpDialog`, `HelpDialogProps { help_open }`, `ICON_HELP`.
- Produces: a `help_open: Signal<bool>` threaded App to Header to HeaderToolbar and BurgerMenu.

- [ ] **Step 1: Import the persistence helper and dialog in app.rs**

In `crates/hotkey-editor/src/app.rs`, add an import for `OnboardingPersistence` next to the existing `CustomKeysPersistence` import, and add an import for `HelpDialog` next to the existing `PreviewDialog` import. Locate the existing use lines for those two names and add the new names to the same paths. The persistence import path is `crate::services::customkeys::persistence`; the dialog import path is `crate::components::dialogs::help_dialog::HelpDialog`.

- [ ] **Step 2: Declare the help_open signal**

In `crates/hotkey-editor/src/app.rs`, immediately after the line:

```rust
    let mut system_hotkeys_open = use_signal::<bool>(|| false);
```

add:

```rust
    let help_open = use_signal::<bool>(|| !OnboardingPersistence::has_been_seen());
```

- [ ] **Step 3: Pass help_open to the Header**

In the `Header { ... }` invocation in `crates/hotkey-editor/src/app.rs`, add `help_open,` to the prop list, after `system_hotkeys_open,`. The block becomes:

```rust
            Header {
                loaded_keys,
                upload_status,
                preview_open,
                grid_layout,
                editing_layout_cell,
                dragging_layout_cell,
                system_hotkeys_open,
                help_open,
                current_view,
                active_race,
                unit_mode,
                selected_unit_id,
                search_query,
            }
```

- [ ] **Step 4: Render the HelpDialog**

In `crates/hotkey-editor/src/app.rs`, directly after the `system_hotkeys_open` dialog block:

```rust
                if *system_hotkeys_open.read() {
                    SystemHotkeysDialog { loaded_keys, system_hotkeys_open }
                }
```

add:

```rust
                if *help_open.read() {
                    HelpDialog { help_open }
                }
```

- [ ] **Step 5: Build to verify the app wiring compiles**

Run: `nix develop -c cargo build -p hotkey-editor --target wasm32-unknown-unknown`
Expected: FAIL, because `HeaderProps` does not yet have a `help_open` field. This confirms the prop is required next.

- [ ] **Step 6: Add help_open to HeaderProps and forward it**

In `crates/hotkey-editor/src/components/shell/header/mod.rs`, add the prop to `HeaderProps` after `system_hotkeys_open`:

```rust
    pub(crate) system_hotkeys_open: Signal<bool>,
    pub(crate) help_open: Signal<bool>,
```

In the `Header` function body, add a binding next to the existing `let system_hotkeys_open = props.system_hotkeys_open;`:

```rust
    let help_open = props.help_open;
```

Then forward `help_open` into both `HeaderToolbar` and `BurgerMenu`. The `HeaderToolbar` invocation becomes:

```rust
                HeaderToolbar {
                    loaded_keys,
                    upload_status,
                    preview_open,
                    templates_dialog_open,
                    system_hotkeys_open,
                    help_open,
                    navigation,
                }
```

The `BurgerMenu` invocation becomes:

```rust
                BurgerMenu {
                    loaded_keys,
                    preview_open,
                    layout_dialog_open,
                    templates_dialog_open,
                    system_hotkeys_open,
                    help_open,
                    navigation,
                }
```

- [ ] **Step 7: Add the Help button to the desktop toolbar**

In `crates/hotkey-editor/src/components/shell/header/toolbar/mod.rs`:

Add `ICON_HELP` to the icons import so the line reads:

```rust
use crate::components::shared::icons::{ICON_COG, ICON_HELP, ICON_TEMPLATES};
```

Add the prop to `HeaderToolbarProps` after `system_hotkeys_open`:

```rust
    pub(crate) system_hotkeys_open: Signal<bool>,
    pub(crate) help_open: Signal<bool>,
```

In the function body, after `let mut system_hotkeys_open = props.system_hotkeys_open;`, add the handler:

```rust
    let mut help_open = props.help_open;
    let open_help = move |_| help_open.set(true);
```

In the `rsx!`, add the Help button as the last child of the toolbar `div`, after `ExportButtons { loaded_keys, preview_open }`:

```rust
            button {
                class: super::TOOLBAR_BTN_CLASS,
                r#type: "button",
                aria_label: "How to use this editor",
                aria_haspopup: "dialog",
                aria_expanded: "{help_open()}",
                onclick: open_help,
                span {
                    class: super::TOOLBAR_ICON_CLASS,
                    aria_hidden: "true",
                    dangerous_inner_html: ICON_HELP,
                }
            }
```

- [ ] **Step 8: Add the Help item to the burger menu**

In `crates/hotkey-editor/src/components/shell/header/burger/mod.rs`:

Add `ICON_HELP` to the icons import. The import block becomes:

```rust
use crate::components::shared::icons::{
    ICON_BURGER, ICON_COG, ICON_DOWNLOAD, ICON_GRID, ICON_HELP, ICON_PREVIEW, ICON_REDO,
    ICON_RESOLVE, ICON_TEMPLATES, ICON_UNDO, ICON_UPLOAD,
};
```

Add the prop to `BurgerMenuProps` after `system_hotkeys_open`:

```rust
    pub(crate) system_hotkeys_open: Signal<bool>,
    pub(crate) help_open: Signal<bool>,
```

In the function body, after `let mut system_hotkeys_open = props.system_hotkeys_open;`, add:

```rust
    let mut help_open = props.help_open;
```

After the existing `toggle_system_hotkeys` closure, add an open handler that also closes the drawer:

```rust
    let open_help = move |_| {
        help_open.set(true);
        burger_open.set(false);
    };
```

In the `rsx!`, add a Help menu item as the last `button` inside the `div` with `role: "menu"`, after the `if has_loaded_file { ... }` block closes:

```rust
                        button {
                            class: BURGER_MENU_ITEM_CLASS,
                            r#type: "button",
                            role: "menuitem",
                            aria_haspopup: "dialog",
                            aria_expanded: "{help_open()}",
                            onclick: open_help,
                            span {
                                class: BURGER_MENU_ITEM_ICON_CLASS,
                                aria_hidden: "true",
                                dangerous_inner_html: ICON_HELP,
                            }
                            span { class: BURGER_MENU_ITEM_LABEL_CLASS, "Help" }
                        }
```

- [ ] **Step 9: Build and lint**

Run: `nix develop -c cargo build -p hotkey-editor --target wasm32-unknown-unknown`
Expected: builds with no errors.

Run: `nix develop -c cargo clippy -p hotkey-editor --all-targets --target wasm32-unknown-unknown -- -D warnings`
Expected: no warnings.

- [ ] **Step 10: Verify in the browser**

Start the dev server if it is not already running: `nix develop -c moon run hotkey-editor:dx/serve`. Open `http://localhost:8123/warcraft-hotkey-editor/`.

Confirm each of these by hand:
1. Clear the `warcraft-hotkey-editor.onboarding-seen` key in devtools Application storage, reload. The Help dialog opens on its own.
2. Close it with the X, reload. It opens again, because the flag was not written.
3. Reopen it, click "Got it, don't show this again", reload. It does not open. The localStorage key now holds `true`.
4. The header `?` button opens the dialog at a desktop width above 1500px.
5. Shrink the window below 1500px. The toolbar `?` is gone, the burger menu has a Help item that opens the dialog and closes the drawer.
6. At phone width the dialog is a full screen sheet and the body scrolls through all three sections. The legend rows stack icon above text.
7. Read the copy in the dialog. No hyphens, no semicolons, no em dashes anywhere.

- [ ] **Step 11: Run the full CI gate**

Run: `nix develop -c moon run :ci`
Expected: green, including the Playwright smoke tests.

- [ ] **Step 12: Stage and report for signing**

```bash
git add crates/hotkey-editor/src/app.rs \
        crates/hotkey-editor/src/components/shell/header/mod.rs \
        crates/hotkey-editor/src/components/shell/header/toolbar/mod.rs \
        crates/hotkey-editor/src/components/shell/header/burger/mod.rs
git status --short
```

Tell the user the feature is complete and every changed file is staged and ready to sign.

---

## Notes for the implementer

- The desktop toolbar (`HeaderToolbar`) is hidden below 1500px and the burger menu is hidden at and above 1500px. The Help control must exist in both so it is reachable at every width. This is why Task 3 touches both files.
- `OnboardingPersistence::seen_from_stored` is the only unit tested piece. The storage read and write are thin wrappers over `LocalStorage`, which is a no-op on the native test target, so they are exercised only in the browser verification step.
- Do not commit. Stage changes and let the user sign with their YubiKey.
