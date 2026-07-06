import { expect, test, type Locator, type Page } from "@playwright/test";

// Regression guard for the grid-editor drag/drop styling, pinned to the exact
// production look (https://clemenscodes.github.io/warcraft-hotkey-editor).
//
// The bugs this locks:
//   - The drag-over gold used to sit as a box-shadow RING stacked on top of the
//     tile's own (too-thin) border; production REPLACES the border color instead.
//   - The lifted source tile was styled differently from empty drop targets;
//     production makes them identical (2px dashed muted-slate, 6px radius).
//   - Every tile rendered nearly square (~1px radius) because the painter's `cqi`
//     border/radius resolved against the tiny tile-face instead of a full tile;
//     production tiles are 6px-rounded with a 2px border.
//   - In the off-state picker, lifting the off button turned EVERY target solid;
//     production keeps idle targets golden-DASHED (adding only a soft glow) and
//     turns solid only the single tile under the cursor.
//
// Production truth, asserted below as computed CSS:
//   - resting + lifted-source + empty-target: ~6px radius, 2px border.
//   - lifted source: 2px DASHED muted-slate (#4a7090 = rgb(74,112,144)),
//     identical to an empty drop target — no source/target distinction.
//   - filled target under the cursor: 2px SOLID gold (#ffce63 = rgb(255,206,99)),
//     border color replaced, NO gold box-shadow ring on the Host.
//   - off-state picker: targets are golden-dashed at rest AND while dragging (idle),
//     gaining only a 12px glow; the tile under the cursor goes solid gold + 28px glow.
//
// The drag look is driven by data-attributes the interaction sets on the Host
// `.grid-editor-tile` (`data-dragging-source`, `data-drag-over`) and the painter
// (`data-drop-target`) — the exact markers the real pointer drag toggles. A
// synthetic Playwright drag never paints its own intermediate frame (press→move→
// release resolves faster than a render, and a paused drag is dropped), so these
// tests set those markers directly and assert the resulting computed CSS — the
// precise contract the fix restored. The drag *behaviour* (that the pointer drag
// sets these markers and commits a move) is covered by drag-drop.spec.ts and
// archmage-qwer-rearrange.spec.ts.

const SLATE = "rgb(74, 112, 144)"; // --color-warcraft-blue-slate #4a7090
const GOLD = "rgb(255, 206, 99)"; // --color-warcraft-gold #ffce63

