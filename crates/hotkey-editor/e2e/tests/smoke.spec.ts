import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

test("app loads with correct title and a unit list", async ({ page }) => {
  await page.goto(APP);
  await expect(page).toHaveTitle(/Warcraft III Hotkey Editor/i);
  await page.locator(".unit-card").first().waitFor();
});

test("selecting a unit shows the command grid", async ({ page }) => {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();
  await page.locator(".unit-card").first().click();
  await page.locator(".grid-tile.has-ability").first().waitFor();
});
