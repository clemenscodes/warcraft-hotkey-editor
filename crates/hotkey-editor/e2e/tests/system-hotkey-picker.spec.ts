import { expect, test } from "@playwright/test";
import type { Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

// The system / menu hotkey picker (`SystemKeyPickerDialog`) accepts keyboard
// input too. Like the shared `KeyPicker` it must restore focus on every reopen,
// and it must only honor keys the board actually offers — `KeyCodes::from_event`
// also maps Tab/Backspace/Enter, which the game does not bind and the grid never
// shows.

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

    // Reopen and pick again — regression guard for the lost-focus-on-reopen bug.
    await openFirstSlotPicker(page);
    await page.keyboard.press("h");
    await expect(page.locator(".key-picker-board")).not.toBeVisible();
    await expect(slotKey).toHaveText("H");
  });

  test("pressing a key not on the board (Tab) is ignored", async ({ page }) => {
    await openFirstSlotPicker(page);
    await page.keyboard.press("Tab");
    // The picker stays open and the slot stays in its editing state, proving Tab
    // was not accepted as a hotkey.
    await expect(page.locator(".key-picker-board")).toBeVisible();
    await expect(
      page.locator('.inventory-filled-slot').first().locator(".plain-slot-key"),
    ).toHaveText("…");
  });
});
