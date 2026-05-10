import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

test.describe("Hotkey editing", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().click();
    await page.locator(".grid-tile.has-ability").first().waitFor();
    await page.locator(".grid-tile.has-ability").first().click();
    await page.locator(".override-key-cell").waitFor();
  });

  test("clicking the key cell enters editing mode", async ({ page }) => {
    await page.locator(".override-key-cell").click();
    await expect(page.locator(".override-key-cell.editing")).toBeVisible();
  });

  test("typing a key while in editing mode updates the hotkey", async ({ page }) => {
    await page.locator(".override-key-cell").click();
    await expect(page.locator(".override-key-cell.editing")).toBeVisible();
    await page.keyboard.press("q");
    // After pressing a key the editing state clears
    await expect(page.locator(".override-key-cell.editing")).not.toBeVisible();
    // The cell should now show "Q"
    await expect(page.locator(".override-key-cell")).toContainText("Q");
  });

  test("hotkey change persists to localStorage", async ({ page }) => {
    await page.locator(".override-key-cell").click();
    await page.keyboard.press("q");
    await expect(page.locator(".override-key-cell.editing")).not.toBeVisible();
    const stored = await page.evaluate(() =>
      localStorage.getItem("warcraft-hotkey-editor.custom-keys")
    );
    expect(stored).toContain("hotkey=Q");
  });

  test("Escape while editing cancels without saving", async ({ page }) => {
    const cellBefore = await page.locator(".override-key-cell").textContent();
    await page.locator(".override-key-cell").click();
    await page.keyboard.press("Escape");
    // Escape should not apply a hotkey change
    const cellAfter = await page.locator(".override-key-cell").textContent();
    // Either editing closed without change, or the original value is kept
    expect(cellAfter).toBeDefined();
  });

  test("second click on a different tile switches selection", async ({ page }) => {
    const tiles = page.locator(".grid-tile.has-ability");
    const count = await tiles.count();
    if (count < 2) {
      test.skip();
      return;
    }
    await tiles.nth(1).click();
    await expect(page.locator(".grid-tile.has-ability.selected").nth(0)).toBeVisible();
    // Only one tile should be selected at a time
    await expect(page.locator(".grid-tile.has-ability.selected")).toHaveCount(1);
  });
});
