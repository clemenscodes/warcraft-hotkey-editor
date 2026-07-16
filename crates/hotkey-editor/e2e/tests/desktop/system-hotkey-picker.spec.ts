import { expect, test } from "@playwright/test";
import type { Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

async function openFirstSlotPicker(page: Page) {
  await page.locator('.inventory-filled-slot').first().click();
  await page.locator(".key-picker-board").waitFor();
  await expect(page.locator(".key-picker-board")).toBeFocused();
}

test.describe("System hotkey picker keyboard input", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('[aria-label="General hotkeys"]').click();
    await page.locator('.inventory-filled-slot').first().waitFor();
  });

  test("pressing a board key assigns it to the slot", async ({ page }) => {
    const slotKey = page.locator('.inventory-filled-slot').first().locator(".plain-slot-key");
    await openFirstSlotPicker(page);
    await page.keyboard.press("g");
    await expect(page.locator(".key-picker-board")).not.toBeVisible();
    await expect(slotKey).toHaveText("G");
  });

  test("keyboard selection still works after reopening the picker", async ({ page }) => {
    const slotKey = page.locator('.inventory-filled-slot').first().locator(".plain-slot-key");
    await openFirstSlotPicker(page);
    await page.keyboard.press("g");
    await expect(slotKey).toHaveText("G");

    await openFirstSlotPicker(page);
    await page.keyboard.press("h");
    await expect(page.locator(".key-picker-board")).not.toBeVisible();
    await expect(slotKey).toHaveText("H");
  });

  test("pressing a key not on the board (Tab) is ignored", async ({ page }) => {
    await openFirstSlotPicker(page);
    await page.keyboard.press("Tab");
    await expect(page.locator(".key-picker-board")).toBeVisible();
    await expect(
      page.locator('.inventory-filled-slot').first().locator(".plain-slot-key"),
    ).toHaveText("…");
  });
});
