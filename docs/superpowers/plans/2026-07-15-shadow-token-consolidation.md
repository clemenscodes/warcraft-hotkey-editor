# Shadow / Glow Token Consolidation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development (recommended) or executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Promote every complex inline shadow / glow / drop-shadow / scrim Tailwind
utility in the `hotkey-editor` `style/mod.rs` files into named design-system tokens in
`crates/hotkey-editor/tailwind.css`, so no component style file carries a giant
`[text-shadow:…]` / `[box-shadow:…]` / `filter-[drop-shadow(…)]` / `after:bg-[linear-gradient(…)]`
literal any more.

**Architecture:** This is a **pure, visual-neutral conversion**. Each promoted token holds
the **byte-exact** value that is currently inline; the component then wears the generated
utility class instead of the literal. No value is "snapped" to a near-match, no opacity,
blur, offset, or color is changed. Tokens live in the shared `@theme` block (or an
`@utility` block, mirroring the existing `shadow-glow` / `drop-glow` precedent) — this is
sharing a *value*, which `docs/COMPONENTS.md` explicitly permits, never sharing a composite
*look*.

**Tech Stack:** Tailwind CSS v4.3 (`@theme` token namespaces `--text-shadow-*`,
`--shadow-*`, `--drop-shadow-*`; `@utility` blocks), the `tw-macro` `classes!`/`tw!`
engine, Dioxus. Verification via `moon run :ci` and Playwright screenshots.

## Global Constraints

- **Only three commands exist:** `moon run :ci` (the one verification gate),
  `moon run :check` (fast compile-only, NOT the gate), `moon run :dev` (the one dev
  server). Never `cargo`, `dx`, `playwright`, or a narrower `moon` target. `cargo fmt`
  directly is allowed for formatting only.
- **Dev URL is `http://localhost:8123/warcraft-hotkey-editor/`** (trailing slash) — bare
  `/` returns 404. Refresh the page rather than waiting on the "being rebuilt" overlay.
- **NO design changes.** Every token holds the current value verbatim (spaces where the
  Rust literal has `_`). The only permitted non-byte-identical change is `transition-property`
  *ordering* in Task 6, which has zero rendering effect (documented there).
- **Never run a second `moon run :ci` while one is running** — check `pgrep -af "moon run|playwright|dx"` and port 8123 first.
- **`--tw-*` composition note:** Tailwind's `shadow-*` / `drop-shadow-*` utilities apply the
  value through the `--tw-shadow` / `--tw-drop-shadow` chain (`box-shadow: …, var(--tw-shadow)`
  / `filter: … var(--tw-drop-shadow,)`). For every element converted here the other slots in
  that chain are empty, so the rendered result is identical to the wholesale inline
  `[box-shadow:…]` / `filter-[…]`. This is the same mechanism the existing `shadow-ring`,
  `shadow-bevel`, and `drop-glow` tokens already rely on. Playwright verification confirms it.
- **`tw!` scanner rule:** every class must stay a literal string in a `tw![…]` list. The new
  class names (`text-shadow-title`, `shadow-tile-highlight`, `drop-shadow-edge`, …) are
  plain literals — no `format!`, no interpolation.
- **Commit identity:** commit as `Clemens <clemenscodes@gmail.com>`.

## Out of scope — do NOT touch

These long tokens surfaced in the same scan but are **not** shadows/glows and are legitimate
bespoke geometry or variable wiring per `docs/COMPONENTS.md`; leave them exactly as they are:

- `kb-focus:[--focus-color:var(--color-warcraft-highlight)]` and every other
  `[--custom-property:…]` setter (long only because token names are long).
- The `hotkey_alt_position_picker_grid_anchor` `[&_.grid-editor-tile:not(:has(…))…]`
  descendant-selector classes (selector complexity, not a shared value — a separate
  structural concern).
- `grid-cols-[repeat(auto-fill,minmax(…))]`, `[grid-template-areas:…]`,
  `[--banner-image:url(…)]`, `[border-image-*]`, `supports-[anchor-name:…]` tooltip
  positioning, `[-webkit-tap-highlight-color:transparent]`.
- `transition-[border-color,box-shadow,transform]` (race_tab, line 20) — a different
  3-property list; only the exact 4-property list in Task 6 is consolidated.
