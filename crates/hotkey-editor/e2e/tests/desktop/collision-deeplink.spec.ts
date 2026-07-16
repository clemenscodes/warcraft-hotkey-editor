import { expect, test, type Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

async function applyDefaultTemplate(page: Page): Promise<void> {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();
  await page.locator('[aria-label="Browse layout templates"]').click();
  await page
    .locator(".template-card", { hasText: "Default" })
    .click();
  await page.locator('[role="alertdialog"]').first().waitFor();
}

async function openCollisionKind(page: Page, kind: string): Promise<void> {
  await page.goto(`${APP}collisions?kind=${kind}`);
  // Each kind renders its own two-pane content component whose identity class is
  // `<kind>-content` (positions-content, hotkeys-content, unit-positions-content).
  await page.locator(`.${kind}-content`).waitFor();
  await page.locator(".collision-card").first().waitFor();
}

function collisionCards(page: Page) {
  return page.locator(".collision-card");
}

// A breadcrumb tab is identified by its visible label text (its slug is no longer
// exposed as a DOM attribute). Labels are unique and not substrings of one
// another, so `hasText` uniquely and stably picks the tab; the count child does
// not interfere.
function collisionBreadcrumb(page: Page, label: string) {
  return page.locator(".breadcrumbs button", { hasText: label });
}

// The selected entry's stable key is not exposed as an attribute; it is the value
// the app writes to `?entry=` when a card is picked. Read it back from the URL.
async function selectedEntry(page: Page): Promise<string> {
  const entry = new URL(page.url()).searchParams.get("entry");
  expect(entry, "expected an ?entry= param after selecting a card").not.toBeNull();
  return entry as string;
}

test.describe("Collision page entry deep-linking", () => {
  // Picking an entry writes it to the URL (via replace, so it does not spam
  // history); opening a unit pushes the editor on top; browser back lands on
  // the collision page with the same entry still selected.
  test("selecting an entry deep-links it and back-from-editor restores it", async ({
    page,
  }) => {
    await applyDefaultTemplate(page);
    await openCollisionKind(page, "unit-positions");

    const cards = collisionCards(page);
    expect(await cards.count()).toBeGreaterThan(3);
    const targetCard = cards.nth(3);

    await targetCard.click();
    await expect(page).toHaveURL(/entry=/);
    const target = await selectedEntry(page);
    await expect(targetCard.locator(".selected-collision-card-button")).toBeVisible();

    // Open the affected unit in the editor.
    await page.locator(".conflict-detail-unit").click();
    await expect(page).not.toHaveURL(/\/collisions/);
    await expect(page.locator(".filled-tile").first()).toBeVisible();

    // Browser back restores the collisions view on the very same entry.
    await page.goBack();
    const restored = new URL(page.url());
    expect(restored.searchParams.get("kind")).toBe("unit-positions");
    expect(restored.searchParams.get("entry")).toBe(target);
    await expect(
      collisionCards(page).nth(3).locator(".selected-collision-card-button"),
    ).toBeVisible();
  });

  // Each kind keeps its own last selection in memory; switching tabs and back
  // returns to where you were, and the URL carries the active kind's entry.
  test("each collision tab remembers its own selected entry", async ({ page }) => {
    await applyDefaultTemplate(page);
    await openCollisionKind(page, "unit-positions");

    const unitPositionCards = collisionCards(page);
    expect(await unitPositionCards.count()).toBeGreaterThan(3);
    const unitPositionCard = unitPositionCards.nth(3);
    await unitPositionCard.click();
    await expect(page).toHaveURL(/entry=/);
    const unitPositionTarget = await selectedEntry(page);
    await expect(
      unitPositionCard.locator(".selected-collision-card-button"),
    ).toBeVisible();

    // Switch to hotkeys and pick a different entry there.
    await collisionBreadcrumb(page, "Hotkey Collisions").click();
    await collisionCards(page).first().waitFor();
    const hotkeyCards = collisionCards(page);
    expect(await hotkeyCards.count()).toBeGreaterThan(3);
    const hotkeyCard = hotkeyCards.nth(3);
    await hotkeyCard.click();
    await expect(page).toHaveURL(/entry=/);
    await expect(
      hotkeyCard.locator(".selected-collision-card-button"),
    ).toBeVisible();

    // Back to unit-positions: the earlier selection is still there.
    await collisionBreadcrumb(page, "Intra Collisions").click();
    await expect(
      collisionCards(page).nth(3).locator(".selected-collision-card-button"),
    ).toBeVisible();
    const backUrl = new URL(page.url());
    expect(backUrl.searchParams.get("kind")).toBe("unit-positions");
    expect(backUrl.searchParams.get("entry")).toBe(unitPositionTarget);

    // And hotkeys still remembers its own.
    await collisionBreadcrumb(page, "Hotkey Collisions").click();
    await expect(
      collisionCards(page).nth(3).locator(".selected-collision-card-button"),
    ).toBeVisible();
  });

  // A pasted/bookmarked deep-link to an entry far down the list selects it. The
  // list is not auto-scrolled to the entry — selection no longer moves the
  // viewport (the scroll-into-view-on-select behaviour was removed because it
  // fired on every click, unlike the editor's unit list).
  test("a deep-linked entry far down the list is selected", async ({
    page,
  }) => {
    await applyDefaultTemplate(page);
    await openCollisionKind(page, "unit-positions");

    const cards = collisionCards(page);
    const count = await cards.count();
    expect(count).toBeGreaterThan(20);
    const lastCard = cards.nth(count - 1);

    // Discover the far-down entry's key by selecting it (the app writes it to
    // ?entry=), then prove a fresh navigation to that URL re-selects the same
    // far-down card.
    await lastCard.click();
    await expect(page).toHaveURL(/entry=/);
    const target = await selectedEntry(page);

    await page.goto(
      `${APP}collisions?kind=unit-positions&entry=${encodeURIComponent(target)}`,
    );
    await expect(
      collisionCards(page).nth(count - 1).locator(".selected-collision-card-button"),
    ).toBeVisible();
  });

  // The same round-trip for cross-unit position islands, whose key contains
  // ':' and is URL-encoded — exercises the encode/decode path.
  test("a position island selection survives navigating into a unit and back", async ({
    page,
  }) => {
    await applyDefaultTemplate(page);
    await openCollisionKind(page, "positions");

    const cards = collisionCards(page);
    expect(await cards.count()).toBeGreaterThan(2);
    const targetCard = cards.nth(2);

    await targetCard.click();
    await expect(targetCard.locator(".selected-collision-card-button")).toBeVisible();
    await expect(page).toHaveURL(/entry=/);

    // Open one of the affected units from the island detail, then back.
    await page.locator(".island-conflict-unit").first().click();
    await expect(page.locator(".filled-tile").first()).toBeVisible();

    await page.goBack();
    await expect(
      collisionCards(page).nth(2).locator(".selected-collision-card-button"),
    ).toBeVisible();
  });
});
