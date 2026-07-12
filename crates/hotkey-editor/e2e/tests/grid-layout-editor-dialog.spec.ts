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

  // Regression: the key picker is a second modal nested inside the editor. When it
  // mounted it grabbed focus, which made the base dialog fire a close on the outer
  // editor — so clicking any cell dismissed the whole editor instead of opening
  // the picker over it. The editor must stay open behind the picker.
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

  // Regression: dismissing the editor via the nested picker's focus grab left the
  // editing-cell signal stranded as set. On the next open the editor immediately
  // re-mounted the picker, which re-dismissed it — so after ever editing a key the
  // editor could never be opened again. It must reopen and stay interactive.
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

    // Close the editor, then reopen it — the crux of the regression.
    await page.keyboard.press("Escape");
    await expect(page.locator(".grid-layout-editor-dialog-content")).toHaveCount(0);

    await openButton.click();
    await expect(page.locator(".grid-layout-editor-dialog-content")).toBeVisible();

    // And it is still interactive: a cell still opens its picker.
    await page.locator(".layout-tile").nth(1).click();
    await page.locator(".key-picker-board").waitFor();
    await expect(page.locator(".grid-layout-editor-dialog-content")).toBeVisible();
  });
});