- Any class already wearing a token (`shadow-ring`, `shadow-bevel`, `shadow-glow-soft`,
  `shadow-focus`, `shadow-glow`, `shadow-none`, `text-shadow-none`) — already correct.

## File map

- **Modify (tokens):** `crates/hotkey-editor/tailwind.css` — every task adds its tokens
  to the `@theme` block or as `@utility` blocks.
- **Modify (consumers):** one or two `style/mod.rs` files per task (exact paths in each task).
- No new files. No Rust logic changes. No `.rs` files outside `style/mod.rs` are touched.

Common path prefix used below (abbreviated as `…SHELL/`):
`crates/hotkey-editor/src/components/app/components/shell/`

---

### Task 1: Text-shadow tokens (title glow, badge outline, tab outline)

**Files:**
- Modify: `crates/hotkey-editor/tailwind.css` (`@theme` block, after line 92 `--text-shadow-outline`)
- Modify: `…SHELL/components/header/components/brand_host/components/brand/components/brand_title/style/mod.rs`
- Modify: `…SHELL/components/header/components/toolbar/components/collisions_button_host/components/collisions_button/components/collisions_button_badge/style/mod.rs`
- Modify: `…SHELL/components/editor_page/components/editor_tabs_bar/components/race_tabs_host/components/race_tabs/components/race_tab_banner/components/shared/race_tab_state/components/shared/race_tab/style/mod.rs`

**Interfaces:**
- Produces utility classes: `text-shadow-title`, `text-shadow-badge`, `text-shadow-tab`
  (Tailwind v4 generates `text-shadow-{name}` from each `--text-shadow-{name}` theme var).

- [ ] **Step 1: Capture baseline screenshots** (dev server must be running: `moon run :dev`)

Navigate to `http://localhost:8123/warcraft-hotkey-editor/` and screenshot: the header
brand title ("Warcraft III Hotkey Editor"), the collisions button badge (its number), and
a race tab label. Save as `text-shadow-baseline-*.png` under the scratchpad. These are the
"no design change" reference.

- [ ] **Step 2: Add the three text-shadow tokens to `@theme`**

In `crates/hotkey-editor/tailwind.css`, immediately after the `--text-shadow-outline:` line
(line 92), add:

```css
  --text-shadow-title: 1px 1px 0 color-mix(in oklab, var(--color-warcraft-shadow) 92%, transparent), 0 0 14px color-mix(in oklab, var(--color-warcraft-gold) 18%, transparent);
  --text-shadow-badge: 1.25cqi 1.25cqi 0 color-mix(in oklab, var(--color-warcraft-shadow) 95%, transparent), -1.25cqi 1.25cqi 0 color-mix(in oklab, var(--color-warcraft-shadow) 95%, transparent), 1.25cqi -1.25cqi 0 color-mix(in oklab, var(--color-warcraft-shadow) 95%, transparent), -1.25cqi -1.25cqi 0 color-mix(in oklab, var(--color-warcraft-shadow) 95%, transparent), 0 0 3.75cqi color-mix(in oklab, var(--color-warcraft-shadow) 95%, transparent);
  --text-shadow-tab: 0.04em 0.04em 0 var(--color-warcraft-shadow), -0.04em 0.04em 0 var(--color-warcraft-shadow), 0.04em -0.04em 0 var(--color-warcraft-shadow), -0.04em -0.04em 0 var(--color-warcraft-shadow), 0 0 0.31em color-mix(in oklab, var(--color-warcraft-shadow) 85%, transparent);
```

- [ ] **Step 3: Replace the inline literal in `brand_title/style/mod.rs`**

Replace the line:

```rust
        "[text-shadow:1px_1px_0_color-mix(in_oklab,var(--color-warcraft-shadow)_92%,transparent),0_0_14px_color-mix(in_oklab,var(--color-warcraft-gold)_18%,transparent)]",
```

with:

```rust
        "text-shadow-title",
```

- [ ] **Step 4: Replace the inline literal in `collisions_button_badge/style/mod.rs`**

Replace the line:

