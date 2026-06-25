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

  test("double-clicking a hero research-menu ability opens its research hotkey picker", async ({ page }) => {
    // Regression: research-menu (hero learn-skill) tiles are selected in research
    // context, where the bindable field is the research hotkey, not the primary
    // Hotkey. The double-click effect must open the research hotkey picker for
    // these — previously it no-op'd because it only handled the primary field.
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('input[type="search"]').fill("Archmage");
    await page.locator(".unit-card").filter({ hasText: "Archmage" }).first().click();

    const researchTile = page
      .locator('[data-grid-section="Research menu"].grid-tile.has-ability')
      .first();
    await researchTile.waitFor();
    await researchTile.dblclick();

    // The picker must open (it did not, for research items, before the fix).
    await page.locator(".key-picker-shell").waitFor();
    await page.locator('.key-picker-key[data-label="Y"]').click();
    await expect(page.locator(".key-picker-shell")).not.toBeVisible();

    // It must write the RESEARCH hotkey field, proving the research picker opened
    // (not the primary Hotkey field).
    const stored = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
    expect(stored?.toLowerCase()).toContain("researchhotkey=y");
  });

  test("double-clicking an ability icon opens the key picker and assigns a key", async ({ page }) => {
    // Reuses Q (proven pickable on this exact target by the click-based test
    // above) so the assertion exercises the double-click path, not fixture luck.
    await page.locator(".grid-tile.has-ability").first().dblclick();
    await page.locator(".key-picker-shell").waitFor();
    await page.locator('.key-picker-key[data-label="Q"]').click();
    await expect(page.locator(".key-picker-shell")).not.toBeVisible();
    await expect(page.locator(".override-key-cell")).toContainText("Q");
    const stored = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
    expect(stored).toContain("hotkey=Q");
  });
});
