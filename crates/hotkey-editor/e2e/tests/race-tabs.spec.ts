import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

test.describe("Race tabs", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.waitForSelector(".unit-card");
  });

  test("Human tab is active by default", async ({ page }) => {
    const humanTab = page.locator('.race-tab[data-race="human"]');
    await expect(humanTab).toHaveAttribute("data-active", "true");
  });

  test("clicking Orc tab switches the active race", async ({ page }) => {
    await page.locator('.race-tab[data-race="orc"]').click();
    await expect(page.locator('.race-tab[data-race="orc"]')).toHaveAttribute("data-active", "true");
    await expect(page.locator('.race-tab[data-race="human"]')).toHaveAttribute("data-active", "false");
  });

  test("switching to Orc loads Orc units in the list", async ({ page }) => {
    await page.locator('.race-tab[data-race="orc"]').click();
    await expect(page.locator(".unit-card").first()).toBeVisible();
    const mainContent = page.locator(".main-content");
    await expect(mainContent).toHaveAttribute("data-race", "orc");
  });

  test("cycling through all race tabs updates the active state", async ({ page }) => {
    const races = ["orc", "nightelf", "undead", "neutral", "human"] as const;
    for (const race of races) {
      await page.locator(`.race-tab[data-race="${race}"]`).click();
      await expect(page.locator(`.race-tab[data-race="${race}"]`)).toHaveAttribute("data-active", "true");
    }
  });

  test("switching races selects a unit automatically", async ({ page }) => {
    await page.locator('.race-tab[data-race="undead"]').click();
    await expect(page.locator(".unit-card.selected")).toBeVisible();
  });
});