```rust
        "[text-shadow:1.25cqi_1.25cqi_0_color-mix(in_oklab,var(--color-warcraft-shadow)_95%,transparent),-1.25cqi_1.25cqi_0_color-mix(in_oklab,var(--color-warcraft-shadow)_95%,transparent),1.25cqi_-1.25cqi_0_color-mix(in_oklab,var(--color-warcraft-shadow)_95%,transparent),-1.25cqi_-1.25cqi_0_color-mix(in_oklab,var(--color-warcraft-shadow)_95%,transparent),0_0_3.75cqi_color-mix(in_oklab,var(--color-warcraft-shadow)_95%,transparent)]",
```

with:

```rust
        "text-shadow-badge",
```

- [ ] **Step 5: Replace the inline literal in `race_tab/style/mod.rs`**

Replace the line (in the `base` list):

```rust
        "[text-shadow:0.04em_0.04em_0_var(--color-warcraft-shadow),-0.04em_0.04em_0_var(--color-warcraft-shadow),0.04em_-0.04em_0_var(--color-warcraft-shadow),-0.04em_-0.04em_0_var(--color-warcraft-shadow),0_0_0.31em_color-mix(in_oklab,var(--color-warcraft-shadow)_85%,transparent)]",
```

with:

```rust
        "text-shadow-tab",
```

- [ ] **Step 6: Compile-check**

Run: `moon run :check`
Expected: PASS (compiles clean).

- [ ] **Step 7: Visual verify (no design change)**

Refresh `http://localhost:8123/warcraft-hotkey-editor/`. Re-screenshot the brand title,
the collisions badge number, and a race tab label. Compare against the Step-1 baselines —
they must be pixel-identical. If any differs, the token value has a typo; fix and re-verify.

- [ ] **Step 8: Commit**

```bash
git add crates/hotkey-editor/tailwind.css \
  crates/hotkey-editor/src/components/app/components/shell/components/header/components/brand_host/components/brand/components/brand_title/style/mod.rs \
  crates/hotkey-editor/src/components/app/components/shell/components/header/components/toolbar/components/collisions_button_host/components/collisions_button/components/collisions_button_badge/style/mod.rs \
  crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/editor_tabs_bar/components/race_tabs_host/components/race_tabs/components/race_tab_banner/components/shared/race_tab_state/components/shared/race_tab/style/mod.rs
git commit -m "refactor(style): promote inline text-shadows to text-shadow-{title,badge,tab} tokens"
```

---

### Task 2: Box-shadow (cqi) tokens — tile focus ring & tile highlight glow

**Files:**
- Modify: `crates/hotkey-editor/tailwind.css` (`@theme` block, after the `--shadow-drawer` line 88)
- Modify: `…SHELL/components/editor_page/components/editor_workspace/components/race_theme/components/shared/unit_detail/components/unit_detail_body/components/unit_detail_row/components/shared/grid_editors/shared/grid_editor/components/captioned_editor_grid/components/editor_grid/components/grid_editor_tile/style/mod.rs`
- Modify: `…SHELL/components/shared/grid_tile/components/empty_tile/style/mod.rs`

**Interfaces:**
- Produces utility classes: `shadow-tile-focus`, `shadow-tile-highlight`
  (Tailwind generates `shadow-{name}` from each `--shadow-{name}` theme var).

- [ ] **Step 1: Capture baseline screenshots**

With `moon run :dev` running, open a unit's command-grid editor. Screenshot (a) a
keyboard-focused grid editor tile (Tab to it — the gold focus ring), and (b) an empty tile
in its highlighted state (`.highlight-overlay`, e.g. during the alt-position picker flow).
Save as `box-shadow-baseline-*.png`.

- [ ] **Step 2: Add the two box-shadow tokens to `@theme`**

In `crates/hotkey-editor/tailwind.css`, immediately after the `--shadow-drawer:` line
(line 88), add:

```css
  --shadow-tile-focus: 0 0 0 0.52cqi var(--color-warcraft-gold), 0 0 3.1cqi color-mix(in oklab, var(--color-warcraft-gold) 55%, transparent);
  --shadow-tile-highlight: 0 0 7cqi color-mix(in oklab, var(--color-warcraft-gold) 50%, transparent);
```

- [ ] **Step 3: Replace the inline literal in `grid_editor_tile/style/mod.rs`**

Replace the line:

```rust
        "kb-focus:[box-shadow:0_0_0_0.52cqi_var(--color-warcraft-gold),0_0_3.1cqi_color-mix(in_oklab,var(--color-warcraft-gold)_55%,transparent)]",
```

