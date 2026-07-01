import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

test.describe("Global hotkey layout editor", () => {
  test("applying the grid closes the dialog (mirrors Resolve)", async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();

    await page.locator('[aria-label="Edit global hotkey layout"]').click();
    await page.locator(".layout-editor-content").waitFor();
    await expect(page.locator(".layout-editor-content")).toBeVisible();

    await page.locator(".apply-button", { hasText: /apply/i }).click();
    await page
      .locator('[role="alertdialog"]')
      .filter({ hasText: /grid applied/i })
      .waitFor();
    await expect(page.locator(".layout-editor-content")).toHaveCount(0);
  });
});
