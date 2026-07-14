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

test.describe("Off-state regression: Healing Wave drag after cascade", () => {
  test.beforeEach(async ({ page }) => {
    await applyTemplateAndCascade(page);
  });

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
