import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

// Regression: the Phoenix (hphx) game data lists Phoenix Fire (Apxf) as one of
// its abilities, but the in-game command card never shows a button for it. The
// domain layer hides it so the editor matches the live client; here we assert
// the rendered command card omits it while keeping the unit's other ability.
test.describe("Phoenix hides Phoenix Fire", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('input[type="search"]').fill("hphx");
    const phoenix = page.locator(".unit-card").filter({ hasText: "hphx" });
    await phoenix.waitFor();
    await phoenix.click();
    await page.locator(".grid-tile.has-ability").first().waitFor();
  });

  test("Phoenix command card omits Phoenix Fire (Apxf)", async ({ page }) => {
    const commandCard = page.locator('[data-grid-id="Command card"]');
    await expect(commandCard.locator('img[alt="Phoenix Fire"]')).toHaveCount(0);
  });

  test("Phoenix command card still shows its Phoenix ability", async ({ page }) => {
    const commandCard = page.locator('[data-grid-id="Command card"]');
    await expect(commandCard.locator('img[alt="Phoenix"]')).toHaveCount(1);
  });
});
