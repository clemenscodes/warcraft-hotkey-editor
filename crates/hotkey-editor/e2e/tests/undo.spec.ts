import { expect, test, type Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";
const KEYS_STORAGE = "warcraft-hotkey-editor.custom-keys";
const UNDO_STORAGE = "warcraft-hotkey-editor.undo-history";

// Global undo/redo: one timeline of full-state snapshots. Every committed action
// (here, applying the cascade) is one entry; undo/redo restore it via the
// toolbar buttons or Ctrl/Cmd+Z / Ctrl/Cmd+Shift+Z. History is compressed and
// persisted to localStorage, so it survives a reload.

function storedKeys(page: Page): Promise<string | null> {
  return page.evaluate((key) => localStorage.getItem(key), KEYS_STORAGE);
}

// Applies the cascade (a discrete keys-mutating action) and returns the new
// stored keys text.
async function applyCascade(page: Page): Promise<string> {
  await page.locator('[data-action="view-resolve"]').first().click();
  await page.locator('[data-action="apply-cascade"]').click();
  await page
    .locator('[role="alertdialog"]')
    .filter({ hasText: "Cascade applied" })
    .waitFor();
  const text = await storedKeys(page);
  return text ?? "";
}

test.describe("Undo / redo", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
  });

  test("toolbar undo reverts an action and redo re-applies it", async ({ page }) => {
    const undoButton = page.locator('[data-action="undo"]').first();
    const redoButton = page.locator('[data-action="redo"]').first();
    await expect(undoButton).toBeDisabled();

    const initial = await storedKeys(page);
    const afterAction = await applyCascade(page);
    expect(afterAction).not.toBe(initial);
    await expect(undoButton).toBeEnabled();

    await undoButton.click();
    await expect.poll(() => storedKeys(page)).toBe(initial);
    await expect(redoButton).toBeEnabled();

    await redoButton.click();
    await expect.poll(() => storedKeys(page)).toBe(afterAction);
  });

  test("Ctrl+Z undoes and Ctrl+Shift+Z redoes", async ({ page }) => {
    const initial = await storedKeys(page);
    const afterAction = await applyCascade(page);
    await page.locator('[data-action="view-editor"]').first().click();
    await page.locator(".unit-card").first().waitFor();
    // Focus a non-editable element so the shortcut is not suppressed.
    await page.locator(".unit-card").first().click();

    await page.keyboard.press("Control+z");
    await expect.poll(() => storedKeys(page)).toBe(initial);

    await page.keyboard.press("Control+Shift+z");
    await expect.poll(() => storedKeys(page)).toBe(afterAction);
  });

  test("Ctrl+Z inside the search field does not trigger app undo", async ({ page }) => {
    await applyCascade(page);
    await page.locator('[data-action="view-editor"]').first().click();
    await page.locator(".unit-card").first().waitFor();

    const before = await storedKeys(page);
    const search = page.locator('input[type="search"]');
    await search.click();
    await search.fill("footman");
    await page.keyboard.press("Control+z");
    // App state must be unchanged — the keypress is the field's native undo.
    await expect.poll(() => storedKeys(page)).toBe(before);
  });

  test("undo history is compressed, persisted, and survives a reload", async ({
    page,
  }) => {
    // Multi-step: action + persistence debounce + full reload.
    test.setTimeout(30_000);
    const initial = await storedKeys(page);
    const afterAction = await applyCascade(page);
    await page.locator('[data-action="view-editor"]').first().click();
    await page.locator(".unit-card").first().waitFor();

    // Wait out the persistence debounce, then confirm a compressed blob exists.
    await expect
      .poll(() => page.evaluate((key) => localStorage.getItem(key), UNDO_STORAGE), {
        timeout: 4000,
      })
      .not.toBeNull();

    await page.reload();
    await page.locator(".unit-card").first().waitFor();

    const undoButton = page.locator('[data-action="undo"]').first();
    await expect(undoButton).toBeEnabled();
    await undoButton.click();
    await expect.poll(() => storedKeys(page)).toBe(initial);
    expect(afterAction).not.toBe(initial);
  });
});
