import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

test.describe("Navigation", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
  });

  test("switching to Orc race shows a unit named Grunt", async ({ page }) => {
    await page.locator('.race-tab[data-race="orc"]').click();
    await page.locator(".unit-card").filter({ hasText: "Grunt" }).waitFor();
  });

  test("search input filters units to show only matches", async ({ page }) => {
    await page.locator('input[type="search"]').fill("Footman");
    await page.locator('.unit-list[data-search-active="true"]').waitFor();
    const remaining = page.locator(".unit-card");
    const count = await remaining.count();
    expect(count).toBeGreaterThanOrEqual(1);
    expect(count).toBeLessThan(20);
  });

  test("clearing search restores the full unit count", async ({ page }) => {
    const countBefore = await page.locator(".unit-card").count();
    await page.locator('input[type="search"]').fill("Footman");
    await page.locator('.unit-list[data-search-active="true"]').waitFor();
    await page.locator('input[type="search"]').clear();
    await page.locator('.unit-list[data-search-active="false"]').waitFor();
    await expect(page.locator(".unit-card")).toHaveCount(countBefore);
  });

  test("switching to Campaign mode shows a non-empty unit list", async ({ page }) => {
    await page.locator("button", { hasText: "Campaign" }).click();
    await page.locator(".unit-card").first().waitFor();
    expect(await page.locator(".unit-card").count()).toBeGreaterThan(0);
  });
});