with:

```rust
        "kb-focus:shadow-tile-focus",
```

- [ ] **Step 4: Replace the inline literal in `empty_tile/style/mod.rs`**

Replace the line:

```rust
        "has-[.highlight-overlay]:[box-shadow:0_0_7cqi_color-mix(in_oklab,var(--color-warcraft-gold)_50%,transparent)]",
```

with:

```rust
        "has-[.highlight-overlay]:shadow-tile-highlight",
```

- [ ] **Step 5: Compile-check**

Run: `moon run :check`
Expected: PASS.

- [ ] **Step 6: Visual verify**

Refresh, re-screenshot the focused tile and the highlighted empty tile, compare against the
Step-1 baselines — pixel-identical. Confirm the `cqi` scaling still tracks tile size (resize
the window; the ring/glow scale with the tile exactly as before).

- [ ] **Step 7: Commit**

```bash
git add crates/hotkey-editor/tailwind.css \
  crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/editor_workspace/components/race_theme/components/shared/unit_detail/components/unit_detail_body/components/unit_detail_row/components/shared/grid_editors/shared/grid_editor/components/captioned_editor_grid/components/editor_grid/components/grid_editor_tile/style/mod.rs \
  crates/hotkey-editor/src/components/app/components/shell/components/shared/grid_tile/components/empty_tile/style/mod.rs
git commit -m "refactor(style): promote cqi tile box-shadows to shadow-tile-{focus,highlight} tokens"
```

---

### Task 3: Drop-shadow tokens — edge (shared 2×), bevel, checkbox, heart, raised-glow

**Files:**
- Modify: `crates/hotkey-editor/tailwind.css` (`@theme` block, after the `--shadow-tile-highlight` line added in Task 2)
- Modify: `…SHELL/components/header/components/brand_host/components/brand/components/shared/brand_decoration_host/components/brand_decoration/style/mod.rs`
- Modify: `…SHELL/components/shared/warcraft_dialog/components/warcraft_dialog_header/components/dialog_header/components/shared/dialog_header_decoration/style/mod.rs`
- Modify: `…SHELL/components/editor_page/components/editor_workspace/components/race_theme/components/shared/unit_detail/components/unit_stats_panel/components/shared/stat_icon_frame_host/components/stat_icon_frame/style/mod.rs`
- Modify: `…SHELL/components/header/components/toolbar/components/toolbar_actions/components/shared/dialogs/grid_layout_editor_dialog/components/grid_layout_editor_dialog_body/components/grid_layout_editor_dialog_content/components/move_hotkey_toggle/components/move_hotkey_checkbox/style/mod.rs`
- Modify: `…SHELL/components/footer/components/footer_credit/components/footer_heart/style/mod.rs`
- Modify: `…SHELL/components/header/components/toolbar/components/toolbar_actions/components/shared/dialogs/system_hotkeys_dialog/components/system_hotkeys_dialog_body/components/inventory_drag_overlay/style/mod.rs`

**Interfaces:**
- Produces utility classes: `drop-shadow-edge`, `drop-shadow-bevel`, `drop-shadow-drop`,
  `drop-shadow-heart`, `drop-shadow-raised-glow`. Tailwind v4 generates `drop-shadow-{name}`
  from each `--drop-shadow-{name}` theme var and applies it through the composing
  `--tw-drop-shadow` filter chain (identical output where no other filter is present — see
  Global Constraints).
- **`edge` is the one true merge:** `brand_decoration` and `dialog_header_decoration` carry
  the byte-identical `filter-[drop-shadow(0_1px_0_shadow70%)]`; both become `drop-shadow-edge`.

- [ ] **Step 1: Capture baseline screenshots**

With `moon run :dev` running, screenshot: the header brand decoration flourish, a dialog
header decoration (open any dialog, e.g. Help), a unit stats icon frame, the grid-layout
editor dialog's "move hotkey" checkbox in its checked state, the footer heart, and — if
reachable — the system-hotkeys inventory drag overlay (drag an inventory slot). Save as
`drop-shadow-baseline-*.png`. (If the drag overlay is hard to trigger interactively, verify
it in Step 7 by asserting the rendered `filter` computed style instead.)

- [ ] **Step 2: Add the five drop-shadow tokens to `@theme`**

