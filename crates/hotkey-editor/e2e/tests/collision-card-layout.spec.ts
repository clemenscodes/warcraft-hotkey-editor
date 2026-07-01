import { expect, test, type Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

// The position-collision detail card shows ONE affected unit (e.g. Blue Dragon
// has both Frost Attack and Devour on the same cell) flanked by the two
// abilities that clash on it. The second "sample carrier" unit that used to
// head the card was misleading and was removed — these tests pin the new shape:
//   * exactly one unit per card, two abilities, one separator,
//   * the ability icon opens the carriers dialog,
//   * the "+N more" link opens the SAME carriers dialog as its ability icon.
// As with the other collision suites, the Default template is applied first so
// the working state is the canonical collision-heavy baseline.

async function openPositionCollisions(page: Page): Promise<void> {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();

  await page.locator('[aria-label="Browse layout templates"]').click();
  await page
    .locator(".template-card", { hasText: "Default" })
    .click();
  await page.locator('[role="alertdialog"]').first().waitFor();

  await page.locator('[data-action="view-collisions"]').click();
  await page.locator('[data-collision-kind="positions"]').waitFor();
  await page.locator(".conflict-card").first().waitFor();
}

async function dialogCarrierNames(page: Page): Promise<string[]> {
  return page.locator(".dialog .carrier-card-name").allTextContents();
}

async function closeCarriersDialog(page: Page): Promise<void> {
  await page.locator('.dialog [aria-label="Close"]').click();
  await page.locator(".dialog").waitFor({ state: "detached" });
}

test.describe("Collision card layout", () => {
  test.beforeEach(async ({ page }) => {
    await openPositionCollisions(page);
  });

  // The core of the redesign: one unit per card (not two), with the two
  // abilities that clash on the cell and a single ✕ separator between them.
  test("every conflict card has exactly one unit, two abilities and one separator", async ({
    page,
  }) => {
    const shapes = await page.locator(".conflict-card").evaluateAll((cards: Element[]) =>
      cards.map((card) => ({
        units: card.querySelectorAll(".conflict-unit").length,
        abilities: card.querySelectorAll(".conflict-ability").length,
        separators: card.querySelectorAll(".conflict-separator").length,
      })),
    );

    expect(shapes.length).toBeGreaterThan(0);
    for (const shape of shapes) {
      expect(shape.units).toBe(1);
      expect(shape.abilities).toBe(2);
      expect(shape.separators).toBe(1);
    }
  });

  // The ability icon is the primary trigger for the carriers dialog.
  test("clicking an ability icon opens the carriers dialog", async ({ page }) => {
    await page.locator(".conflict-ability-trigger").first().click();

    const dialog = page.locator(".dialog");
    await dialog.waitFor();
    await expect(dialog.locator(".carrier-card").first()).toBeVisible();

    await closeCarriersDialog(page);
  });

  // The "+N more" link must open the exact same carriers dialog as the ability
  // icon next to it: same title, same list of carriers.
  test('"+N more" opens the same carriers dialog as its ability icon', async ({
    page,
  }) => {
    const abilityWithMore = page
      .locator(".conflict-ability", { has: page.locator(".conflict-more") })
      .first();
    await expect(
      abilityWithMore,
      "expected at least one ability with a '+N more' link on the first island",
    ).toBeVisible();

    await abilityWithMore.locator(".conflict-ability-trigger").click();
    await page.locator(".dialog").waitFor();
    const titleFromIcon = await page.locator(".dialog h2").textContent();
    const namesFromIcon = await dialogCarrierNames(page);
    await closeCarriersDialog(page);

    await abilityWithMore.locator(".conflict-more").click();
    await page.locator(".dialog").waitFor();
    const titleFromMore = await page.locator(".dialog h2").textContent();
    const namesFromMore = await dialogCarrierNames(page);
    await closeCarriersDialog(page);

    expect(namesFromIcon.length).toBeGreaterThan(0);
    expect(titleFromMore).toBe(titleFromIcon);
    expect(namesFromMore).toEqual(namesFromIcon);
  });
});