// Read computed styles while a data-attribute is set on a tile's drag Host, then
// restore the DOM. `painter` is the .filled-tile/.empty-tile; `host` is its
// wrapping .grid-editor-tile (which carries the drag markers and, for the source,
// draws the ghost border itself).
async function styleWithHostMarker(
  tile: Locator,
  marker: "data-dragging-source" | "data-drag-over",
) {
  return tile.evaluate((painter, markerName) => {
    const host = painter.closest(".grid-editor-tile") as HTMLElement;
    host.setAttribute(markerName, "true");
    // Force a full style recalc. Setting an attribute on an ancestor does not
    // reliably invalidate a descendant's `[ancestor-attr] .descendant` color/
    // shadow match in headless Chromium (width invalidates, color does not); the
    // real Dioxus re-render triggers it, and re-attaching the node here does too.
    const parent = painter.parentNode!;
    const next = painter.nextSibling;
    parent.removeChild(painter);
    parent.insertBefore(painter, next);
    const painterStyle = getComputedStyle(painter);
    const hostStyle = getComputedStyle(host);
    const result = {
      painterBorderColor: painterStyle.borderTopColor,
      painterBorderStyle: painterStyle.borderTopStyle,
      painterBorderWidth: painterStyle.borderTopWidth,
      painterRadius: painterStyle.borderTopLeftRadius,
      hostBorderColor: hostStyle.borderTopColor,
      hostBorderStyle: hostStyle.borderTopStyle,
      hostBorderWidth: hostStyle.borderTopWidth,
      hostRadius: hostStyle.borderTopLeftRadius,
      hostShadow: hostStyle.boxShadow,
    };
    host.setAttribute(markerName, "false");
    return result;
  }, marker);
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
    return page.locator('[data-grid-id="Command card"] .filled-tile').first();
  }
  function firstEmpty(page: Page): Locator {
    return page.locator('[data-grid-id="Command card"] .empty-tile').first();
  }

  test.beforeEach(async ({ page }) => {
    await page.goto("/warcraft-hotkey-editor/");
    await page.locator(".unit-card").first().waitFor();
    await page.locator(".unit-card").first().click();
    await page.locator('[data-grid-id="Command card"] .filled-tile').first().waitFor();
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

  test("the lifted source tile is a rounded muted-slate dashed ghost", async ({ page }) => {
    const s = await styleWithHostMarker(firstFilled(page), "data-dragging-source");
    // The Host draws the ghost (the painter's children are hidden while lifted):
    // a 2px dashed muted-slate border with the same 6px rounding an empty drop
    // target wears, so the source is not distinguished from the targets.
    expect(s.hostBorderColor).toBe(SLATE);
    expect(s.hostBorderStyle).toBe("dashed");
    expect(isTwoPx(s.hostBorderWidth)).toBe(true);
    expect(isRounded(s.hostRadius)).toBe(true);
  });

  // THE root-cause guard. The drag decides which tile is under the cursor with
  // `element_from_point(cursor).closest("[data-grid-row]")`, and that attribute is
  // on the painter. Hiding the lifted painter with `visibility:hidden` (the old
  // `invisible`) made it NON-hit-testable, so hovering the source found no tile —
  // the source never became its own drop target and never lit up gold. It must be
  // hidden with `opacity:0` instead, which stays hit-testable.
  test("the lifted source stays hit-testable so it can be its own drop target", async ({ page }) => {
    const tile = firstFilled(page);
    await tile.scrollIntoViewIfNeeded(); // elementFromPoint only sees the viewport
    const found = await tile.evaluate((painter) => {
      const host = painter.closest(".grid-editor-tile") as HTMLElement;
      host.setAttribute("data-dragging-source", "true");
      const box = painter.getBoundingClientRect();
      const hit = document.elementFromPoint(
        Math.round(box.x + box.width / 2),
        Math.round(box.y + box.height / 2),
      );
      const tile = hit && hit.closest("[data-grid-row]");
      host.setAttribute("data-dragging-source", "false");
      return { hitTestable: !!tile, matchesPainter: tile === painter };
    });
    expect(found.hitTestable).toBe(true);
    expect(found.matchesPainter).toBe(true);
  });

  // When the cursor is over the source itself during a drag (a minimal drag that
  // never leaves the source, or one dragged back onto its origin), the source is
  // its own drop target and must light up gold like any other hovered target —
  // not stay slate. Requires the hit-test fix above plus the gold rule.
  test("the source lights up gold when it is the tile under the cursor", async ({ page }) => {
    const border = await firstFilled(page).evaluate((painter) => {
      const host = painter.closest(".grid-editor-tile") as HTMLElement;
      host.setAttribute("data-dragging-source", "true");
      host.setAttribute("data-drag-over", "true");
      const cs = getComputedStyle(host);
      const result = { color: cs.borderTopColor, style: cs.borderTopStyle, width: cs.borderTopWidth };
      host.setAttribute("data-dragging-source", "false");
      host.setAttribute("data-drag-over", "false");
      return result;
    });
    expect(border.color).toBe(GOLD);
    expect(border.style).toBe("dashed");
    expect(isTwoPx(border.width)).toBe(true);
  });

  // A filled tile keeps its state while a drag hovers it, so a `data-drag-over`
  // marker reproduces its real under-cursor look exactly (an empty tile instead
  // changes to the `DropTarget` state, a class only a live drag can paint, so the
  // empty under-cursor look is covered by the off-state picker's glow test and by
  // the fact that the source ghost — verified above — shares the empty target's
  // look). This test is the guard for both the ring removal and the gold replace.
  test("a filled target under the cursor gets a solid gold border, no ring", async ({ page }) => {
    const s = await styleWithHostMarker(firstFilled(page), "data-drag-over");
    // The filled tile keeps its state while a drag hovers it; the border is
    // replaced with 2px solid gold — no second border, no box-shadow ring.
    expect(s.painterBorderColor).toBe(GOLD);
    expect(s.painterBorderStyle).toBe("solid");
    expect(isTwoPx(s.painterBorderWidth)).toBe(true);
    expect(s.hostShadow).not.toContain("255, 206, 99");
  });
});

