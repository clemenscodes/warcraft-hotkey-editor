import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";
const LS_KEY = "warcraft-hotkey-editor.custom-keys";

test("applying a template shows a toast and writes hotkey content to localStorage", async ({ page }) => {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();

  await page.locator('[aria-label="Browse layout templates"]').click();
  await page.locator(".templates-dialog-shell .wc3-dialog-body button").first().click();

  await page.locator('[role="alertdialog"]').first().waitFor();

  const stored = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
  expect(stored).not.toBeNull();
  expect(stored).toMatch(/\[.*\]|hotkey=/i);
});
