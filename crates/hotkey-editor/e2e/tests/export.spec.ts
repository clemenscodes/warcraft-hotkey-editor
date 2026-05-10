import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

test.describe("Export / Download flow", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.waitForSelector(".unit-card");
  });

  test("Download button is visible (file always loaded on boot)", async ({ page }) => {
    await expect(page.locator('[aria-label="Download CustomKeys.txt"]')).toBeVisible();
  });

  test("clicking Download opens the download info dialog", async ({ page }) => {
    await page.locator('[aria-label="Download CustomKeys.txt"]').click();
    await expect(page.locator(".download-info-dialog")).toBeVisible();
  });

  test("download info dialog shows the filename", async ({ page }) => {
    await page.locator('[aria-label="Download CustomKeys.txt"]').click();
    await expect(page.locator(".download-info-dialog")).toContainText("CustomKeys.txt");
  });

  test("Cancel button closes the download dialog without downloading", async ({ page }) => {
    await page.locator('[aria-label="Download CustomKeys.txt"]').click();
    await page.locator(".download-info-dialog button", { hasText: "Cancel" }).click();
    await expect(page.locator(".download-info-dialog")).not.toBeVisible();
  });

  test("Download button in dialog triggers a file download", async ({ page }) => {
    await page.locator('[aria-label="Download CustomKeys.txt"]').click();
    const [download] = await Promise.all([
      page.waitForEvent("download"),
      page.locator(".download-info-dialog button", { hasText: "Download" }).click(),
    ]);
    expect(download.suggestedFilename()).toBe("CustomKeys.txt");
  });

  test("downloaded file is not empty", async ({ page }) => {
    await page.locator('[aria-label="Download CustomKeys.txt"]').click();
    const [download] = await Promise.all([
      page.waitForEvent("download"),
      page.locator(".download-info-dialog button", { hasText: "Download" }).click(),
    ]);
    const path = await download.path();
    const { readFileSync } = await import("fs");
    const content = readFileSync(path!).toString();
    expect(content.length).toBeGreaterThan(0);
    expect(content).toMatch(/\[.*\]|hotkey=/i);
  });

  test("closing download dialog via header X works", async ({ page }) => {
    await page.locator('[aria-label="Download CustomKeys.txt"]').click();
    await page.locator(".download-info-dialog").locator('[aria-label="Close"]').click();
    await expect(page.locator(".download-info-dialog")).not.toBeVisible();
  });
});
