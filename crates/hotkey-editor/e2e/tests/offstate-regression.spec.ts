import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

// Sets up the full pre-condition: default template applied, cascade resolved.
// After this, every unit's abilities are at their post-cascade positions.
async function applyTemplateAndCascade(page: import("@playwright/test").Page) {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();

  await page.locator('[aria-label="Browse layout templates"]').click();
  await page.locator(".templates-dialog-shell .wc3-dialog-body button", { hasText: "Default" }).click();
  await page.locator('[role="alertdialog"]').first().waitFor();

  await page.locator('[aria-label="Resolve conflicts"]').click();
  await page.locator(".resolve-info-dialog").waitFor();
  await page.locator(".resolve-info-dialog button", { hasText: "Apply" }).click();
  await page
    .locator('[role="alertdialog"]')
    .filter({ hasText: "Cascade applied" })
    .waitFor();
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
    await page.locator('.race-tab[data-race="neutral"]').click();
    await page.locator('input[type="search"]').fill("ndrs");
    await page.locator(".unit-card").filter({ hasText: "Draenei Seer" }).waitFor();
    await page.locator(".unit-card").filter({ hasText: "Draenei Seer" }).click();
    await page.locator(".grid-tile.has-ability").first().waitFor();

    const sourceCell = page.locator(
      '[data-grid-section="Command card"][data-grid-col="3"][data-grid-row="2"]',
    );
    const targetCell = page.locator(
      '[data-grid-section="Command card"][data-grid-col="0"][data-grid-row="2"]',
    );

    await expect(sourceCell).toHaveClass(/has-ability/);
    await expect(targetCell).not.toHaveClass(/has-ability/);

    await sourceCell.dragTo(targetCell);

    await expect(targetCell).toHaveClass(/has-ability/);
    await expect(sourceCell).not.toHaveClass(/has-ability/);

    await expect(
      page.locator('[role="alertdialog"]').filter({ hasText: /reserved.*off-state/i }),
    ).not.toBeVisible();
  });
});