In `crates/hotkey-editor/tailwind.css`, immediately after the `--shadow-tile-highlight:`
line added in Task 2, add:

```css
  --drop-shadow-edge: 0 1px 0 color-mix(in oklab, var(--color-warcraft-shadow) 70%, transparent);
  --drop-shadow-bevel: 0 1px 2px color-mix(in oklab, var(--color-warcraft-shadow) 60%, transparent);
  --drop-shadow-drop: 1px 1px 0 var(--color-warcraft-shadow);
  --drop-shadow-heart: 0 0 0.3em color-mix(in oklab, var(--color-race-orc) 35%, transparent);
  --drop-shadow-raised-glow: 0 8px 24px color-mix(in oklab, var(--color-warcraft-shadow) 60%, transparent), 0 0 16px color-mix(in oklab, var(--color-warcraft-gold) 60%, transparent);
```

- [ ] **Step 3: Replace in `brand_decoration/style/mod.rs`**

Replace:

```rust
        "filter-[drop-shadow(0_1px_0_color-mix(in_oklab,var(--color-warcraft-shadow)_70%,transparent))]",
```

with:

```rust
        "drop-shadow-edge",
```

- [ ] **Step 4: Replace in `dialog_header_decoration/style/mod.rs`**

Replace the same identical line:

```rust
        "filter-[drop-shadow(0_1px_0_color-mix(in_oklab,var(--color-warcraft-shadow)_70%,transparent))]",
```

with:

```rust
        "drop-shadow-edge",
```

- [ ] **Step 5: Replace in `stat_icon_frame/style/mod.rs`**

Replace:

```rust
        "filter-[drop-shadow(0_1px_2px_color-mix(in_oklab,var(--color-warcraft-shadow)_60%,transparent))]",
```

with:

```rust
        "drop-shadow-bevel",
```

- [ ] **Step 6: Replace in `move_hotkey_checkbox/style/mod.rs`**

Replace:

```rust
        "checked:after:filter-[drop-shadow(1px_1px_0_var(--color-warcraft-shadow))]",
```

with:

```rust
        "checked:after:drop-shadow-drop",
```

- [ ] **Step 7: Replace in `footer_heart/style/mod.rs`**

Replace:

```rust
        "drop-shadow-[0_0_0.3em_color-mix(in_oklab,var(--color-race-orc)_35%,transparent)]",
```

with:

```rust
        "drop-shadow-heart",
```

- [ ] **Step 8: Replace in `inventory_drag_overlay/style/mod.rs`**

Replace:

```rust
        "filter-[drop-shadow(0_8px_24px_color-mix(in_oklab,var(--color-warcraft-shadow)_60%,transparent))_drop-shadow(0_0_16px_color-mix(in_oklab,var(--color-warcraft-gold)_60%,transparent))]",
```

with:

```rust
        "drop-shadow-raised-glow",
```

- [ ] **Step 9: Compile-check**

Run: `moon run :check`
Expected: PASS.

- [ ] **Step 10: Visual verify (including the multi-layer case)**

Refresh and compare each component against its Step-1 baseline. **Pay special attention to
`inventory_drag_overlay`** (the two-layer raised+glow): confirm both the drop shadow and the
gold glow render. If the two-layer `--drop-shadow-raised-glow` renders as a single or wrong
shadow (a Tailwind multi-value edge case), fall back to an explicit `@utility` instead of the
theme var:

```css
@utility drop-shadow-raised-glow {
  filter: drop-shadow(0 8px 24px color-mix(in oklab, var(--color-warcraft-shadow) 60%, transparent)) drop-shadow(0 0 16px color-mix(in oklab, var(--color-warcraft-gold) 60%, transparent));
}
```

(Remove the `--drop-shadow-raised-glow` theme line if you take the fallback; the class name
consumed by the component stays `drop-shadow-raised-glow` either way.) Re-verify.

- [ ] **Step 11: Commit**

