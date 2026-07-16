import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

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

    await page.mouse.move(centerX, centerY);
    await page.mouse.down();
    await page.mouse.up();

    await page.mouse.move(centerX, centerY);
    await page.mouse.down({ clickCount: 2 });
    await page.mouse.move(centerX + 12, centerY, { steps: 3 });
    await page.mouse.move(centerX, centerY, { steps: 3 });
    await page.mouse.up({ clickCount: 2 });

    await page.waitForTimeout(250);
    await expect(page.locator(".key-picker-board")).toHaveCount(0);
  });

  test("a genuine double-click still opens the hotkey picker", async ({
    page,
  }) => {
    await page.locator(".filled-tile").first().dblclick();
    await page.locator(".key-picker-board").waitFor();
    await expect(page.locator(".key-picker-board")).toBeVisible();
  });
});
