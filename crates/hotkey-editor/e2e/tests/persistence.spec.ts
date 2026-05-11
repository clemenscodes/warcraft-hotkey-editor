import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";
const LS_KEY = "warcraft-hotkey-editor.custom-keys";

test("localStorage is populated before and after a page reload", async ({ page }) => {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();

  const before = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
  expect(before).toMatch(/\[.*\]|hotkey=/i);

  await page.reload();
  await page.locator(".unit-card").first().waitFor();

  const after = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
  expect(after).toMatch(/\[.*\]|hotkey=/i);
});

test("after switching to Orc race the URL contains 'orc'", async ({ page }) => {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();

  await page.locator('.race-tab[data-race="orc"]').click();
  await page.waitForURL(/orc/);
});
