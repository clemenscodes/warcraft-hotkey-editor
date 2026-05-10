import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

// Minimal valid CustomKeys.txt — two ability bindings
const SAMPLE_CUSTOM_KEYS = [
  "[Amov]",
  "hotkey=A",
  "",
  "[Aatk]",
  "hotkey=Q",
  "",
].join("\r\n");

test.describe("File import", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.waitForSelector(".unit-card");
  });

  test("Upload button is visible", async ({ page }) => {
    await expect(page.locator('[aria-label="Upload CustomKeys.txt"]')).toBeVisible();
  });

  test("clicking Upload opens the upload info dialog", async ({ page }) => {
    await page.locator('[aria-label="Upload CustomKeys.txt"]').click();
    await expect(page.locator(".upload-info-dialog")).toBeVisible();
  });

  test("upload info dialog Cancel closes it", async ({ page }) => {
    await page.locator('[aria-label="Upload CustomKeys.txt"]').click();
    await page.locator(".upload-info-dialog button", { hasText: "Cancel" }).click();
    await expect(page.locator(".upload-info-dialog")).not.toBeVisible();
  });

  test("setting a file on the hidden input imports it and shows a toast", async ({ page }) => {
    await page.locator("#upload-customkeys-input").setInputFiles({
      name: "CustomKeys.txt",
      mimeType: "text/plain",
      buffer: Buffer.from(SAMPLE_CUSTOM_KEYS),
    });
    await expect(page.locator(".dx-toast").first()).toBeVisible();
    await expect(page.locator(".dx-toast").first()).toContainText(/import/i);
  });

  test("import stores normalized content in localStorage", async ({ page }) => {
    await page.locator("#upload-customkeys-input").setInputFiles({
      name: "CustomKeys.txt",
      mimeType: "text/plain",
      buffer: Buffer.from(SAMPLE_CUSTOM_KEYS),
    });
    await page.locator(".dx-toast").first().waitFor();
    const stored = await page.evaluate(() =>
      localStorage.getItem("warcraft-hotkey-editor.custom-keys")
    );
    expect(stored).not.toBeNull();
    // The normalized file should still contain Amov and Aatk sections
    expect(stored).toContain("[Amov]");
  });

  test("Choose File in upload dialog triggers file chooser", async ({ page }) => {
    await page.locator('[aria-label="Upload CustomKeys.txt"]').click();
    const [fileChooser] = await Promise.all([
      page.waitForEvent("filechooser"),
      page.locator(".upload-info-dialog button", { hasText: "Choose File" }).click(),
    ]);
    await fileChooser.setFiles({
      name: "CustomKeys.txt",
      mimeType: "text/plain",
      buffer: Buffer.from(SAMPLE_CUSTOM_KEYS),
    });
    await expect(page.locator(".dx-toast").first()).toBeVisible();
  });

  test("imported file updates the preview", async ({ page }) => {
    await page.locator("#upload-customkeys-input").setInputFiles({
      name: "CustomKeys.txt",
      mimeType: "text/plain",
      buffer: Buffer.from(SAMPLE_CUSTOM_KEYS),
    });
    await page.locator(".dx-toast").first().waitFor();
    await page.locator('[aria-label="Preview"]').click();
    const content = await page.locator(".preview-dialog textarea").inputValue();
    expect(content).toContain("[Amov]");
  });
});
