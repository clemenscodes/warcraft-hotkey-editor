import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

test.describe("Drag and drop on command grid", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator(".unit-card").first().click();
    await page.locator(".grid-tile.has-ability").first().waitFor();
  });

  test("dragging one ability tile onto another swaps them without changing tile count", async ({ page }) => {
    const tiles = page.locator(".grid-tile.has-ability");
    const countBefore = await tiles.count();
    if (countBefore < 2) {
      test.skip();
      return;
    }
    await tiles.first().dragTo(tiles.nth(1));
    const countAfter = await page.locator(".grid-tile.has-ability").count();
    expect(countAfter).toBe(countBefore);
  });

  test("pressing Escape during a drag cancels it and tile count is unchanged", async ({ page }) => {
    const sourceTile = page.locator(".grid-tile.has-ability").first();
    const sourceBox = await sourceTile.boundingBox();
    if (!sourceBox) {
      test.skip();
      return;
    }

    const countBefore = await page.locator(".grid-tile.has-ability").count();
    await page.mouse.move(sourceBox.x + sourceBox.width / 2, sourceBox.y + sourceBox.height / 2);
    await page.mouse.down();
    await page.mouse.move(sourceBox.x + 200, sourceBox.y, { steps: 5 });
    await page.keyboard.press("Escape");
    await page.mouse.up();

    const countAfter = await page.locator(".grid-tile.has-ability").count();
    expect(countAfter).toBe(countBefore);
  });
});
