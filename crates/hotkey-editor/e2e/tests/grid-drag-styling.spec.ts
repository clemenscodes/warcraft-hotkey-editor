import { expect, test, type Locator, type Page } from "@playwright/test";

// Regression guard for the grid-editor drag/drop styling, pinned to the exact
// production look (https://clemenscodes.github.io/warcraft-hotkey-editor).
//
// The bugs this locks:
//   - The drag-over gold used to sit as a box-shadow RING stacked on top of the
//     tile's own (too-thin) border; production REPLACES the border color instead.
//   - The lifted source tile was styled differently from empty drop targets;
//     production makes them identical (2px dashed deep-blue, 6px radius).
//   - Every tile rendered nearly square (~1px radius) because the painter's `cqi`
//     border/radius resolved against the tiny tile-face instead of a full tile;
//     production tiles are 6px-rounded with a 2px border.
//   - In the off-state picker, lifting the off button turned EVERY target solid;
//     production keeps idle targets golden-DASHED (adding only a soft glow) and
//     turns solid only the single tile under the cursor.
//
// Production truth, asserted below as computed CSS:
//   - resting + lifted-source + empty-target: ~6px radius, 2px border.
//   - lifted source: 2px DASHED deep-blue (#1a3a5c = rgb(26,58,92)),
//     identical to an empty drop target — no source/target distinction.
//   - filled target under the cursor: 2px SOLID gold (#ffce63 = rgb(255,206,99)),
//     border color replaced, NO gold box-shadow ring on the Host.
//   - off-state picker: targets are golden-dashed at rest AND while dragging (idle),
//     gaining only an 8px glow; the tile under the cursor goes solid gold + 14px glow.
//
// The drag look is driven by mounted OVERLAY-CHILD components the interaction adds
// inside the painter — `.dragging-source-ghost` and `.drag-over-ring` on the
// `.filled-tile`/`.empty-tile`, keyed off by the tile root via `:has(...)` (the empty
// drop target additionally mounts `.drop-target-overlay`). These replaced the old
// `data-*` attributes; there is no data attribute anywhere in the drag subsystem now.
// A synthetic Playwright drag never paints its own intermediate frame (press→move→
// release resolves faster than a render, and a paused drag is dropped), so these tests
// inject those overlay children directly and assert the resulting computed CSS — the
// precise contract the fix restored. The drag *behaviour* (that the pointer drag mounts
// these overlays and commits a move) is covered by drag-drop.spec.ts and
// archmage-qwer-rearrange.spec.ts.

const DEEP = "rgb(26, 58, 92)"; // --color-warcraft-blue-deep #1a3a5c
const GOLD = "rgb(255, 206, 99)"; // --color-warcraft-gold #ffce63

// The "Command card" grid, located by its heading instead of a positional attribute.
function commandGrid(page: Page): Locator {
  return page.locator(".grid-editor", {
    has: page.locator(".grid-heading", { hasText: "Command card" }),
  });
}

// Mount the given overlay-child classes inside a tile's painter, read the painter's
// (and its Host's) computed styles, then restore the DOM. Mounting a child is exactly
// what the live drag does; the painter root reacts to it via `:has(...)`.
async function styleWithOverlays(tile: Locator, overlayClasses: string[]) {
  return tile.evaluate((painter, classes) => {
    const added: HTMLElement[] = [];
    for (const overlayClass of classes) {
      const overlay = document.createElement("div");
      overlay.className = overlayClass;
      painter.appendChild(overlay);
      added.push(overlay);
    }
    // Force a full style recalc. Headless Chromium does not reliably re-evaluate a
    // `:has()` color/shadow match on a child insertion (width invalidates, color does
    // not); the real Dioxus re-render triggers it, and re-attaching the node here does
    // too.
    const parent = painter.parentNode!;
    const next = painter.nextSibling;
    parent.removeChild(painter);
    parent.insertBefore(painter, next);
    const host = painter.closest(".grid-editor-tile") as HTMLElement;
    const painterStyle = getComputedStyle(painter);
    const hostStyle = getComputedStyle(host);
    const result = {
      painterBorderColor: painterStyle.borderTopColor,
      painterBorderStyle: painterStyle.borderTopStyle,
      painterBorderWidth: painterStyle.borderTopWidth,
      painterRadius: painterStyle.borderTopLeftRadius,
      hostShadow: hostStyle.boxShadow,
    };
    for (const overlay of added) painter.removeChild(overlay);
    return result;
  }, overlayClasses);
}

