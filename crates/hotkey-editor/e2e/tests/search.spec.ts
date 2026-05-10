import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

test.describe("Unit search", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.waitForSelector(".unit-card");
  });

  test("search box is visible", async ({ page }) => {
    await expect(page.locator('input[type="search"]')).toBeVisible();
  });

  test("typing in search filters the unit list", async ({ page }) => {
    const countBefore = await page.locator(".unit-card").count();
    await page.locator('input[type="search"]').fill("Footman");
    await expect(page.locator(".unit-card")).not.toHaveCount(countBefore);
    await expect(page.locator(".unit-card").filter({ hasText: "Footman" })).toBeVisible();
  });

  test("search result shows only matched units", async ({ page }) => {
    await page.locator('input[type="search"]').fill("Archmage");
    await expect(page.locator(".unit-card").filter({ hasText: "Archmage" })).toBeVisible();
    const count = await page.locator(".unit-card").count();
    expect(count).toBeGreaterThanOrEqual(1);
    expect(count).toBeLessThan(20);
  });

  test("clearing search restores the full unit list", async ({ page }) => {
    const countBefore = await page.locator(".unit-card").count();
    await page.locator('input[type="search"]').fill("Footman");
    await page.locator('input[type="search"]').clear();
    await expect(page.locator(".unit-card")).toHaveCount(countBefore);
  });

  test("search works across race tab switch", async ({ page }) => {
    await page.locator('input[type="search"]').fill("Grunt");
    await page.locator('.race-tab[data-race="orc"]').click();
    await expect(page.locator(".unit-card").filter({ hasText: "Grunt" })).toBeVisible();
  });
});
