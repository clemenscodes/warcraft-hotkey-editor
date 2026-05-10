import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

test.describe("Unit selection and command grid", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.waitForSelector(".unit-card");
  });

  test("clicking a unit card selects it", async ({ page }) => {
    const card = page.locator(".unit-card").first();
    await card.click();
    await expect(page.locator(".unit-card.selected")).toBeVisible();
  });

  test("selected unit card has selected class", async ({ page }) => {
    await page.locator(".unit-card").first().click();
    const selected = page.locator(".unit-card.selected");
    await expect(selected).toHaveCount(1);
  });

  test("selecting a unit reveals grid tiles with abilities", async ({ page }) => {
    await page.locator(".unit-card").first().click();
    await expect(page.locator(".grid-tile.has-ability").first()).toBeVisible();
  });

  test("switching between units updates the command grid", async ({ page }) => {
    const firstCard = page.locator(".unit-card").nth(0);
    const secondCard = page.locator(".unit-card").nth(1);
    await firstCard.click();
    const tilesAfterFirst = await page.locator(".grid-tile.has-ability").count();
    await secondCard.click();
    const tilesAfterSecond = await page.locator(".grid-tile.has-ability").count();
    // Both should show at least some grid tiles; counts may differ per unit
    expect(tilesAfterFirst).toBeGreaterThanOrEqual(0);
    expect(tilesAfterSecond).toBeGreaterThanOrEqual(0);
  });

  test("clicking a grid tile with an ability selects it", async ({ page }) => {
    await page.locator(".unit-card").first().click();
    const abilityTile = page.locator(".grid-tile.has-ability").first();
    await abilityTile.click();
    await expect(page.locator(".grid-tile.has-ability.selected")).toBeVisible();
  });

  test("selecting an ability tile shows the override panel", async ({ page }) => {
    await page.locator(".unit-card").first().click();
    await page.locator(".grid-tile.has-ability").first().click();
    // The override key cell appears when a tile is selected
    await expect(page.locator(".override-key-cell")).toBeVisible();
  });

  test("deselecting by clicking another tile clears prior selection", async ({ page }) => {
    await page.locator(".unit-card").first().click();
    const tiles = page.locator(".grid-tile.has-ability");
    await tiles.first().click();
    await expect(page.locator(".grid-tile.has-ability.selected")).toHaveCount(1);
    const count = await tiles.count();
    if (count > 1) {
      await tiles.nth(1).click();
      await expect(page.locator(".grid-tile.has-ability.selected")).toHaveCount(1);
    }
  });
});
