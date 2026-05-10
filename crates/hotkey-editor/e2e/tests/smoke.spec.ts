import { expect, test } from "@playwright/test";

test.describe("App shell", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/warcraft-hotkey-editor/");
  });

  test("page loads and title is correct", async ({ page }) => {
    await expect(page).toHaveTitle(/Warcraft III Hotkey Editor/);
  });

  test("race tabs are visible", async ({ page }) => {
    await expect(page.locator("[data-race]").first()).toBeVisible();
  });

  test("unit list renders at least one unit card", async ({ page }) => {
    await expect(page.locator(".unit-card").first()).toBeVisible();
  });

  test("selecting a unit reveals the command grid", async ({ page }) => {
    await page.locator(".unit-card").first().click();
    await expect(page.locator(".grid-tile").first()).toBeVisible();
  });
});
