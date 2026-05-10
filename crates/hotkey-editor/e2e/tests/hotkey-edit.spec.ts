import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";
const LS_KEY = "warcraft-hotkey-editor.custom-keys";

test.describe("Hotkey editing", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator(".unit-card").first().click();
    await page.locator(".grid-tile.has-ability").first().waitFor();
    await page.locator(".grid-tile.has-ability").first().click();
    await page.locator(".override-key-cell").waitFor();
  });

  test("clicking the key cell opens the key picker", async ({ page }) => {
    await page.locator(".override-key-cell").click();
    await page.locator(".key-picker-shell").waitFor();
  });

  test("picking a key updates the cell display and writes hotkey=Q to localStorage", async ({ page }) => {
    await page.locator(".override-key-cell").click();
    await page.locator(".key-picker-shell").waitFor();
    await page.locator('.key-picker-key[data-label="Q"]').click();
    await expect(page.locator(".key-picker-shell")).not.toBeVisible();
    await expect(page.locator(".override-key-cell")).toContainText("Q");
    const stored = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
    expect(stored).toContain("hotkey=Q");
  });

  test("Escape while picker is open cancels without changing localStorage", async ({ page }) => {
    const storedBefore = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
    await page.locator(".override-key-cell").click();
    await page.locator(".key-picker-shell").waitFor();
    await page.keyboard.press("Escape");
    await expect(page.locator(".key-picker-shell")).not.toBeVisible();
    const storedAfter = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
    expect(storedAfter).toBe(storedBefore);
  });
});
