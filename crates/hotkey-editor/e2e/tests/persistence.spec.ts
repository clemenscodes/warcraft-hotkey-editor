import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";
const LS_KEY = "warcraft-hotkey-editor.custom-keys";

test.describe("localStorage persistence", () => {
  test("app writes to localStorage on boot", async ({ page }) => {
    await page.goto(APP);
    await page.waitForSelector(".unit-card");
    const stored = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
    expect(stored).not.toBeNull();
    expect(stored!.length).toBeGreaterThan(0);
  });

  test("state persists across page reload after template apply", async ({ page }) => {
    await page.goto(APP);
    await page.waitForSelector(".unit-card");

    // Apply a template
    await page.locator('[aria-label="Browse layout templates"]').click();
    await page.locator(".templates-dialog-shell button").first().click();
    await page.locator(".dx-toast").first().waitFor();

    const storedBefore = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);

    // Reload and check localStorage is still the same
    await page.reload();
    await page.waitForSelector(".unit-card");

    const storedAfter = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
    expect(storedAfter).toBe(storedBefore);
  });

  test("state persists across page reload after file import", async ({ page }) => {
    await page.goto(APP);
    await page.waitForSelector(".unit-card");

    const sampleKeys = "[Amov]\r\nhotkey=Z\r\n";
    await page.locator("#upload-customkeys-input").setInputFiles({
      name: "CustomKeys.txt",
      mimeType: "text/plain",
      buffer: Buffer.from(sampleKeys),
    });
    await page.locator(".dx-toast").first().waitFor();

    const storedBefore = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);

    await page.reload();
    await page.waitForSelector(".unit-card");

    const storedAfter = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
    expect(storedAfter).toBe(storedBefore);
    expect(storedAfter).toContain("[Amov]");
  });

  test("URL encodes race and unit selection, which is restored on reload", async ({ page }) => {
    await page.goto(APP);
    await page.waitForSelector(".unit-card");

    // Switch to Orc
    await page.locator('.race-tab[data-race="orc"]').click();
    await page.waitForSelector(".unit-card.selected");

    const urlAfterSwitch = page.url();
    expect(urlAfterSwitch).toContain("orc");

    // Reload and verify Orc tab is still active
    await page.reload();
    await page.waitForSelector(".unit-card");
    await expect(page.locator('.race-tab[data-race="orc"]')).toHaveAttribute("data-active", "true");
  });

  test("search query is encoded in the URL", async ({ page }) => {
    await page.goto(APP);
    await page.waitForSelector(".unit-card");

    await page.locator('input[type="search"]').fill("Footman");
    await page.waitForTimeout(200); // allow URL effect to flush

    const url = page.url();
    expect(url).toContain("Footman");
  });
});
