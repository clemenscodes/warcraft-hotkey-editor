# Tablet Layout Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development (recommended) or executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Render the mobile pager ("big mobile") on the `tablet` band (768 to 1279px) so tablets get a touch layout with each unit's grids side by side, instead of the cramped desktop layout they inherit today.

**Architecture:** Three small style/branch edits. (1) Widen the viewport hook that gates the pager from the `mobile` band to the touch bands (`< 1280px`) and flip the `editor_page` branch. (2) Make the pager's snap-scroll container band-agnostic so it works on tablet too. (3) Cap and centre each unit card on tablet so its `cqi`-driven interior stays ergonomic at landscape widths. Everything else — the race nav, `UnitCommandGrids` (already carries a `tablet:` two-column grid), the carousel dots (auto-hide off `mobile`), the hotkey override (auto full-width off `tablet:`), the three touch-drag grid editors — is reused unchanged.

**Tech Stack:** Rust, Dioxus (wasm), the `tw-macro` `classes!` styling engine, Tailwind v4 responsive bands. No domain crate changes.

## Global Constraints

- Responsive bands are **disjoint**, nothing cascades: `mobile` is `width < 768px`, `tablet` is `768px <= width < 1280px`, `laptop` is `1280px <= width < 1920px` (`crates/hotkey-editor/tailwind.css`). A `mobile:` class never applies at `>= 768px`.
- Styling is only the `classes!` macro. In a `classes!` block: `base` must carry **no** band-prefixed class; each non-`base` key (`mobile`/`tablet`/…) must carry **only** its own prefix. Violating either is a compile error.
- Prefer a real Tailwind token/utility over an arbitrary `[…]` value where one exists.
- **Verification is the browser.** This is a pure UI change; there is no unit test. Verify by driving the running dev server at `http://localhost:8123/warcraft-hotkey-editor/` (trailing slash) at the named widths. The dev server is the USER's `moon run :dev` — never start it.
- **Never run `moon run :check` / `:ci` / `tailwind/build` while the user's `moon run :dev` is running** — they contend for the cargo `target/` build lock and stall the dev server's incremental rebuild (looks like "stale builds"). Let the dev server rebuild on save and reload once. Run the final `moon run :ci` gate only when the dev server is stopped, or have the user run it.
- No new components; reuse existing ones.
- Commit identity: `Clemens <clemenscodes@gmail.com>`. Commit messages end with the trailer `Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>`. Commits are gpg-signed; the user may defer signing/committing until the work is approved, so a blocked commit is expected, not an error.

---

## File Structure

All paths under `crates/hotkey-editor/src/components/app/components/shell/components/editor_page/`.

- `viewport.rs` — the media-query hook that gates the pager. Widened to the touch bands and renamed.
- `mod.rs` — the mobile/desktop branch. Flipped to branch on the touch hook.
- `components/mobile_editor/style/mod.rs` — the pager's snap-scroll container. Made band-agnostic so it also governs tablet.
- `components/mobile_editor/components/pager_card_host/style/mod.rs` — the per-unit snap box and `cqi` reference container. Gains a tablet width cap.

No files are created. No presentation, model, view, or data files change.

---

## Task 1: Make the pager container band-agnostic

Do this **before** Task 2. Today the container's flex/scroll/snap classes live only on the `mobile:` band with a `hidden` base, because the pager only ever mounted on `mobile`. Moving them to `base` (plus a `tablet:` padding step) is a no-op while the branch is still `mobile`-only, and it makes the container correct for tablet before the branch starts mounting it there. Splitting it out first means Task 2 never produces a blank tablet screen.

**Files:**
- Modify: `editor_page/components/mobile_editor/style/mod.rs`

**Interfaces:**
- Consumes: nothing.
- Produces: `MobileEditor`'s root `<section>` renders as a vertical snap-scroll pager on **any** band it is mounted on (currently still only `mobile`).

- [ ] **Step 1: Read the current file to confirm the starting point**

Run: `cat crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/mobile_editor/style/mod.rs`
Expected: `base` is `tw!["hidden"]`; `mobile` is `tw!["mobile:flex", "mobile:flex-col", "mobile:flex-1", "mobile:min-h-0", "mobile:min-w-0", "mobile:overflow-y-auto", "mobile:overscroll-contain", "mobile:snap-y", "mobile:snap-mandatory", "mobile:px-4"]`.