```bash
git add crates/hotkey-editor/tailwind.css \
  crates/hotkey-editor/src/components/app/components/shell/components/header/components/brand_host/components/brand/components/shared/brand_decoration_host/components/brand_decoration/style/mod.rs \
  crates/hotkey-editor/src/components/app/components/shell/components/shared/warcraft_dialog/components/warcraft_dialog_header/components/dialog_header/components/shared/dialog_header_decoration/style/mod.rs \
  crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/editor_workspace/components/race_theme/components/shared/unit_detail/components/unit_stats_panel/components/shared/stat_icon_frame_host/components/stat_icon_frame/style/mod.rs \
  crates/hotkey-editor/src/components/app/components/shell/components/header/components/toolbar/components/toolbar_actions/components/shared/dialogs/grid_layout_editor_dialog/components/grid_layout_editor_dialog_body/components/grid_layout_editor_dialog_content/components/move_hotkey_toggle/components/move_hotkey_checkbox/style/mod.rs \
  crates/hotkey-editor/src/components/app/components/shell/components/footer/components/footer_credit/components/footer_heart/style/mod.rs \
  crates/hotkey-editor/src/components/app/components/shell/components/header/components/toolbar/components/toolbar_actions/components/shared/dialogs/system_hotkeys_dialog/components/system_hotkeys_dialog_body/components/inventory_drag_overlay/style/mod.rs
git commit -m "refactor(style): promote inline drop-shadows to drop-shadow-{edge,bevel,drop,heart,raised-glow} tokens"
```

---

### Task 4: Bottom-fade scrim gradient utility

**Files:**
- Modify: `crates/hotkey-editor/tailwind.css` (add an `@utility` block near the other `bg-panel-*` utilities, after line 102)
- Modify: `…SHELL/components/editor_page/components/editor_tabs_bar/components/race_tabs_host/components/race_tabs/components/race_tab_banner/components/shared/race_tab_state/components/shared/race_tab/style/mod.rs`

**Interfaces:**
- Produces utility class: `bg-scrim-bottom` (a `background-image` linear-gradient). It is
  consumed under the `after:` variant as `after:bg-scrim-bottom`, exactly like the existing
  `before:[background-image:var(--banner-image)]` pattern in the same file.

- [ ] **Step 1: Baseline screenshot**

With `moon run :dev` running, screenshot a race tab (the bottom of the banner where the
label sits over the dark scrim). Save as `scrim-baseline.png`.

- [ ] **Step 2: Add the `@utility` block**

In `crates/hotkey-editor/tailwind.css`, after the `@utility bg-race-banner …` line
(line 102), add:

```css
@utility bg-scrim-bottom { background-image: linear-gradient(180deg, color-mix(in oklab, var(--color-warcraft-shadow) 0%, transparent) 0%, color-mix(in oklab, var(--color-warcraft-shadow) 0%, transparent) 45%, color-mix(in oklab, var(--color-warcraft-shadow) 55%, transparent) 75%, color-mix(in oklab, var(--color-warcraft-shadow) 85%, transparent) 100%); }
```

- [ ] **Step 3: Replace the inline literal in `race_tab/style/mod.rs`**

Replace the line:

```rust
        "after:bg-[linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-shadow)_0%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-shadow)_0%,transparent)_45%,color-mix(in_oklab,var(--color-warcraft-shadow)_55%,transparent)_75%,color-mix(in_oklab,var(--color-warcraft-shadow)_85%,transparent)_100%)]",
```

with:

```rust
        "after:bg-scrim-bottom",
```

- [ ] **Step 4: Compile-check**

Run: `moon run :check`
Expected: PASS.

- [ ] **Step 5: Visual verify**

Refresh, re-screenshot the race tab scrim, compare against `scrim-baseline.png` — pixel-identical.

- [ ] **Step 6: Commit**

```bash
git add crates/hotkey-editor/tailwind.css \
  crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/editor_tabs_bar/components/race_tabs_host/components/race_tabs/components/race_tab_banner/components/shared/race_tab_state/components/shared/race_tab/style/mod.rs
git commit -m "refactor(style): promote race-tab bottom scrim to bg-scrim-bottom utility"
```

---

### Task 5: `transition-interactive` utility (consolidate 10 identical property lists)

