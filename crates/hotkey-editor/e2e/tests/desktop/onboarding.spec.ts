import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";
const SEEN_KEY = "warcraft-hotkey-editor.onboarding-seen";

test.use({ storageState: { cookies: [], origins: [] } });

test.describe("Onboarding Help dialog", () => {
  test("auto-opens on first visit, and the button dismisses it for good", async ({ page }) => {
    await page.goto(APP);
    const dialog = page.getByRole("dialog");
    await expect(dialog).toBeVisible();

    await dialog.getByRole("button", { name: "Got it, don't show this again" }).click();
    await expect(dialog).toBeHidden();

    const flag = await page.evaluate((key) => localStorage.getItem(key), SEEN_KEY);
    expect(flag).toBe("true");

    await page.reload();
    await page.locator(".unit-card").first().waitFor();
    await expect(page.getByRole("dialog")).toBeHidden();

    await page.getByRole("button", { name: "How to use this editor" }).click();
    await expect(page.getByRole("dialog")).toBeVisible();
  });

  test("closing without the button leaves it to reopen next visit", async ({ page }) => {
    await page.goto(APP);
    const dialog = page.getByRole("dialog");
    await expect(dialog).toBeVisible();

    await dialog.getByRole("button", { name: "Close" }).click();
    await expect(dialog).toBeHidden();

    const flag = await page.evaluate((key) => localStorage.getItem(key), SEEN_KEY);
    expect(flag).toBeNull();

    await page.reload();
    await expect(page.getByRole("dialog")).toBeVisible();
  });
});
