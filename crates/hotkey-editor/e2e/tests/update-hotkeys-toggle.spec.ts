import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";
const PREF_KEY = "warcraft-hotkey-editor.update-hotkeys-on-move";

test.describe("Update-hotkeys-on-move toggle", () => {
  test("defaults to checked and persists when unchecked", async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();

    await page.locator('[aria-label="Edit global hotkey layout"]').click();
    await page.locator(".layout-editor-content").waitFor();

    const toggle = page.locator('input[aria-label="Update hotkeys when moving abilities"]');
    await expect(toggle).toBeChecked();

    await toggle.uncheck();
    const storedAfter = await page.evaluate((key) => localStorage.getItem(key), PREF_KEY);
    expect(storedAfter).toBe("false");

    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('[aria-label="Edit global hotkey layout"]').click();
    await page.locator(".layout-editor-content").waitFor();
    await expect(
      page.locator('input[aria-label="Update hotkeys when moving abilities"]'),
    ).not.toBeChecked();
  });

  test("with the toggle off, moving an ability keeps its hotkey", async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator(".unit-card").first().click();
    await page.locator(".filled-tile").first().waitFor();

    // Give the first ability a known manual hotkey (Q).
    await page.locator(".filled-tile").first().click();
    await page.locator(".normal-override-key, .special-override-key").waitFor();
    await page.locator(".normal-override-key, .special-override-key").click();
    await page.locator(".key-picker-board").waitFor();
    await page.locator('.key-picker-board').getByRole('button', { name: /^Q/ }).click();
    await expect(page.locator(".normal-override-key, .special-override-key")).toContainText("Q");

    // Turn the toggle off.
    await page.locator('[aria-label="Edit global hotkey layout"]').click();
    await page.locator(".layout-editor-content").waitFor();
    await page.locator('input[aria-label="Update hotkeys when moving abilities"]').uncheck();
    await page.locator(".layout-editor [aria-label='Close']").click().catch(() => {});
    await page.keyboard.press("Escape");
    await expect(page.locator(".layout-editor-content")).toHaveCount(0);

    // Move the ability to a different cell.
    const tiles = page.locator(".filled-tile");
    if ((await tiles.count()) < 2) {
      test.skip();
      return;
    }

    // Capture the target cell's grid coordinates before the drag so we can
    // locate it by identity after the swap (both tiles have abilities, so
    // .first() after the swap is ambiguous).
    const targetSection = await tiles
      .nth(1)
      .evaluate(
        (el) =>
          el.closest(".grid-editor")?.querySelector(".grid-heading")?.textContent?.trim() ?? "",
      );
    const targetIndex = await tiles.nth(1).evaluate((el) => {
      const host = el.closest(".grid-editor-tile");
      let index = 0;
      let sibling = host ? host.previousElementSibling : null;
      while (sibling) {
        index += 1;
        sibling = sibling.previousElementSibling;
      }
      return index;
    });

    await tiles.first().dragTo(tiles.nth(1));

    // After the swap the Q-ability is at the TARGET cell's original position.
    const movedAbilityCell = page
      .locator(".grid-editor", {
        has: page.locator(".grid-heading", { hasText: targetSection }),
      })
      .locator(".grid-editor-tile")
      .nth(targetIndex);
    await movedAbilityCell.click();
    await page.locator(".normal-override-key, .special-override-key").waitFor();
    // The moved ability still shows Q (hotkey not snapped to the new cell).
    await expect(page.locator(".normal-override-key, .special-override-key")).toContainText("Q");
  });
});
