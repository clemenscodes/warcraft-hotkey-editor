import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

test.describe("Navigation", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
  });

  test("switching to Orc race shows a unit named Grunt", async ({ page }) => {
    await page.locator('.race-tabs [class*="orc-race-tab"]').click();
    await page.locator(".unit-card").filter({ hasText: "Grunt" }).waitFor();
  });

  test("search input filters units to show only matches", async ({ page }) => {
    await page.locator('input[type="search"]').fill("Footman");
    await expect(page).toHaveURL(/search_query=Footman/);
    const remaining = page.locator(".unit-card");
    const count = await remaining.count();
    expect(count).toBeGreaterThanOrEqual(1);
    expect(count).toBeLessThan(20);
  });

  test("clearing search restores the full unit count", async ({ page }) => {
    const countBefore = await page.locator(".unit-card").count();
    await page.locator('input[type="search"]').fill("Footman");
    await expect(page).toHaveURL(/search_query=Footman/);
    await page.locator('input[type="search"]').clear();
    await expect(page).not.toHaveURL(/search_query=/);
    await expect(page.locator(".unit-card")).toHaveCount(countBefore);
  });

  test("a click outside the sidebar closes the filter panel", async ({ page }) => {
    await page.locator('.unit-list [aria-label="Filter"]').click();
    const panel = page.locator(".unit-list .search-dialog-filters");
    await expect(panel).toBeVisible();
    const card = await page.locator(".unit-detail").boundingBox();
    if (!card) throw new Error("unit detail not rendered");
    await page.mouse.click(card.x + card.width / 2, card.y + card.height / 2);
    await expect(panel).toHaveCount(0);
  });

  test("the clear button empties the search and hides itself", async ({ page }) => {
    const search = page.locator('.unit-list input[type="search"]');
    const clear = page.locator('.unit-list [aria-label="Clear search"]');
    await expect(clear).toHaveCount(0);
    await search.fill("Footman");
    await expect(page).toHaveURL(/search_query=Footman/);
    await clear.click();
    await expect(search).toHaveValue("");
    await expect(page).not.toHaveURL(/search_query=/);
    await expect(clear).toHaveCount(0);
  });

  test("switching to Campaign mode shows a non-empty unit list", async ({ page }) => {
    await page.locator('.unit-list [aria-label="Filter"]').click();
    await page.locator(".mode-group button", { hasText: "Campaign" }).click();
    await page.locator(".unit-card").first().waitFor();
    expect(await page.locator(".unit-card").count()).toBeGreaterThan(0);
  });
});
