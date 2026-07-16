import { expect, test, type Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

async function applyTemplateAndCascade(page: Page): Promise<void> {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();

  await page.locator('[aria-label="Browse layout templates"]').click();
  await page
    .locator(".template-card", { hasText: "Default" })
    .click();
  await page.locator('[role="alertdialog"]').first().waitFor();

  await page.locator('[aria-label="Resolve conflicts"]').click();
  await page.locator(".apply-button", { hasText: /apply/i }).click();
  await page
    .locator('[role="alertdialog"]')
    .filter({ hasText: "Cascade applied" })
    .waitFor();
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();
}

async function pickUnit(
  page: Page,
  options: {
    race: string;
    query: string;
    cardText: string | RegExp;
    skipNavigate?: boolean;
  },
): Promise<void> {
  if (!options.skipNavigate) {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
  }
  await page.locator(`.race-tabs [class*="${options.race}-race-tab"]`).click();
  await page
    .locator(`.race-tabs .${options.race}-race-tab .active-race-tab`)
    .waitFor();
  await page.locator('input[type="search"]').fill(options.query);
  const card = page.locator(".unit-card").filter({ hasText: options.cardText });
  await card.first().waitFor();
  await card.first().click();
  await page.locator(".filled-tile").first().waitFor();
}

function slotImg(
  page: Page,
  section: string,
  col: number,
  row: number,
): import("@playwright/test").Locator {
  return page
    .locator(".grid-editor", {
      has: page.locator(".grid-heading", { hasText: section }),
    })
    .locator(".grid-editor-tile")
    .nth(row * 4 + col)
    .locator(".filled-tile img");
}

test.describe("Balance-patch ability dedup regression", () => {
  test.beforeEach(async ({ page }) => {
    await applyTemplateAndCascade(page);
  });

  // Death Knight carries both AUan and AUa2 ("Animate Dead") at the same
  // default slot.  Rule 5 (last wins) keeps AUa2 and suppresses AUan.
  // Resolved position: col=3, row=2 on the command card.
  test("Death Knight: Animate Dead lands at command card (3,2) after cascade", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "undead",
      query: "Udea",
      cardText: "Death Knight",
      skipNavigate: true,
    });
    await expect(slotImg(page, "Command card", 3, 2)).toHaveAttribute(
      "alt",
      "Animate Dead",
    );
  });

  // Banshee carries both Apos and Aps2 ("Possession") at the same default
  // slot.  Rule 5 (last wins) keeps Aps2 and suppresses Apos.
  // Resolved position: col=2, row=2 on the command card.
  test("Banshee: Possession lands at command card (2,2) after cascade", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "undead",
      query: "uban",
      cardText: "Banshee",
      skipNavigate: true,
    });
    await expect(slotImg(page, "Command card", 2, 2)).toHaveAttribute(
      "alt",
      "Possession",
    );
  });

  // Archer carries both Acoa and Aco2 ("Mount Hippogryph") at the same
  // default slot.  Rule 5 keeps Aco2 and suppresses Acoa.
  // Resolved position: col=0, row=2 on the command card.
  test("Archer: Mount Hippogryph lands at command card (0,2) after cascade", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "nightelf",
      query: "earc",
      cardText: "Archer",
      skipNavigate: true,
    });
    await expect(slotImg(page, "Command card", 0, 2)).toHaveAttribute(
      "alt",
      "Mount Hippogryph",
    );
  });

  // Hippogryph carries both Acoh and Aco3 ("Pick up Archer") at the same
  // default slot.  Rule 5 keeps Aco3 and suppresses Acoh.
  // Resolved position: col=0, row=2 on the command card.
  test("Hippogryph: Pick up Archer lands at command card (0,2) after cascade", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "nightelf",
      query: "ehip",
      cardText: "Hippogryph",
      skipNavigate: true,
    });
    await expect(slotImg(page, "Command card", 0, 2)).toHaveAttribute(
      "alt",
      "Pick up Archer",
    );
  });

  // Fire Lord carries ANic (passive) and ANia (autocast toggle) both named
  // "Incinerate".  split_toggle_passive_positions() splits them: ANia keeps
  // its command-card button_position; ANic keeps its research_button_position.
  // Command card resolved position: col=2, row=2.
  test("Fire Lord: Incinerate toggle (ANia) lands at command card (2,2) after cascade", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "neutral",
      query: "nfir",
      cardText: "Firelord",
      skipNavigate: true,
    });
    await expect(slotImg(page, "Command card", 2, 2)).toHaveAttribute(
      "alt",
      "Incinerate",
    );
  });

  // ANic (passive indicator) must appear in the research menu — not the
  // command card — so players can see which hero ability to level up.
  // Research menu resolved position: col=2, row=0.
  test("Fire Lord: Incinerate passive (ANic) lands at research menu (2,0) after cascade", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "neutral",
      query: "nfir",
      cardText: "Firelord",
      skipNavigate: true,
    });
    await expect(slotImg(page, "Research menu", 2, 0)).toHaveAttribute(
      "alt",
      "Incinerate",
    );
  });
});
