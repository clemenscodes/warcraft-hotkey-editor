import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

test("clicking Download in the dialog triggers a CustomKeys.txt download with hotkey content", async ({ page }) => {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();

  await page.locator('[aria-label="Download CustomKeys.txt"]').click();

  const [download] = await Promise.all([
    page.waitForEvent("download"),
    page.locator(".download-info-dialog button", { hasText: "Download" }).click(),
  ]);

  expect(download.suggestedFilename()).toBe("CustomKeys.txt");

  const path = await download.path();
  const { readFileSync } = await import("fs");
  const content = readFileSync(path!).toString();
  expect(content.length).toBeGreaterThan(0);
  expect(content).toMatch(/\[.*\]|hotkey=/i);
});
