import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

// Sets up the full pre-condition: default template applied, cascade resolved.
// After this, every unit's abilities are at their post-cascade positions.
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

test.describe("Off-state regression: Healing Wave drag after cascade", () => {
  test.beforeEach(async ({ page }) => {
    await applyTemplateAndCascade(page);
  });

  // Regression for two bugs fixed together:
  //   1. Healing Wave (AChv) had a phantom unbutton_position materialized even
  //      though it is a one-shot ability with no off-state.
  //   2. After the cascade moved Slow (ACsw) off row 2, its off-state ghost
  //      stayed at the original (0,2) cell, blocking the now-empty drop target.
  //
  // Expected layout on Draenei Seer (ndrs) after template + cascade:
  //   (0,2) <empty>                ← drop target (Y key; was blocked by Slow's ghost off-state before fix)
  //   (1,2) ACsw — Slow            ← moved here by cascade; off-state co-moved (fix 2)
  //   (2,2) ACba — Brilliance Aura
  //   (3,2) AChv — Healing Wave    ← source for this drag (moved here by cascade)
  test("Healing Wave can be dragged to the Y cell without off-state blocking", async ({
    page,
  }) => {
    await page.locator('.race-tabs [class*="neutral-race-tab"]').click();
    await page.locator('input[type="search"]').fill("ndrs");
    await page.locator(".unit-card").filter({ hasText: "Draenei Seer" }).waitFor();
    await page.locator(".unit-card").filter({ hasText: "Draenei Seer" }).click();
    await page.locator(".filled-tile").first().waitFor();

    const commandCard = page.locator(".grid-editor", {
      has: page.locator(".grid-heading", { hasText: "Command card" }),
    });
    const sourceCell = commandCard.locator(".grid-editor-tile").nth(11);
    const targetCell = commandCard.locator(".grid-editor-tile").nth(8);

    await expect(sourceCell.locator(".filled-tile")).toHaveCount(1);
    await expect(targetCell.locator(".filled-tile")).toHaveCount(0);

    await sourceCell.dragTo(targetCell);

    await expect(targetCell.locator(".filled-tile")).toHaveCount(1);
    await expect(sourceCell.locator(".filled-tile")).toHaveCount(0);

    await expect(
      page.locator('[role="alertdialog"]').filter({ hasText: /reserved.*off-state/i }),
    ).not.toBeVisible();
  });
});
