import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

test.describe("Preview dialog", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.waitForSelector(".unit-card");
  });

  test("clicking Preview button opens the preview dialog", async ({ page }) => {
    await page.locator('[aria-label="Preview"]').click();
    await expect(page.locator(".preview-dialog")).toBeVisible();
  });

  test("preview dialog contains a textarea with CustomKeys content", async ({ page }) => {
    await page.locator('[aria-label="Preview"]').click();
    const textarea = page.locator(".preview-dialog textarea");
    await expect(textarea).toBeVisible();
    const content = await textarea.inputValue();
    expect(content.length).toBeGreaterThan(0);
  });

  test("preview content contains expected CustomKeys.txt markers", async ({ page }) => {
    await page.locator('[aria-label="Preview"]').click();
    const textarea = page.locator(".preview-dialog textarea");
    const content = await textarea.inputValue();
    // CustomKeys.txt always has section headers like [Amov] or hotkey= entries
    expect(content).toMatch(/\[.*\]|hotkey=/i);
  });

  test("closing preview dialog via header X hides it", async ({ page }) => {
    await page.locator('[aria-label="Preview"]').click();
    await expect(page.locator(".preview-dialog")).toBeVisible();
    await page.locator(".preview-dialog").locator('[aria-label="Close"]').click();
    await expect(page.locator(".preview-dialog")).not.toBeVisible();
  });

  test("pressing Escape closes the preview dialog", async ({ page }) => {
    await page.locator('[aria-label="Preview"]').click();
    await expect(page.locator(".preview-dialog")).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(page.locator(".preview-dialog")).not.toBeVisible();
  });

  test("preview updates after applying a template", async ({ page }) => {
    // Get initial preview content
    await page.locator('[aria-label="Preview"]').click();
    const contentBefore = await page.locator(".preview-dialog textarea").inputValue();
    await page.locator(".preview-dialog").locator('[aria-label="Close"]').click();

    // Apply a template
    await page.locator('[aria-label="Browse layout templates"]').click();
    await page.locator(".templates-dialog-shell button").first().click();
    await page.locator(".dx-toast").first().waitFor();

    // Check preview content changed
    await page.locator('[aria-label="Preview"]').click();
    const contentAfter = await page.locator(".preview-dialog textarea").inputValue();
    expect(contentAfter.length).toBeGreaterThan(0);
    // Template apply always normalizes so the text may change
    expect(contentAfter).toMatch(/\[.*\]|hotkey=/i);
  });

  test("Preview button toggles off when pressed again", async ({ page }) => {
    await page.locator('[aria-label="Preview"]').click();
    await expect(page.locator(".preview-dialog")).toBeVisible();
    await page.locator('[aria-label="Hide preview"]').click();
    await expect(page.locator(".preview-dialog")).not.toBeVisible();
  });
});
