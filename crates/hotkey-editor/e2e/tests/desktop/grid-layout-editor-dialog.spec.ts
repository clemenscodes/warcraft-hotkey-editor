import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

test.describe("Global hotkey layout editor", () => {
  test("applying the grid closes the dialog (mirrors Resolve)", async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();

    await page.locator('[aria-label="Edit global hotkey layout"]').click();
    await page.locator(".grid-layout-editor-dialog-content").waitFor();
    await expect(page.locator(".grid-layout-editor-dialog-content")).toBeVisible();

    await page.locator(".apply-button", { hasText: /apply/i }).click();
    await page
      .locator('[role="alertdialog"]')
      .filter({ hasText: /grid applied/i })
      .waitFor();
    await expect(page.locator(".grid-layout-editor-dialog-content")).toHaveCount(0);
  });

  test("opening a cell's key picker keeps the editor open behind it", async ({
    page,
  }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();

    await page.locator('[aria-label="Edit global hotkey layout"]').click();
    await page.locator(".grid-layout-editor-dialog-content").waitFor();

    await page.locator(".layout-tile").nth(0).click();
    await page.locator(".key-picker-board").waitFor();
    await expect(page.locator(".grid-layout-editor-dialog-content")).toBeVisible();
  });

  test("the editor reopens and works after a key was changed", async ({
    page,
  }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();

    const openButton = page.locator('[aria-label="Edit global hotkey layout"]');
    const firstCell = page.locator(".layout-tile").nth(0);

    await openButton.click();
    await page.locator(".grid-layout-editor-dialog-content").waitFor();
    await firstCell.click();
    await page.locator(".key-picker-board").waitFor();
    await page.keyboard.press("h");
    await expect(page.locator(".key-picker-board")).toHaveCount(0);
    await expect(firstCell).toHaveText("H");
    await expect(page.locator(".grid-layout-editor-dialog-content")).toBeVisible();

    await page.keyboard.press("Escape");
    await expect(page.locator(".grid-layout-editor-dialog-content")).toHaveCount(0);

    await openButton.click();
    await expect(page.locator(".grid-layout-editor-dialog-content")).toBeVisible();

    await page.locator(".layout-tile").nth(1).click();
    await page.locator(".key-picker-board").waitFor();
    await expect(page.locator(".grid-layout-editor-dialog-content")).toBeVisible();
  });
});