// ~2px like production; cqi resolves sub-pixel and the engines serialize it
// differently, so round — still catches the ~1px "too thin" regression.
function isTwoPx(width: string): boolean {
  return Math.round(parseFloat(width)) === 2;
}

// Clearly rounded (production is 6px) — a comfortable floor that still fails hard
// on the ~1px "square tiles" regression, tolerating the cqi sub-pixel spread.
function isRounded(radius: string): boolean {
  return parseFloat(radius) >= 5;
}

test.describe("Grid editor drag styling matches production", () => {
  function firstFilled(page: Page): Locator {
    return commandGrid(page).locator(".filled-tile").first();
  }
  function firstEmpty(page: Page): Locator {
    return commandGrid(page).locator(".empty-tile").first();
  }

  test.beforeEach(async ({ page }) => {
    await page.goto("/warcraft-hotkey-editor/");
    await page.locator(".unit-card").first().waitFor();
    await page.locator(".unit-card").first().click();
    await commandGrid(page).locator(".filled-tile").first().waitFor();
  });

  test("resting tiles are rounded with a 2px border like production", async ({ page }) => {
    // The container-resolution regression rendered these ~1px radius / ~1px border
    // (square, hairline). Production is 6px radius / 2px border.
    for (const tile of [firstFilled(page), firstEmpty(page)]) {
      const style = await tile.evaluate((el) => {
        const cs = getComputedStyle(el);
        return { radius: cs.borderTopLeftRadius, width: cs.borderTopWidth };
      });
      expect(isRounded(style.radius)).toBe(true);
      expect(isTwoPx(style.width)).toBe(true);
    }
  });

  test("the lifted source tile is a rounded deep-blue dashed ghost", async ({ page }) => {
    // The lifted source mounts a `.dragging-source-ghost`; the painter root reacts by
    // turning its OWN border into a 2px dashed deep-blue ghost with the same 6px rounding
    // an empty drop target wears, so the source is not distinguished from the targets.
    const s = await styleWithOverlays(firstFilled(page), ["dragging-source-ghost"]);
    expect(s.painterBorderColor).toBe(DEEP);
    expect(s.painterBorderStyle).toBe("dashed");
    expect(isTwoPx(s.painterBorderWidth)).toBe(true);
    expect(isRounded(s.painterRadius)).toBe(true);
  });

  // THE root-cause guard. The drag decides which tile is under the cursor with
  // `element_from_point(cursor).closest(".grid-editor-tile")`. The lifted source hides
  // its painter's CHILDREN (`>*` invisible) but keeps the painter root itself visible, so
  // the source stays hit-testable and can become its own drop target. (The old bug hid
  // the whole painter with `visibility:hidden`, making it non-hit-testable.)
  test("the lifted source stays hit-testable so it can be its own drop target", async ({ page }) => {
    const tile = firstFilled(page);
    await tile.scrollIntoViewIfNeeded(); // elementFromPoint only sees the viewport
    const found = await tile.evaluate((painter) => {
      const overlay = document.createElement("div");
      overlay.className = "dragging-source-ghost";
      painter.appendChild(overlay);
      const box = painter.getBoundingClientRect();
      const hit = document.elementFromPoint(
        Math.round(box.x + box.width / 2),
        Math.round(box.y + box.height / 2),
      );
      const host = hit && hit.closest(".grid-editor-tile");
      const expectedHost = painter.closest(".grid-editor-tile");
      painter.removeChild(overlay);
      return { hitTestable: !!host, matchesHost: host === expectedHost };
    });
    expect(found.hitTestable).toBe(true);
    expect(found.matchesHost).toBe(true);
  });

  // When the cursor is over the source itself during a drag (a minimal drag that never
  // leaves the source, or one dragged back onto its origin), the source is its own drop
  // target and must light up gold like any other hovered target — not stay slate.
  // Requires the hit-test above plus the gold rule; the source keeps its dashed style.
  test("the source lights up gold when it is the tile under the cursor", async ({ page }) => {
    const s = await styleWithOverlays(firstFilled(page), ["dragging-source-ghost", "drag-over-ring"]);
    expect(s.painterBorderColor).toBe(GOLD);
    expect(s.painterBorderStyle).toBe("dashed");
    expect(isTwoPx(s.painterBorderWidth)).toBe(true);
  });

  // A filled tile keeps its state while a drag hovers it, so a mounted `.drag-over-ring`
  // reproduces its real under-cursor look exactly: the border is REPLACED with 2px solid
  // gold — no second border, no box-shadow ring on the Host.
  test("a filled target under the cursor gets a solid gold border, no ring", async ({ page }) => {
    const s = await styleWithOverlays(firstFilled(page), ["drag-over-ring"]);
    expect(s.painterBorderColor).toBe(GOLD);
    expect(s.painterBorderStyle).toBe("solid");
    expect(isTwoPx(s.painterBorderWidth)).toBe(true);
    expect(s.hostShadow).not.toContain("255, 206, 99");
  });
});