- [ ] **Step 2: Rewrite the file to base classes plus a tablet padding step**

Replace the whole file with:

```rust
use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "flex-1",
        "min-h-0",
        "min-w-0",
        "overflow-y-auto",
        "overscroll-contain",
        "snap-y",
        "snap-mandatory",
        "px-4",
    ],
    tablet: tw![
        "tablet:px-6",
    ],
}
```

Rationale: the component is only ever in the render tree when the touch branch is active, so the container styling belongs in `base` (no `hidden`, no `mobile:` prefixes). `px-4` is the shared horizontal padding; `tablet:px-6` is the one per-band delta (a touch more breathing room on the wider band). This mirrors the header/footer role-model pattern of "`BASE` is the common truth, bands carry only the width deltas".

- [ ] **Step 3: Verify mobile is unchanged in the browser**

The dev server rebuilds on save. Reload `http://localhost:8123/warcraft-hotkey-editor/` once at a mobile size and confirm the pager still works exactly as before:

```js
// In the Playwright/MCP browser, viewport 390x844:
await page.setViewportSize({ width: 390, height: 844 });
await page.goto('http://localhost:8123/warcraft-hotkey-editor/');
await page.evaluate(() => {
  const s = document.querySelector('section[aria-label="Mobile editor"]');
  const cs = getComputedStyle(s);
  return { display: cs.display, overflowY: cs.overflowY, scrollSnapType: cs.scrollSnapType, raceNav: !!document.querySelector('.mobile-race-nav') };
});
```
Expected: `{ display: "flex", overflowY: "auto", scrollSnapType: "y mandatory", raceNav: true }` — identical behaviour to before, one unit per snap screen, race nav present.

- [ ] **Step 4: Commit**

```bash
git add crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/mobile_editor/style/mod.rs
git commit -m "refactor: make mobile pager container band-agnostic

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```
(If gpg signing is not unlocked, the commit is expected to be deferred — leave the change staged and continue.)

---

## Task 2: Mount the pager on the touch bands (mobile + tablet)

Widen the gate hook to `< 1280px` and rename it to say what it now means, then flip the branch. After Task 1 the container is ready, so the pager renders correctly on tablet immediately, and `UnitCommandGrids` falls through to its `base`/`tablet:` two-column grid (the `mobile:` carousel classes do not match at `>= 768px`).

**Files:**
- Modify: `editor_page/viewport.rs`
- Modify: `editor_page/mod.rs:21`, `editor_page/mod.rs:26-27`

**Interfaces:**
- Consumes: Task 1's band-agnostic container.
- Produces: `use_is_touch_viewport() -> bool` (true for `width <= 1279.98px`), consumed by `editor_page/mod.rs`.

- [ ] **Step 1: Rewrite `viewport.rs` to the touch band**

Replace the whole file with (only the media query, the identifiers, and the comments change; the listener logic is identical):

```rust
use dioxus::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

// Matches the touch bands (`mobile` `< 768px` plus `tablet` `768-1279px`, i.e.
// everything below the `laptop` band) declared in tailwind.css, so the runtime
// mount decision agrees with the CSS bands exactly.
const TOUCH_MEDIA_QUERY: &str = "(max-width: 1279.98px)";

struct TouchMediaListener {
    query_list: web_sys::MediaQueryList,
    change_closure: Closure<dyn FnMut(web_sys::MediaQueryListEvent)>,
}

impl Drop for TouchMediaListener {
    fn drop(&mut self) {
        let callback = self.change_closure.as_ref().unchecked_ref();
        let _ = self
            .query_list
            .remove_event_listener_with_callback("change", callback);
    }
}

// Whether the viewport is currently in a touch band (mobile or tablet). Reactive:
// the returned bool updates when the viewport crosses the breakpoint. The `change`
// event fires from the browser event loop (never during a Dioxus render/commit), so
// writing the signal from its callback is safe.
pub(super) fn use_is_touch_viewport() -> bool {
    let mut is_touch = use_signal(|| {
        web_sys::window()
            .and_then(|window| window.match_media(TOUCH_MEDIA_QUERY).ok().flatten())
            .map(|query_list| query_list.matches())
            .unwrap_or(false)
    });

    use_hook(|| {
        let window = web_sys::window()?;
        let query_list = window.match_media(TOUCH_MEDIA_QUERY).ok().flatten()?;
        let change_closure = Closure::<dyn FnMut(web_sys::MediaQueryListEvent)>::new(
            move |event: web_sys::MediaQueryListEvent| {
                let matches = event.matches();
                if *is_touch.peek() != matches {
                    is_touch.set(matches);
                }
            },
        );
        let callback = change_closure.as_ref().unchecked_ref();
        let _ = query_list.add_event_listener_with_callback("change", callback);
        let listener = TouchMediaListener {
            query_list,
            change_closure,
        };
        Some(Rc::new(listener))
    });

    *is_touch.read()
}
```

