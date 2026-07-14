import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";
const LS_KEY = "warcraft-hotkey-editor.custom-keys";

test.describe("Hotkey editing", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator(".unit-card").first().click();
    await page.locator(".filled-tile").first().waitFor();
    await page.locator(".filled-tile").first().click();
    await page.locator(".normal-override-key, .special-override-key").waitFor();
  });

  test("clicking the key cell opens the key picker", async ({ page }) => {
    await page.locator(".normal-override-key, .special-override-key").click();
    await page.locator(".key-picker-board").waitFor();
  });

  test("picking a key updates the cell display and writes hotkey=Q to localStorage", async ({ page }) => {
    await page.locator(".normal-override-key, .special-override-key").click();
    await page.locator(".key-picker-board").waitFor();
    await page.locator('.key-picker-board').getByRole('button', { name: /^Q/ }).click();
    await expect(page.locator(".key-picker-board")).not.toBeVisible();
    await expect(page.locator(".normal-override-key, .special-override-key")).toContainText("Q");
    const stored = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
    expect(stored).toContain("hotkey=Q");
  });

  test("Escape while picker is open cancels without changing localStorage", async ({ page }) => {
    const storedBefore = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
    await page.locator(".normal-override-key, .special-override-key").click();
    await page.locator(".key-picker-board").waitFor();
    await page.keyboard.press("Escape");
    await expect(page.locator(".key-picker-board")).not.toBeVisible();
    const storedAfter = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
    expect(storedAfter).toBe(storedBefore);
  });

  test("double-clicking a hero research-menu ability opens its research hotkey picker", async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('input[type="search"]').fill("Archmage");
    await page.locator(".unit-card").filter({ hasText: "Archmage" }).first().click();

    const researchTile = page
      .locator(".grid-editor", {
        has: page.locator(".grid-heading", { hasText: "Research menu" }),
      })
      .locator(".filled-tile")
      .first();
    await researchTile.waitFor();
    await researchTile.dblclick();

    await page.locator(".key-picker-board").waitFor();
    await page.locator('.key-picker-board').getByRole('button', { name: /^Y/ }).click();
    await expect(page.locator(".key-picker-board")).not.toBeVisible();

    const stored = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
    expect(stored?.toLowerCase()).toContain("researchhotkey=y");
  });

  test("double-clicking an ability icon opens the key picker and assigns a key", async ({ page }) => {
    await page.locator(".filled-tile").first().dblclick();
    await page.locator(".key-picker-board").waitFor();
    await page.locator('.key-picker-board').getByRole('button', { name: /^Q/ }).click();
    await expect(page.locator(".key-picker-board")).not.toBeVisible();
    await expect(page.locator(".normal-override-key, .special-override-key")).toContainText("Q");
    const stored = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
    expect(stored).toContain("hotkey=Q");
  });
});
