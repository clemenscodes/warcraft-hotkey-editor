import { expect, test, type Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";
const KEYS_STORAGE = "warcraft-hotkey-editor.custom-keys";
const UNDO_STORAGE = "warcraft-hotkey-editor.undo-history";

function storedKeys(page: Page): Promise<string | null> {
  return page.evaluate((key) => localStorage.getItem(key), KEYS_STORAGE);
}

async function applyCascade(page: Page): Promise<string> {
  await page.locator('.resolve-button').first().click();
  await page.locator(".apply-button", { hasText: /apply/i }).click();
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
    const undoButton = page.locator('.undo-button button');
    const redoButton = page.locator('.redo-button button');
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
    await page.locator('.brand').first().click();
    await page.locator(".unit-card").first().waitFor();
    await page.locator(".unit-card").first().click();

    await page.keyboard.press("Control+z");
    await expect.poll(() => storedKeys(page)).toBe(initial);

    await page.keyboard.press("Control+Shift+z");
    await expect.poll(() => storedKeys(page)).toBe(afterAction);
  });

  test("Ctrl+Z inside the search field does not trigger app undo", async ({ page }) => {
    await applyCascade(page);
    await page.locator('.brand').first().click();
    await page.locator(".unit-card").first().waitFor();

    const before = await storedKeys(page);
    const search = page.locator('input[type="search"]');
    await search.click();
    await search.fill("footman");
    await page.keyboard.press("Control+z");
    await expect.poll(() => storedKeys(page)).toBe(before);
  });

  test("undo history is compressed, persisted, and survives a reload", async ({
    page,
  }) => {
    test.setTimeout(30_000);
    const initial = await storedKeys(page);
    const afterAction = await applyCascade(page);
    await page.locator('.brand').first().click();
    await page.locator(".unit-card").first().waitFor();

    await expect
      .poll(() => page.evaluate((key) => localStorage.getItem(key), UNDO_STORAGE), {
        timeout: 4000,
      })
      .not.toBeNull();

    await page.reload();
    await page.locator(".unit-card").first().waitFor();

    const undoButton = page.locator('.undo-button button');
    await expect(undoButton).toBeEnabled();
    await undoButton.click();
    await expect.poll(() => storedKeys(page)).toBe(initial);
    expect(afterAction).not.toBe(initial);
  });

  test("a fresh boot with no action never persists undo history", async ({ page }) => {
    await expect(page.locator('.undo-button button')).toBeDisabled();

    await page.waitForTimeout(2000);

    const undoBlob = await page.evaluate(
      (key) => localStorage.getItem(key),
      UNDO_STORAGE,
    );
    expect(undoBlob).toBeNull();
  });
});