- [ ] **Step 2: Update the import in `editor_page/mod.rs`**

Change line 21 from:
```rust
use viewport::use_is_mobile_viewport;
```
to:
```rust
use viewport::use_is_touch_viewport;
```

- [ ] **Step 3: Update the branch in `editor_page/mod.rs`**

Change lines 26-27 from:
```rust
    let is_mobile = use_is_mobile_viewport();
    if is_mobile {
```
to:
```rust
    let is_touch = use_is_touch_viewport();
    if is_touch {
```
(The `return rsx! { MobileEditor {} }` body and the desktop `Page` branch below are unchanged.)

- [ ] **Step 4: Verify the tablet pager in the browser at three widths**

The dev server rebuilds on save (a hook-level Rust change is non-hot-reloadable, so it does a full rebuild; reload once). For each width below, check the pager renders with side-by-side grids:

```js
async function checkTablet(page, w, h) {
  await page.setViewportSize({ width: w, height: h });
  await page.goto('http://localhost:8123/warcraft-hotkey-editor/');
  return await page.evaluate(() => {
    const grids = document.querySelector('.unit-command-grids');
    const dots = document.querySelector('.grid-carousel-dots');
    const override = document.querySelector('.hotkey-override-section');
    return {
      raceNav: !!document.querySelector('.mobile-race-nav'),
      gridsDisplay: grids ? getComputedStyle(grids).display : 'MISSING',
      dotsHidden: dots ? getComputedStyle(dots).display === 'none' : true,
      overrideWidth: override ? Math.round(override.getBoundingClientRect().width) : null,
    };
  });
}
// checkTablet(page, 810, 1080) and (1024, 800) and (1279, 800)
```
Expected at each width: `raceNav: true`, `gridsDisplay: "grid"` (the two-column grid, NOT `"flex"` which is the mobile carousel), `dotsHidden: true`, and `overrideWidth` near the card content width (full-width). If `gridsDisplay` is `"flex"` at any of these widths, the `mobile:` band is leaking — recheck the band definitions in `tailwind.css`.

- [ ] **Step 5: Verify desktop is unchanged**

```js
await page.setViewportSize({ width: 1440, height: 900 });
await page.goto('http://localhost:8123/warcraft-hotkey-editor/');
await page.evaluate(() => ({
  desktopTabs: !!document.querySelector('.editor-tabs-bar'),
  pager: !!document.querySelector('section[aria-label="Mobile editor"]'),
}));
```
Expected: `{ desktopTabs: true, pager: false }` — laptop and up still render the desktop layout, no pager.

- [ ] **Step 6: Commit**

