import { expect, test, type Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

// Deep-linking on the collision pages: the selected list entry rides in the
// `?entry=` URL param so it survives leaving the page (clicking a unit → editor)
// and is restorable via browser back, per-tab, and from a pasted/bookmarked URL
// (scrolled into view). As with the other collision suites the Default template
// is applied first so the entry set is the canonical collision-heavy baseline.

async function applyDefaultTemplate(page: Page): Promise<void> {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();
  await page.locator('[aria-label="Browse layout templates"]').click();
  await page
    .locator(".templates-dialog-shell .wc3-dialog-body button", { hasText: "Default" })
    .click();
  await page.locator('[role="alertdialog"]').first().waitFor();
}

async function openCollisionKind(
  page: Page,
  kind: string,
  rowAttribute: string,
): Promise<void> {
  await page.goto(`${APP}?view=collisions&kind=${kind}`);
  await page.locator(`[data-collision-kind="${kind}"]`).waitFor();
  await page.locator(`[${rowAttribute}]`).first().waitFor();
}

function rowKeys(page: Page, rowAttribute: string): Promise<string[]> {
  return page
    .locator(`[${rowAttribute}]`)
    .evaluateAll(
      (elements, attribute) =>
        elements.map((element) => element.getAttribute(attribute) ?? ""),
      rowAttribute,
    );
}

test.describe("Collision page entry deep-linking", () => {
  // Picking an entry writes it to the URL (via replace, so it does not spam
  // history); opening a unit pushes the editor on top; browser back lands on
  // the collision page with the same entry still selected.
  test("selecting an entry deep-links it and back-from-editor restores it", async ({
    page,
  }) => {
    await applyDefaultTemplate(page);
    await openCollisionKind(page, "unit-positions", "data-unit-position-key");

    const keys = await rowKeys(page, "data-unit-position-key");
    expect(keys.length).toBeGreaterThan(3);
    const target = keys[3];

    const targetRow = page.locator(`[data-unit-position-key="${target}"]`);
    await targetRow.click();
    await expect(page).toHaveURL(new RegExp(`entry=${target}(&|$)`));
    await expect(targetRow).toHaveClass(/selected/);

    // Open the affected unit in the editor.
    await page.locator(".hotkey-detail-unit").click();
    await expect(page).not.toHaveURL(/view=collisions/);
    await expect(page.locator(".grid-tile.has-ability").first()).toBeVisible();

    // Browser back restores the collisions view on the very same entry.
    await page.goBack();
    await expect(page).toHaveURL(
      new RegExp(`kind=unit-positions&entry=${target}`),
    );
    await expect(
      page.locator(`[data-unit-position-key="${target}"]`),
    ).toHaveClass(/selected/);
  });

  // Each kind keeps its own last selection in memory; switching tabs and back
  // returns to where you were, and the URL carries the active kind's entry.
  test("each collision tab remembers its own selected entry", async ({ page }) => {
    await applyDefaultTemplate(page);
    await openCollisionKind(page, "unit-positions", "data-unit-position-key");

    const unitPositionKeys = await rowKeys(page, "data-unit-position-key");
    expect(unitPositionKeys.length).toBeGreaterThan(3);
    const unitPositionTarget = unitPositionKeys[3];
    await page.locator(`[data-unit-position-key="${unitPositionTarget}"]`).click();
    await expect(
      page.locator(`[data-unit-position-key="${unitPositionTarget}"]`),
    ).toHaveClass(/selected/);

    // Switch to hotkeys and pick a different entry there.
    await page.locator('[data-breadcrumb="hotkeys"]').click();
    await page.locator("[data-hotkey-unit-key]").first().waitFor();
    const hotkeyKeys = await rowKeys(page, "data-hotkey-unit-key");
    expect(hotkeyKeys.length).toBeGreaterThan(3);
    const hotkeyTarget = hotkeyKeys[3];
    await page.locator(`[data-hotkey-unit-key="${hotkeyTarget}"]`).click();
    await expect(
      page.locator(`[data-hotkey-unit-key="${hotkeyTarget}"]`),
    ).toHaveClass(/selected/);

    // Back to unit-positions: the earlier selection is still there.
    await page.locator('[data-breadcrumb="unit-positions"]').click();
    await expect(
      page.locator(`[data-unit-position-key="${unitPositionTarget}"]`),
    ).toHaveClass(/selected/);
    await expect(page).toHaveURL(
      new RegExp(`kind=unit-positions&entry=${unitPositionTarget}`),
    );

    // And hotkeys still remembers its own.
    await page.locator('[data-breadcrumb="hotkeys"]').click();
    await expect(
      page.locator(`[data-hotkey-unit-key="${hotkeyTarget}"]`),
    ).toHaveClass(/selected/);
  });

  // A pasted/bookmarked deep-link to an entry far down the list selects it. The
  // list is not auto-scrolled to the entry — selection no longer moves the
  // viewport (the scroll-into-view-on-select behaviour was removed because it
  // fired on every click, unlike the editor's unit list).
  test("a deep-linked entry far down the list is selected", async ({
    page,
  }) => {
    await applyDefaultTemplate(page);
    await openCollisionKind(page, "unit-positions", "data-unit-position-key");

    const keys = await rowKeys(page, "data-unit-position-key");
    expect(keys.length).toBeGreaterThan(20);
    const target = keys[keys.length - 1];

    await page.goto(`${APP}?view=collisions&kind=unit-positions&entry=${target}`);
    const targetRow = page.locator(`[data-unit-position-key="${target}"]`);
    await expect(targetRow).toHaveClass(/selected/);
  });

  // The same round-trip for cross-unit position islands, whose key contains
  // ':' and is URL-encoded — exercises the encode/decode path.
  test("a position island selection survives navigating into a unit and back", async ({
    page,
  }) => {
    await applyDefaultTemplate(page);
    await openCollisionKind(page, "positions", "data-island-key");

    const keys = await rowKeys(page, "data-island-key");
    expect(keys.length).toBeGreaterThan(2);
    const target = keys[2];

    const targetRow = page.locator(`[data-island-key="${target}"]`);
    await targetRow.click();
    await expect(targetRow).toHaveClass(/selected/);
    await expect(page).toHaveURL(/entry=/);

    // Open one of the affected units from the island detail, then back.
    await page.locator(".conflict-unit").first().click();
    await expect(page.locator(".grid-tile.has-ability").first()).toBeVisible();

    await page.goBack();
    await expect(page.locator(`[data-island-key="${target}"]`)).toHaveClass(
      /selected/,
    );
  });
});
