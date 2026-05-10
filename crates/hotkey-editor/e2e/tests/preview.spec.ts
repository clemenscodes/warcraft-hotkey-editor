import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

test("clicking Preview opens a dialog whose textarea contains CustomKeys.txt format content", async ({ page }) => {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();

  await page.locator('[aria-label="Preview"]').click();
  await page.locator(".preview-dialog").waitFor();

  const content = await page.locator(".preview-dialog textarea").inputValue();
  expect(content).toMatch(/\[.*\]|hotkey=/i);
});
