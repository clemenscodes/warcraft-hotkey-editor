import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";
const LS_KEY = "warcraft-hotkey-editor.custom-keys";

// Extract the value of a field inside a [section] from a CustomKeys.txt string.
// Section ids are case-insensitive in CustomKeys.txt, and the serializer emits
// them in their canonical case (e.g. [ACdm]), so match without regard to case.
function fieldInSection(content: string, section: string, field: string): string | null {
  const lower = content.toLowerCase();
  const start = lower.indexOf(`[${section.toLowerCase()}]`);
  if (start === -1) return null;
  const end = lower.indexOf("[", start + 1);
  const chunk = end === -1 ? content.slice(start) : content.slice(start, end);
  const match = chunk.match(new RegExp(`${field}=([^\\r\\n]+)`, "i"));
  return match ? match[1].trim() : null;
}

async function applyTemplateAndCascade(page: import("@playwright/test").Page) {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();
  await page.locator('[aria-label="Browse layout templates"]').click();
  await page.locator(".template-card", { hasText: "Default" }).click();
  await page.locator('[role="alertdialog"]').first().waitFor();
  await page.locator('[aria-label="Resolve conflicts"]').click();
  await page.locator(".apply-button", { hasText: /apply/i }).click();
  await page.locator('[role="alertdialog"]').filter({ hasText: "Cascade applied" }).waitFor();
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();
}

test.describe("Drag and drop on command grid", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator(".unit-card").first().click();
    await page.locator(".filled-tile").first().waitFor();
  });

  test("dragging one ability tile onto another swaps them without changing tile count", async ({ page }) => {
    const tiles = page.locator(".filled-tile");
    const countBefore = await tiles.count();
    if (countBefore < 2) {
      test.skip();
      return;
    }
    await tiles.first().dragTo(tiles.nth(1));
    const countAfter = await page.locator(".filled-tile").count();
    expect(countAfter).toBe(countBefore);
  });

  test("swapping a toggle ability (with colocated Unbuttonpos) also swaps Unbuttonpos", async ({
    page,
  }) => {
    // Regression: move_slot required AbilityOff(id) in slot_ids before it would
    // co-move an ability's Unbuttonpos.  Regular command cards only contain
    // Ability(id) slots, so Unbuttonpos was always left behind after a swap.
    //
    // After template + cascade on Kobold Geomancer (nkog):
    //   ACdm stays at (0,2) and ACsw is cascaded to (1,2), both with co-located
    //   Unbuttonpos.  Dragging (0,2) onto (1,2) swaps the pair; both Unbuttonpos
    //   values must follow their respective abilities.
    await applyTemplateAndCascade(page);

    await page.locator('.race-tabs [class*="neutral-race-tab"]').click();
    await page.locator('input[type="search"]').fill("nkog");
    await page.locator(".unit-card").filter({ hasText: "Kobold Geomancer" }).waitFor();
    await page.locator(".unit-card").filter({ hasText: "Kobold Geomancer" }).click();
    await page.locator(".filled-tile").first().waitFor();

    const commandCard = page.locator(".grid-editor", {
      has: page.locator(".grid-heading", { hasText: "Command card" }),
    });
    const sourceCell = commandCard.locator(".grid-editor-tile").nth(8);
    const targetCell = commandCard.locator(".grid-editor-tile").nth(9);

    await expect(sourceCell.locator(".filled-tile")).toHaveCount(1);
    await expect(targetCell.locator(".filled-tile")).toHaveCount(1);

    await sourceCell.dragTo(targetCell);

    const stored = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
    expect(stored).not.toBeNull();

    // ACdm dragged from (0,2) to (1,2) — its Unbuttonpos must follow.
    expect(fieldInSection(stored!, "ACdm", "Unbuttonpos")).toBe("1,2");
    // ACsw displaced from (1,2) to (0,2) — its Unbuttonpos must follow.
    expect(fieldInSection(stored!, "ACsw", "Unbuttonpos")).toBe("0,2");
  });

  test("pressing Escape during a drag cancels it and tile count is unchanged", async ({ page }) => {
    const sourceTile = page.locator(".filled-tile").first();
    const sourceBox = await sourceTile.boundingBox();
    if (!sourceBox) {
      test.skip();
      return;
    }

    const countBefore = await page.locator(".filled-tile").count();
    await page.mouse.move(sourceBox.x + sourceBox.width / 2, sourceBox.y + sourceBox.height / 2);
    await page.mouse.down();
    await page.mouse.move(sourceBox.x + 200, sourceBox.y, { steps: 5 });
    await page.keyboard.press("Escape");
    await page.mouse.up();

    const countAfter = await page.locator(".filled-tile").count();
    expect(countAfter).toBe(countBefore);
  });
});