**Files:**
- Modify: `crates/hotkey-editor/tailwind.css` (add an `@utility` block after the `duration-slow` line 120)
- Modify (each carries the exact 4-property `transition-[…]` list):
  - `…SHELL/components/header/components/toolbar/components/shared/toolbar_button_surface/components/attention_surface/style/mod.rs`
  - `…SHELL/components/header/components/toolbar/components/shared/toolbar_button_surface/components/clear_surface/style/mod.rs`
  - `…SHELL/components/header/components/toolbar/components/shared/toolbar_button_surface/components/interactive_surface/style/mod.rs`
  - `…SHELL/components/header/components/toolbar/components/toolbar_actions/components/burger_menu/components/burger_drawer/components/burger_drawer_body/components/shared/burger_menu_item/components/active_menu_item/style/mod.rs`
  - `…SHELL/components/header/components/toolbar/components/toolbar_actions/components/burger_menu/components/burger_drawer/components/burger_drawer_body/components/shared/burger_menu_item/components/idle_menu_item/style/mod.rs`
  - `…SHELL/components/header/components/toolbar/components/toolbar_actions/components/burger_menu/components/burger_drawer/components/burger_drawer_body/components/shared/burger_menu_item/components/primary_menu_item/style/mod.rs`
  - `…SHELL/components/header/components/toolbar/components/toolbar_actions/components/burger_menu/style/mod.rs`
  - `…SHELL/components/header/components/toolbar/components/toolbar_actions/components/shared/dialogs/templates_dialog/components/templates_dialog_body/components/template_gallery/components/template_card/style/mod.rs`
  - `…SHELL/components/shared/editable_keycap/components/editing_keycap/style/mod.rs` (order variant)
  - `…SHELL/components/shared/editable_keycap/components/idle_keycap/style/mod.rs` (order variant)

**Interfaces:**
- Produces utility class: `transition-interactive` (sets `transition-property` + the theme
  default duration/timing). Consumes nothing from earlier tasks.
- **Documented, provably-neutral non-byte change:** the eight surface/menu/card files use
  `transition-[border-color,color,background,box-shadow]`; the two keycap files use the same
  four properties in a different order `transition-[box-shadow,border-color,background,color]`.
  `transition-property` is a set — order does not affect rendering — so all ten collapse to
  one `transition-interactive`. This is the only place in the plan output is not byte-identical;
  it is visually identical.

- [ ] **Step 1: Find and confirm all ten occurrences**

Run:

```bash
grep -rn 'transition-\[border-color,color,background,box-shadow\]\|transition-\[box-shadow,border-color,background,color\]' crates/hotkey-editor/src
```

Expected: exactly the ten files listed above (8 of the first form, 2 of the second).

- [ ] **Step 2: Add the `@utility` block**

In `crates/hotkey-editor/tailwind.css`, after the `@utility duration-slow …` line (line 120), add:

```css
@utility transition-interactive {
  transition-property: border-color, color, background, box-shadow;
  transition-timing-function: var(--default-transition-timing-function);
  transition-duration: var(--default-transition-duration);
}
```

- [ ] **Step 3: Replace in the eight `border-color,color,background,box-shadow` files**

In each of the eight non-keycap files, replace:

```rust
        "transition-[border-color,color,background,box-shadow]",
```

with:

```rust
        "transition-interactive",
```

- [ ] **Step 4: Replace in the two keycap files**

In `editing_keycap/style/mod.rs` and `idle_keycap/style/mod.rs`, replace:

```rust
        "transition-[box-shadow,border-color,background,color]",
```

with:

```rust
        "transition-interactive",
```

- [ ] **Step 5: Compile-check**

Run: `moon run :check`
Expected: PASS.

- [ ] **Step 6: Visual verify (interaction, not just static)**

With the dev server refreshed: hover a toolbar button, an open burger-menu item, a template
card, and an editable keycap. Each must still animate its border/color/background/box-shadow
over ~0.15s exactly as before (no snap, no missing transition).

- [ ] **Step 7: Commit**