test.describe("Off-state position picker styling matches production", () => {
  const ANCHOR = ".alt-position-picker-grid-anchor";

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

  // Read a picker target's look while the given drag markers are set on it, forcing
  // the descendant recalc headless Chromium skips on ancestor setAttribute.
  async function pickerTargetStyle(
    page: Page,
    markers: { dropTarget?: boolean; dragOver?: boolean },
  ) {
    return page.locator(`${ANCHOR} .empty-tile`).first().evaluate((painter, m) => {
      const host = painter.closest(".grid-editor-tile") as HTMLElement;
      if (m.dropTarget) painter.setAttribute("data-drop-target", "true");
      if (m.dragOver) host.setAttribute("data-drag-over", "true");
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
      painter.setAttribute("data-drop-target", "false");
      host.setAttribute("data-drag-over", "false");
      return result;
    }, markers);
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

    // A non-draggable occupied tile (another ability) is dimmed out.
    const dimmed = page.locator(`${ANCHOR} .grid-editor-tile[data-draggable="false"] .filled-tile`).first();
    await expect(dimmed).toHaveCSS("opacity", "0.32");

    // The draggable off-state button itself carries the gold border.
    const button = page.locator(`${ANCHOR} .grid-editor-tile[data-draggable="true"] .filled-tile`).first();
    await expect(button).toHaveCSS("border-top-color", GOLD);
  });

  test("idle targets during a drag STAY golden-dashed, gaining only a soft glow", async ({
    page,
  }) => {
    await openPicker(page);
    // A target that is a drop candidate but NOT under the cursor: the regression
    // turned all of these solid on lift. Production keeps them dashed + a 12px glow.
    const idle = await pickerTargetStyle(page, { dropTarget: true });
    expect(idle.style).toBe("dashed");
    expect(idle.width).toBe("2px");
    expect(idle.shadow).toContain("12px");
    // Not the big under-cursor glow.
    expect(idle.shadow).not.toContain("28px");
  });

  test("the target under the cursor goes solid gold with the big glow", async ({ page }) => {
    await openPicker(page);
    // The real under-cursor tile carries both markers: the Host's data-drag-over
    // and the painter's data-drop-target. It out-specifies the idle rule → solid.
    const hovered = await pickerTargetStyle(page, { dropTarget: true, dragOver: true });
    expect(hovered.style).toBe("solid");
    expect(hovered.color).toBe(GOLD);
    // The signature golden glow: a wide (28px) gold outer box-shadow.
    expect(hovered.shadow).toContain("28px");
    // The INNER glow: a gold gradient fills the tile. The regression left this as
    // `background-image: none` because `bg-panel-gold-diag-32-2` put a bare panel
    // COLOR in `background-image` (invalid → dropped by the browser), so the tile
    // only glowed from the border outward. It must be a valid gold gradient.
    expect(hovered.backgroundImage).toContain("gradient");
  });

  // The same source-as-target bug hit the off-state grid: the lifted off button
  // (a filled tile that hides its painter) must stay hit-testable and light up
  // solid gold + the big glow when it is under the cursor — drawn by its Host,
  // since its painter is hidden.
  test("the lifted off button lights up gold when it is under the cursor", async ({ page }) => {
    await openPicker(page);
    const button = page.locator(`${ANCHOR} .grid-editor-tile[data-draggable="true"]`).first();
    await button.scrollIntoViewIfNeeded(); // elementFromPoint only sees the viewport

    // Root cause: the lifted off button's painter must remain hit-testable.
    const hitTestable = await button.evaluate((host) => {
      host.setAttribute("data-dragging-source", "true");
      const painter = host.querySelector(".filled-tile") as HTMLElement;
      const box = painter.getBoundingClientRect();
      const hit = document.elementFromPoint(
        Math.round(box.x + box.width / 2),
        Math.round(box.y + box.height / 2),
      );
      const found = !!(hit && hit.closest("[data-grid-row]"));
      host.setAttribute("data-dragging-source", "false");
      return found;
    });
    expect(hitTestable).toBe(true);

    // Under the cursor it goes solid gold with the big glow, like any target.
    const style = await button.evaluate((host) => {
      host.setAttribute("data-dragging-source", "true");
      host.setAttribute("data-drag-over", "true");
      const cs = getComputedStyle(host);
      const result = { color: cs.borderTopColor, style: cs.borderTopStyle, shadow: cs.boxShadow };
      host.setAttribute("data-dragging-source", "false");
      host.setAttribute("data-drag-over", "false");
      return result;
    });
    expect(style.color).toBe(GOLD);
    expect(style.style).toBe("solid");
    expect(style.shadow).toContain("28px");
  });
});
