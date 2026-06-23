# Onboarding and Help dialog

## Goal

First time visitors find the editor overwhelming. They have no way to learn
the intended flow, which is load defaults, detect collisions, resolve
conflicts, apply a global grid, then fine tune with drag and drop. This
feature adds a Help dialog that explains that flow and labels every header
control. It opens automatically on the first visit and stays reachable from a
Help button in the header.

## Scope

This is renderer only. It adds presentation and one small persistence helper.
It contains no domain logic. It does not call into `warcraft_keybinds`, it
does not touch `CustomKeysFile`, and it never calls `binding.set_*`. It honors
the wall described in `docs/ARCHITECTURE.md`.

## Behavior

### First visit detection

A new localStorage key tracks whether the user has dismissed the guide for
good. The key is `warcraft-hotkey-editor.onboarding-seen` and its value is the
string `true`. The key is read and written through the existing `LocalStorage`
wrapper in `services/storage/local_storage.rs`.

A new persistence helper sits next to `CustomKeysPersistence`. It exposes two
methods. One reports whether the guide has been marked as seen. One marks it as
seen. The helper holds a single `LocalStorage` constant for the new key.

### Auto open

The app declares a new signal, `help_open: Signal<bool>`. The signal
initializes to `true` only when the persistence helper reports that the guide
has not been seen. Otherwise it initializes to `false`. First time visitors
therefore get the dialog opened for them. Returning visitors who already opted
out do not.

### Dismissal

The dialog can close in two ways and they differ.

The primary action is a button labelled "Got it, don't show this again". It
writes the seen flag through the persistence helper and then closes the dialog.
This is the only way to stop the automatic open on future visits.

Closing through the X button, the Escape key, or a click on the overlay simply
closes the dialog. It does not write the flag. The dialog will open again on
the next visit. This matches the rule that the user decides whether to keep
seeing it.

### Help button

A new Help button appears in the header toolbar. Its icon is a question mark.
Clicking it sets `help_open` to `true` and opens the dialog at any time. The
button works regardless of the seen flag, so the guide is always available as a
reference.

## Components and files

All paths are under `crates/hotkey-editor/`.

### Icon

Add `ICON_HELP` to `src/components/shared/icons/mod.rs`. It is an inline SVG
question mark that follows the existing icon convention, which is a `&str`
constant rendered through `dangerous_inner_html`.

### Header button

Add a `HelpButton` to the header toolbar in `src/components/shell/header/`. It
mirrors the structure and styling of the existing toolbar buttons such as
`ResolveButton` and `UploadButton`, reusing `TOOLBAR_BTN_CLASS` and
`TOOLBAR_ICON_CLASS`. It receives the `help_open` signal and sets it to `true`
on click.

### Dialog

Add `src/components/dialogs/help_dialog/mod.rs` and a sibling
`help_dialog.css`. The component is built on `DialogRoot`, `DialogContent`,
and `DialogHeader`, the same primitives the `PreviewDialog` uses. It takes the
`help_open` signal. Its `on_open_change` and X handler close the dialog without
writing the flag. The primary button writes the flag through the persistence
helper and then closes.

### Persistence helper

Add the onboarding persistence helper next to `CustomKeysPersistence` in the
services layer. It owns the `warcraft-hotkey-editor.onboarding-seen` key.

### App wiring

Declare `help_open` in `src/app.rs` with the conditional initial value
described above. Render the `HelpDialog` near the other top level dialogs such
as `PreviewDialog`. Thread `help_open` to the `Header` as a prop, the same way
`preview_open` is threaded today.

## Dialog content

The dialog body is one scrollable column with three sections.

### Section one, the workflow

An ordered list that gives a quick overview.

1. The editor loads the Warcraft III default keybinds, or a template if you
   applied one.
2. It automatically finds every hotkey and position collision.
3. You open the Collisions and Resolve pages to settle each conflict.
4. You define a global grid layout and apply it to all units.
5. You search for a unit and drag abilities onto the grid to rebind them.
6. You export your CustomKeys.txt file.

### Section two, key tasks explained

Four short blocks, each with a heading and a plain paragraph.

**Applying a template.** A template is a complete keybind set that someone
prepared in advance. Applying one replaces all of your current keys with the
keys from the template. The editor then normalizes the result, which means it
resolves the cascades and settles the collisions on its own. Reach for a
template when you want a solid starting point instead of the Warcraft III
defaults.

**Resolving conflicts.** After you load the defaults or a template, some
commands will clash. Two abilities might ask for the same hotkey. Two units
might want the same grid cell. The editor finds all of these for you. The
Collisions page shows every clash so you can see what is wrong. The Resolve
page settles them in bulk by shifting and swapping bindings along the cascade
until nothing conflicts. Keep going until the editor reports a clean state.

**Applying the grid to all units.** You do not have to bind each unit by hand.
Instead you define one global grid layout, which decides which command lives in
which cell of the command card. You then apply that layout to every unit at
once. The editor rewrites the positions and hotkeys across all units, so the
whole game shares one consistent layout that is easy to learn.

**Fine tuning with drag and drop.** Use this for small adjustments after the
global grid is applied. Search for a unit, then drag an ability from one grid
cell to another. When you drop it, the editor rewrites that ability's hotkey to
match its new cell. This is how you handle the units that need a layout
slightly different from the global one.

### Section three, button legend

Each row pairs the real header icon, taken from `icons/mod.rs` so it can never
drift from the actual header, with one plain sentence. The rows cover Grid
Layout, Collisions, Templates, Upload, System Hotkeys, Resolve, Preview, Export
and download, Undo and Redo, and the Help button itself.

## Responsive layout

The dialog works on every viewport, which is mandatory for this project. The
body scrolls when it is taller than the viewport. On narrow screens the legend
rows stack so the icon sits above its text rather than beside it. This follows
the same responsive treatment the other `wc3-dialog` bodies already use.

## CSS build note

The new `help_dialog.css` will not be picked up by `dx serve` on its own. After
editing it, run the Tailwind build task for the hotkey editor, then reload the
browser. This is a known quirk of the dev server.

## Testing

The persistence helper gets a unit test that covers the round trip. When the
key is absent the helper reports not seen. After the mark method runs the
helper reports seen. The dialog itself is renderer UI and is verified by
opening the app in a browser and confirming the auto open on first visit, the
permanent opt out, and the Help button reopen. The Playwright smoke gate must
stay green.

## Out of scope

A guided coach mark tour that highlights live elements step by step is out of
scope. We chose the static guide first because it ports cleanly to mobile and
reuses the existing dialog pattern. A tour can be revisited later if the guide
proves insufficient. The seen flag is not tied to the app version, so the guide
does not reappear when features change.