```bash
git add crates/hotkey-editor/src/components/app/components/shell/components/editor_page/viewport.rs \
        crates/hotkey-editor/src/components/app/components/shell/components/editor_page/mod.rs
git commit -m "feat: render the mobile pager on the tablet band

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

## Task 3: Cap and centre the unit card on tablet

At landscape tablet widths the card is a full-width `@container`, so its `cqi`-driven interior scales up with width and gets oversized near 1279px. Cap the card (and therefore its `cqi` reference width) on the `tablet:` band and centre it, leaving quiet margins. Mobile stays full-width.

**Files:**
- Modify: `editor_page/components/mobile_editor/components/pager_card_host/style/mod.rs`

**Interfaces:**
- Consumes: Task 2 (the card only renders on tablet once the pager is mounted there).
- Produces: on tablet, each `pager-card-host` is capped at ~896px and horizontally centred; the card's `cqi` interior scales off the capped width.

- [ ] **Step 1: Read the current file to confirm the starting point**

Run: `cat crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/mobile_editor/components/pager_card_host/style/mod.rs`
Expected `base`: `tw!["@container", "flex", "flex-col", "h-full", "min-w-0", "shrink-0", "snap-start", "py-4"]`.

- [ ] **Step 2: Add the tablet width cap**

Rewrite the file to:

```rust
use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "h-full",
        "min-w-0",
        "shrink-0",
        "snap-start",
        "py-4",
    ],
    tablet: tw![
        "tablet:w-full",
        "tablet:max-w-4xl",
        "tablet:self-center",
    ],
}
```

`max-w-4xl` is 896px (the spec's "~900px" start, expressed as a real token). `self-center` centres the host on the pager section's cross axis (the section is `flex flex-col`, so its children otherwise stretch to full width). On `mobile` there is no cap — the card stays full-width.

- [ ] **Step 3: Verify the cap and centring at landscape width**

```js
await page.setViewportSize({ width: 1279, height: 800 });
await page.goto('http://localhost:8123/warcraft-hotkey-editor/');
await page.evaluate(() => {
  const host = document.querySelector('.pager-card-host');
  const r = host.getBoundingClientRect();
  return { width: Math.round(r.width), left: Math.round(r.left), right: Math.round(window.innerWidth - r.right) };
});
```
Expected at 1279px: `width` ≈ 896 (capped, not ~1247), and `left` ≈ `right` (centred, roughly equal margins). Then eyeball it: two 4-column grids side by side should read as comfortable touch tiles, not oversized. If landscape feels too narrow, bump `tablet:max-w-4xl` to `tablet:max-w-5xl` (1024px) and re-check; if too wide, drop to `tablet:max-w-3xl` (768px). This is the sanctioned live tune from the spec.

- [ ] **Step 4: Verify portrait stays full-width**

```js
await page.setViewportSize({ width: 810, height: 1080 });
await page.goto('http://localhost:8123/warcraft-hotkey-editor/');
await page.evaluate(() => {
  const host = document.querySelector('.pager-card-host');
  return Math.round(host.getBoundingClientRect().width);
});
```
Expected: ≈ 762 (810 minus the section's `tablet:px-6` = 24px each side), i.e. the card fills the width at portrait (below the 896px cap), no wasted margins.

- [ ] **Step 5: Commit**

```bash
git add crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/mobile_editor/components/pager_card_host/style/mod.rs
git commit -m "feat: cap and centre the tablet unit card

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

## Task 4: Final touch-drag and gate verification

Confirm the core interaction (dragging a button between grid cells) still works by touch on the tablet layout, and run the full gate.

**Files:** none (verification only).

- [ ] **Step 1: Verify a touch drag on a grid tile at tablet width**

At `1024x800`, open the editor, then perform a real stepped pointer drag from one command-card tile to an empty cell (mirror the mechanics in `docs/AGENTS.md` — record a pending drag on pointerdown, promote on the first `pointermove` past the threshold; use `page.mouse.move(..., { steps })`, never `dragTo`). Expected: the follower appears, the target cell shows the ring, and on release the button moves (the URL/state updates). This exercises that the reused `GridEditor` variants drag correctly inside the side-by-side tablet grid.

- [ ] **Step 2: Run the full gate (dev server stopped or user-run)**

Do NOT run this while `moon run :dev` is up. Either ask the user to run it, or stop the dev server first, then:

Run: `moon run :ci`
Expected: green (fmt, tw-lint, clippy, keybinds tests, wasm build, e2e). Note: the mobile e2e spec runs at a mobile width and is unaffected; there is no tablet e2e in scope for this task.

- [ ] **Step 3: Commit any lint/fmt fixups the gate required**

```bash
git add -A
git commit -m "chore: gate fixups for tablet layout

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

## Notes for the implementer

- The whole feature is three style/branch edits plus verification. If a tablet-width check shows the carousel instead of the grid, the cause is always the same: a `mobile:`-prefixed class matching at `>= 768px`, which the disjoint band definitions in `tailwind.css` forbid — recheck those band boundaries, do not add tablet overrides to force it.
- `MobileEditor` deliberately keeps its name though it now serves tablet too ("big mobile"). Do not rename the subtree.
- Reused unchanged, do not touch: `MobileRaceNav`, `PagerCard`/`PagerCardHeader`, `UnitCommandGrids` and its menu wrappers, `GridCarouselDots`, `HotkeyOverrideSection`, the three `*GridEditor` variants, and `warcraft_keybinds::UnitSlotContainers`.
