import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

async function applyTemplateAndCascade(page: import("@playwright/test").Page) {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();

  await page.locator('[aria-label="Browse layout templates"]').click();
  await page.locator(".template-card", { hasText: "Default" }).click();
  await page.locator('[role="alertdialog"]').first().waitFor();

  await page.locator('[aria-label="Resolve conflicts"]').click();
  await page.locator(".apply-button", { hasText: /apply/i }).click();
  await page
    .locator('[role="alertdialog"]')
    .filter({ hasText: "Cascade applied" })
    .waitFor();
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();
}

test.describe("Gargoyle regression: Prioritize renders at (1,1) after cascade", () => {
  test.beforeEach(async ({ page }) => {
    await applyTemplateAndCascade(page);
  });

  // Regression: Prioritize (Aatp) has no default Buttonpos in the game data,
  // so before the fix the renderer skipped it entirely — the Gargoyle command
  // card never showed the Prioritize toggle.  The fix materializes (0,0) as a
  // fallback for toggle abilities (has_off_state) missing a default position,
  // and the cascade wraps rightward overflow into the next row's leftmost free
  // cell.  Row 0 of the Gargoyle is fully pinned by the standard commands
  // (Move/Stop/HoldPos/Attack), Patrol pins (0,1), so Prioritize lands at (1,1).
  test("Gargoyle command card shows Prioritize at (1,1) after cascade", async ({
    page,
  }) => {
    await page.locator('.race-tabs [class*="undead-race-tab"]').click();
    await page.locator('input[type="search"]').fill("ugar");
    const gargoyle = page.locator(".unit-card").filter({ hasText: "ugar" });
    await gargoyle.waitFor();
    await gargoyle.click();
    await page.locator(".filled-tile").first().waitFor();

    const cell = (col: number, row: number) =>
      page
        .locator(".grid-editor", {
          has: page.locator(".grid-heading", { hasText: "Command card" }),
        })
        .locator(".grid-editor-tile")
        .nth(row * 4 + col);

    await expect(cell(1, 1).locator("img")).toHaveAttribute("alt", "Prioritize");
  });
});
