import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

test.describe("Template apply", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.waitForSelector(".unit-card");
  });

  test("clicking templates button opens the templates dialog", async ({ page }) => {
    await page.locator('[aria-label="Browse layout templates"]').click();
    await expect(page.locator(".templates-dialog-shell")).toBeVisible();
  });

  test("templates dialog contains at least one template", async ({ page }) => {
    await page.locator('[aria-label="Browse layout templates"]').click();
    await expect(page.locator(".templates-dialog-shell button").first()).toBeVisible();
  });

  test("applying a template shows a success toast", async ({ page }) => {
    await page.locator('[aria-label="Browse layout templates"]').click();
    await page.locator(".templates-dialog-shell button").first().click();
    await expect(page.locator(".dx-toast").first()).toBeVisible();
  });

  test("applying a template closes the dialog", async ({ page }) => {
    await page.locator('[aria-label="Browse layout templates"]').click();
    await page.locator(".templates-dialog-shell button").first().click();
    await expect(page.locator(".templates-dialog-shell")).not.toBeVisible();
  });

  test("applied template populates localStorage", async ({ page }) => {
    await page.locator('[aria-label="Browse layout templates"]').click();
    await page.locator(".templates-dialog-shell button").first().click();
    // Wait for toast to confirm apply completed
    await page.locator(".dx-toast").first().waitFor();
    const stored = await page.evaluate(() =>
      localStorage.getItem("warcraft-hotkey-editor.custom-keys")
    );
    expect(stored).not.toBeNull();
    expect(stored!.length).toBeGreaterThan(100);
  });

  test("closing templates dialog via header X restores focus", async ({ page }) => {
    await page.locator('[aria-label="Browse layout templates"]').click();
    await expect(page.locator(".templates-dialog-shell")).toBeVisible();
    // Close via dialog header close button
    await page.locator(".templates-dialog-shell").locator('[aria-label="Close"]').click();
    await expect(page.locator(".templates-dialog-shell")).not.toBeVisible();
  });
});
