import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

// A grid tile opens the hotkey picker on a genuine double-click. But a single
// click followed by a drag gesture on the same tile must NOT open it: the
// browser combines the prior click with the drag's trailing synthetic click
// into a `dblclick`. Initiating a drag has to reset that double-click trigger.
test.describe("Drag does not trigger the double-click picker", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator(".unit-card").first().click();
    await page.locator(".filled-tile").first().waitFor();
  });

  test("a click then a drag on the same tile must not open the hotkey picker", async ({
    page,
  }) => {
    const tile = page.locator(".filled-tile").first();
    const box = await tile.boundingBox();
    if (!box) {
      test.skip();
      return;
    }
    const centerX = box.x + box.width / 2;
    const centerY = box.y + box.height / 2;

    // First: a single click that selects the tile and primes the browser's
    // double-click detection.
    await page.mouse.move(centerX, centerY);
    await page.mouse.down();
    await page.mouse.up();

    // Then: a drag gesture on the same tile that the browser counts as the
    // second click of a double-click (clickCount: 2 — exactly what happens when
    // a real user presses again within the double-click window). Press, move
    // past the 4px threshold to commit the drag, return to the origin, and
    // release. Initiating the drag must reset the double-click trigger so the
    // picker does not open.
    await page.mouse.move(centerX, centerY);
    await page.mouse.down({ clickCount: 2 });
    await page.mouse.move(centerX + 12, centerY, { steps: 3 });
    await page.mouse.move(centerX, centerY, { steps: 3 });
    await page.mouse.up({ clickCount: 2 });

    // Give any double-click-triggered dialog a chance to render, then assert it
    // did not open.
    await page.waitForTimeout(250);
    await expect(page.locator(".key-picker-shell")).toHaveCount(0);
  });

  test("a genuine double-click still opens the hotkey picker", async ({
    page,
  }) => {
    // Guard: the fix must only suppress the picker after a real drag, never on
    // an ordinary double-click.
    await page.locator(".filled-tile").first().dblclick();
    await page.locator(".key-picker-shell").waitFor();
    await expect(page.locator(".key-picker-shell")).toBeVisible();
  });
});
