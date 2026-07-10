import { expect, test } from "@playwright/test";
import type { Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";
const LS_KEY = "warcraft-hotkey-editor.custom-keys";

// The hotkey picker (shared `KeyPicker`, used for both ability hotkeys and the
// global grid layout) accepts keyboard input. Two things must hold: only keys
// the board actually offers are honored, and the focus that the keydown handler
// depends on is restored on every reopen — the picker mounts in a portal whose
// focus the dialog resets a tick after mount, which previously left the keyboard
// dead after the first pick.

async function openBlizzardPicker(page: Page) {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();
  await page.locator('input[type="search"]').fill("Hamg");
  const archmage = page.locator(".unit-card").filter({ hasText: "Hamg" });
  await archmage.waitFor();
  await archmage.click();
  const blizzardTile = page
    .locator('[data-grid-id="Command card"] .filled-tile')
    .filter({ has: page.locator('img[alt="Blizzard"]') });
  await blizzardTile.waitFor();
  await blizzardTile.click();
  await page.locator(".normal-override-key, .special-override-key").waitFor();
  await page.locator(".normal-override-key, .special-override-key").click();
  await page.locator(".key-picker-board").waitFor();
  // The keydown handler only fires once focus lands inside the dialog; wait for
  // it so the test never races the deferred focus.
  await expect(page.locator(".key-picker-board")).toBeFocused();
}

test.describe("Ability hotkey picker keyboard input", () => {
  test("pressing an available letter selects it and closes the picker", async ({ page }) => {
    await openBlizzardPicker(page);
    await page.keyboard.press("e");
    await expect(page.locator(".key-picker-board")).not.toBeVisible();
    await expect(page.locator(".normal-override-key, .special-override-key")).toContainText("E");
    const stored = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
    expect(stored).toContain("=E");
  });

  test("keyboard selection still works after reopening the picker", async ({ page }) => {
    // First pick via keyboard.
    await openBlizzardPicker(page);
    await page.keyboard.press("e");
    await expect(page.locator(".key-picker-board")).not.toBeVisible();
    await expect(page.locator(".normal-override-key, .special-override-key")).toContainText("E");

    // Reopen and pick a different key — this is the regression: focus used to be
    // lost on the second open, so the keypress did nothing.
    await page.locator(".normal-override-key, .special-override-key").click();
    await page.locator(".key-picker-board").waitFor();
    await expect(page.locator(".key-picker-board")).toBeFocused();
    await page.keyboard.press("r");
    await expect(page.locator(".key-picker-board")).not.toBeVisible();
    await expect(page.locator(".normal-override-key, .special-override-key")).toContainText("R");
  });

  test("pressing a conflicting (disabled) key does nothing", async ({ page }) => {
    await openBlizzardPicker(page);
    // 'W' is taken by Summon Water Elemental on the Archmage, so it renders as a
    // disabled conflict cell and must not be selectable from the keyboard.
    await expect(page.locator('.key-picker-board [data-label="W"]')).toBeDisabled();
    await page.keyboard.press("w");
    await expect(page.locator(".key-picker-board")).toBeVisible();
    await expect(page.locator(".normal-override-key, .special-override-key")).toContainText("B");
  });

  test("pressing a key not on the board (a digit) does nothing", async ({ page }) => {
    await openBlizzardPicker(page);
    await page.keyboard.press("1");
    await expect(page.locator(".key-picker-board")).toBeVisible();
    await expect(page.locator(".normal-override-key, .special-override-key")).toContainText("B");
  });
});

test.describe("Global grid layout picker keyboard input", () => {
  async function openGridCellPicker(page: Page, column: number, row: number) {
    await page
      .locator(`[data-layout-col="${column}"][data-layout-row="${row}"]`)
      .click();
    await page.locator(".key-picker-board").waitFor();
    await expect(page.locator(".key-picker-board")).toBeFocused();
  }

  test("keyboard selection assigns the cell and keeps working after reopen", async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('[aria-label="Edit global hotkey layout"]').click();
    await page.locator(".layout-editor-content").waitFor();

    // First grid cell, keyboard pick. 'H' doubles as a spatial-navigation key,
    // so this also guards that the picker — not the navigation handler — gets it.
    await openGridCellPicker(page, 0, 0);
    await page.keyboard.press("h");
    await expect(page.locator(".key-picker-board")).not.toBeVisible();
    await expect(
      page.locator('[data-layout-col="0"][data-layout-row="0"]'),
    ).toHaveText("H");

    // A second cell, another navigation-letter pick — proves focus is restored
    // on reopen and that J/K/L also reach the picker.
    await openGridCellPicker(page, 1, 0);
    await page.keyboard.press("j");
    await expect(page.locator(".key-picker-board")).not.toBeVisible();
    await expect(
      page.locator('[data-layout-col="1"][data-layout-row="0"]'),
    ).toHaveText("J");
  });
});
