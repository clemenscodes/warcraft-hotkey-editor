import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";
const LS_KEY = "warcraft-hotkey-editor.custom-keys";

const SAMPLE_CUSTOM_KEYS = ["[Amov]", "hotkey=A", "", "[Aatk]", "hotkey=Q", ""].join("\r\n");

test("importing a file shows a toast and stores the content in localStorage", async ({ page }) => {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();

  await page.locator("#upload-customkeys-input").setInputFiles({
    name: "CustomKeys.txt",
    mimeType: "text/plain",
    buffer: Buffer.from(SAMPLE_CUSTOM_KEYS),
  });

  await page.locator('[role="alertdialog"]').first().waitFor();

  const stored = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
  expect(stored).toMatch(/\[.*\]|hotkey=/i);
});