```bash
git add crates/hotkey-editor/tailwind.css \
  crates/hotkey-editor/src/components/app/components/shell/components/header/components/toolbar/components/shared/toolbar_button_surface/components/attention_surface/style/mod.rs \
  crates/hotkey-editor/src/components/app/components/shell/components/header/components/toolbar/components/shared/toolbar_button_surface/components/clear_surface/style/mod.rs \
  crates/hotkey-editor/src/components/app/components/shell/components/header/components/toolbar/components/shared/toolbar_button_surface/components/interactive_surface/style/mod.rs \
  crates/hotkey-editor/src/components/app/components/shell/components/header/components/toolbar/components/toolbar_actions/components/burger_menu/components/burger_drawer/components/burger_drawer_body/components/shared/burger_menu_item/components/active_menu_item/style/mod.rs \
  crates/hotkey-editor/src/components/app/components/shell/components/header/components/toolbar/components/toolbar_actions/components/burger_menu/components/burger_drawer/components/burger_drawer_body/components/shared/burger_menu_item/components/idle_menu_item/style/mod.rs \
  crates/hotkey-editor/src/components/app/components/shell/components/header/components/toolbar/components/toolbar_actions/components/burger_menu/components/burger_drawer/components/burger_drawer_body/components/shared/burger_menu_item/components/primary_menu_item/style/mod.rs \
  crates/hotkey-editor/src/components/app/components/shell/components/header/components/toolbar/components/toolbar_actions/components/burger_menu/style/mod.rs \
  crates/hotkey-editor/src/components/app/components/shell/components/header/components/toolbar/components/toolbar_actions/components/shared/dialogs/templates_dialog/components/templates_dialog_body/components/template_gallery/components/template_card/style/mod.rs \
  crates/hotkey-editor/src/components/app/components/shell/components/shared/editable_keycap/components/editing_keycap/style/mod.rs \
  crates/hotkey-editor/src/components/app/components/shell/components/shared/editable_keycap/components/idle_keycap/style/mod.rs
git commit -m "refactor(style): consolidate 10 identical transition lists into transition-interactive utility"
```

---

### Task 6: Full gate + final sweep

**Files:** none modified (verification only), unless the sweep finds a straggler.

- [ ] **Step 1: Re-run the long-token scan to confirm the shadow/glow offenders are gone**

Run:

```bash
grep -rn '\[text-shadow:\|\[box-shadow:\|filter-\[drop-shadow\|drop-shadow-\[\|after:bg-\[linear-gradient' crates/hotkey-editor/src
```

Expected: **zero** matches. Any remaining hit is a straggler this plan missed — promote it
using the same pattern (byte-exact token/utility + class replacement + visual verify) before
proceeding.

- [ ] **Step 2: Confirm no second `moon run :ci` is already running**

Run: `pgrep -af "moon run|playwright|dx"` and check port 8123. Stop the `moon run :dev`
server from earlier tasks first (the e2e gate starts its own server).

- [ ] **Step 3: Run the one and only gate**

Run: `moon run :ci`
Expected: PASS — fmt, clippy (wasm), keybinds tests, wasm build, and all Playwright e2e
smoke tests green. Read the process's own exit summary, not a piped tail.

- [ ] **Step 4: Final holistic visual pass**

With a fresh `moon run :dev`, walk the app once (header, race tabs, a unit grid, system
hotkeys dialog, grid-layout dialog, templates dialog, footer) and confirm nothing looks
different from the pre-refactor baselines captured across Tasks 1–5.

- [ ] **Step 5: Commit (only if the gate produced formatting changes)**

```bash
git add -A
git commit -m "chore(style): moon run :ci green after shadow/glow token consolidation"
```

---

## Self-Review

**Spec coverage** — every genuine shadow/glow/effect offender from the inventory is mapped:
- Text-shadows (title, badge, tab) → Task 1 ✓
- cqi box-shadows (tile focus, tile highlight) → Task 2 ✓
- Drop-shadows (edge ×2 merged, bevel, checkbox drop, heart, raised-glow) → Task 3 ✓
- Scrim gradient → Task 4 ✓
- transition list ×10 → Task 5 ✓
- Full gate + straggler sweep → Task 6 ✓
- Genuinely-bespoke non-shadow long tokens (custom-property setters, complex selectors,
  banner urls, tooltip anchoring) → explicitly listed **Out of scope** ✓

**Placeholder scan** — every code step carries the verbatim before/after string; no "TBD",
no "handle edge cases", no "similar to Task N". The one uncertainty (multi-value drop-shadow)
has a concrete fallback in Task 3 Step 10. ✓

**Type/name consistency** — class names are stable across tasks: `text-shadow-title/badge/tab`,
`shadow-tile-focus/highlight`, `drop-shadow-edge/bevel/drop/heart/raised-glow`,
`bg-scrim-bottom`, `transition-interactive`. Each `--token` name matches the utility it
generates. No collisions with existing tokens (distinct `--text-shadow-*` / `--shadow-*` /
`--drop-shadow-*` namespaces). ✓