test.describe("Off-state position picker styling matches production", () => {
  const ANCHOR = ".hotkey-alt-position-picker-grid-anchor";

  // Open the off-state picker on the Footman (Defend is a toggle with an
  // off-state), returning once its embedded command grid is on screen.
  async function openPicker(page: Page): Promise<void> {
    await page.goto("/warcraft-hotkey-editor/?race=human&mode=melee&unit=hfoo");
    await page.locator(".filled-tile").first().waitFor();
    const control = page.locator('[aria-label="Edit off-state button position"]');
    const tiles = page.locator(".filled-tile");
    const count = await tiles.count();
    for (let index = 0; index < count; index += 1) {
      await tiles.nth(index).click();
      if (await control.count()) break;
    }
    await control.first().click();
    await page.locator(ANCHOR).waitFor();
  }

  // Read a picker target's look while the given overlay children are mounted inside it,
  // forcing the `:has()` recalc headless Chromium skips on a child insertion.
  async function pickerTargetStyle(page: Page, overlayClasses: string[]) {
    return page.locator(`${ANCHOR} .empty-tile`).first().evaluate((painter, classes) => {
      const added: HTMLElement[] = [];
      for (const overlayClass of classes) {
        const overlay = document.createElement("div");
        overlay.className = overlayClass;
        painter.appendChild(overlay);
        added.push(overlay);
      }
      const parent = painter.parentNode!;
      const next = painter.nextSibling;
      parent.removeChild(painter);
      parent.insertBefore(painter, next);
      const style = getComputedStyle(painter);
      const result = {
        style: style.borderTopStyle,
        width: style.borderTopWidth,
        color: style.borderTopColor,
        shadow: style.boxShadow,
        backgroundImage: style.backgroundImage,
      };
      for (const overlay of added) painter.removeChild(overlay);
      return result;
    }, overlayClasses);
  }

  test("empty targets are golden-dashed and non-target tiles are dimmed", async ({ page }) => {
    await openPicker(page);

    const emptyStyle = await page.locator(`${ANCHOR} .empty-tile`).first().evaluate((element) => {
      const style = getComputedStyle(element);
      return { style: style.borderTopStyle, width: style.borderTopWidth, color: style.borderTopColor };
    });
    expect(emptyStyle.style).toBe("dashed");
    expect(emptyStyle.width).toBe("2px");
    // Golden, not the resting blue-bright-deep — the picker paints its own accent.
    expect(emptyStyle.color).not.toBe("rgb(31, 74, 114)");

    // A non-draggable occupied tile (another ability, no `.draggable-marker`) is dimmed.
    const dimmed = page
      .locator(`${ANCHOR} .grid-editor-tile:not(:has(.draggable-marker)) .filled-tile`)
      .first();
    await expect(dimmed).toHaveCSS("opacity", "0.32");

    // The draggable off-state button itself (mounts `.draggable-marker`) carries the gold
    // border.
    const button = page
      .locator(`${ANCHOR} .grid-editor-tile:has(.draggable-marker) .filled-tile`)
      .first();
    await expect(button).toHaveCSS("border-top-color", GOLD);
  });

  test("idle targets during a drag STAY golden-dashed, gaining only a soft glow", async ({
    page,
  }) => {
    await openPicker(page);
    // A target that is a drop candidate but NOT under the cursor: the regression turned
    // all of these solid on lift. Production keeps them dashed + an 8px glow.
    const idle = await pickerTargetStyle(page, ["drop-target-overlay"]);
    expect(idle.style).toBe("dashed");
    expect(idle.width).toBe("2px");
    expect(idle.shadow).toContain("8px");
    // Not the big under-cursor glow.
    expect(idle.shadow).not.toContain("14px");
  });

  test("the target under the cursor goes solid gold with the big glow", async ({ page }) => {
    await openPicker(page);
    // The real under-cursor tile mounts both the drop-target overlay AND the drag-over
    // ring. It out-specifies the idle rule → solid.
    const hovered = await pickerTargetStyle(page, ["drop-target-overlay", "drag-over-ring"]);
    expect(hovered.style).toBe("solid");
    expect(hovered.color).toBe(GOLD);
    // The signature golden glow: the gold `shadow-ring` outer box-shadow (14px).
    expect(hovered.shadow).toContain("14px");
    // The INNER glow: a gold gradient fills the tile. The regression left this as
    // `background-image: none` because `bg-panel-gold-diag-32-2` put a bare panel
    // COLOR in `background-image` (invalid → dropped by the browser), so the tile
    // only glowed from the border outward. It must be a valid gold gradient.
    expect(hovered.backgroundImage).toContain("gradient");
  });

  // The same source-as-target bug hit the off-state grid: the lifted off button
  // (a filled tile) must stay hit-testable and light up solid gold + the big glow when
  // it is under the cursor.
  test("the lifted off button lights up gold when it is under the cursor", async ({ page }) => {
    await openPicker(page);
    const button = page.locator(`${ANCHOR} .grid-editor-tile:has(.draggable-marker)`).first();
    await button.scrollIntoViewIfNeeded(); // elementFromPoint only sees the viewport

    // Root cause: the lifted off button's painter root must remain hit-testable.
    const hitTestable = await button.evaluate((host) => {
      const painter = host.querySelector(".filled-tile") as HTMLElement;
      const overlay = document.createElement("div");
      overlay.className = "dragging-source-ghost";
      painter.appendChild(overlay);
      const box = painter.getBoundingClientRect();
      const hit = document.elementFromPoint(
        Math.round(box.x + box.width / 2),
        Math.round(box.y + box.height / 2),
      );
      const found = !!(hit && hit.closest(".grid-editor-tile"));
      painter.removeChild(overlay);
      return found;
    });
    expect(hitTestable).toBe(true);

    // Under the cursor it goes solid gold with the big glow, like any target.
    const style = await button.evaluate((host) => {
      const painter = host.querySelector(".filled-tile") as HTMLElement;
      const added: HTMLElement[] = [];
      for (const overlayClass of ["dragging-source-ghost", "drag-over-ring"]) {
        const overlay = document.createElement("div");
        overlay.className = overlayClass;
        painter.appendChild(overlay);
        added.push(overlay);
      }
      const parent = painter.parentNode!;
      const next = painter.nextSibling;
      parent.removeChild(painter);
      parent.insertBefore(painter, next);
      const cs = getComputedStyle(painter);
      const result = { color: cs.borderTopColor, style: cs.borderTopStyle, shadow: cs.boxShadow };
      for (const overlay of added) painter.removeChild(overlay);
      return result;
    });
    expect(style.color).toBe(GOLD);
    expect(style.style).toBe("solid");
    expect(style.shadow).toContain("14px");
  });
});
