import { expect, test, type Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

// Regression: several Warcraft III units carry both the pre-patch and
// post-patch version of an ability because CASC additive merging keeps
// both IDs.  Rule 5 in `WarcraftDataAggregation::suppress_same_slot_duplicates`
// detects ability pairs that share a default button position AND the same
// display name and suppresses the older one.
//
// Affected units and their patched pairs (both IDs default to the same slot):
//   Death Knight (Udea/Uear): AUan + AUa2 ("Animate Dead")
//   Banshee (uban):           Apos + Aps2 ("Possession")
//   Archer (earc):            Acoa + Aco2 ("Mount Hippogryph")
//   Hippogryph (ehip):        Acoh + Aco3 ("Pick up Archer")
//   Fire Lord (nfir):         ANia (toggle passive) + ANic ("Incinerate")
//
// Template + cascade is applied first so that both abilities in each pair
// are placed on visible, non-colliding cells.  Without the Rule 5 fix the
// ability would appear twice; with the fix it appears exactly once.

async function applyTemplateAndCascade(page: Page): Promise<void> {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();

  await page.locator('[aria-label="Browse layout templates"]').click();
  await page
    .locator(".templates-dialog-shell .wc3-dialog-body button", { hasText: "Default" })
    .click();
  await page.locator('[role="alertdialog"]').first().waitFor();

  await page.locator('[aria-label="Resolve conflicts"]').click();
  await page.locator(".resolve-info-dialog").waitFor();
  await page.locator(".resolve-info-dialog button", { hasText: "Apply" }).click();
  await page
    .locator('[role="alertdialog"]')
    .filter({ hasText: "Cascade applied" })
    .waitFor();
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
  await page.locator(`.race-tab[data-race="${options.race}"]`).click();
  await page
    .locator(`.race-tab[data-race="${options.race}"][data-active="true"]`)
    .waitFor();
  await page.locator('input[type="search"]').fill(options.query);
  const card = page.locator(".unit-card").filter({ hasText: options.cardText });
  await card.first().waitFor();
  await card.first().click();
  await page.locator(".grid-tile.has-ability").first().waitFor();
}

function commandCardSlotAlts(page: Page): Promise<string[]> {
  return page
    .locator('[data-grid-section="Command card"].has-ability img')
    .evaluateAll((nodes: Element[]) =>
      nodes
        .map((node) => node.getAttribute("alt"))
        .filter((alt): alt is string => Boolean(alt)),
    );
}

test.describe("Balance-patch ability dedup regression", () => {
  test.beforeEach(async ({ page }) => {
    await applyTemplateAndCascade(page);
  });

  // Death Knight carries both AUan and AUa2 ("Animate Dead") at the same
  // default slot.  After template+cascade both would be visible without the
  // fix.  Rule 5 (last wins) must keep AUa2 and suppress AUan.
  test("Death Knight shows Animate Dead exactly once after cascade", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "undead",
      query: "Udea",
      cardText: "Death Knight",
      skipNavigate: true,
    });
    const alts = await commandCardSlotAlts(page);
    const count = alts.filter((alt) => alt === "Animate Dead").length;
    expect(
      count,
      `Death Knight must show Animate Dead exactly once (tiles: ${alts.join(", ")})`,
    ).toBe(1);
  });

  // Banshee carries both Apos and Aps2 ("Possession") at the same default
  // slot.  Rule 5 (last wins) must keep Aps2 and suppress Apos.
  test("Banshee shows Possession exactly once after cascade", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "undead",
      query: "uban",
      cardText: "Banshee",
      skipNavigate: true,
    });
    const alts = await commandCardSlotAlts(page);
    const count = alts.filter((alt) => alt === "Possession").length;
    expect(
      count,
      `Banshee must show Possession exactly once (tiles: ${alts.join(", ")})`,
    ).toBe(1);
  });

  // Archer carries both Acoa (self-ref) and Aco2 (alias of Acoi) sharing the
  // same default slot and display name "Mount Hippogryph".  Rule 5 keeps the
  // last occurrence (Aco2, from the balance overlay) and suppresses Acoa.
  test("Archer shows Mount Hippogryph exactly once after cascade", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "nightelf",
      query: "earc",
      cardText: "Archer",
      skipNavigate: true,
    });
    const alts = await commandCardSlotAlts(page);
    const count = alts.filter((alt) => alt === "Mount Hippogryph").length;
    expect(
      count,
      `Archer must show Mount Hippogryph exactly once (tiles: ${alts.join(", ")})`,
    ).toBe(1);
  });

  // Hippogryph carries both Acoh (self-ref) and Aco3 (alias of Acoi) sharing
  // the same default slot and display name "Pick up Archer".  Rule 5 keeps the
  // last occurrence (Aco3, from the balance overlay) and suppresses Acoh.
  test("Hippogryph shows Pick up Archer exactly once after cascade", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "nightelf",
      query: "ehip",
      cardText: "Hippogryph",
      skipNavigate: true,
    });
    const alts = await commandCardSlotAlts(page);
    const count = alts.filter((alt) => alt === "Pick up Archer").length;
    expect(
      count,
      `Hippogryph must show Pick up Archer exactly once (tiles: ${alts.join(", ")})`,
    ).toBe(1);
  });

  // Fire Lord carries ANia (toggle passive) and ANic (auto-passive) both named
  // "Incinerate" at the same default slot.  ANia appears LAST in the merged
  // list so "last wins" alone would keep the wrong one.  Rule 5's off-state
  // tiebreaker suppresses the toggle passive (ANia) and retains ANic.
  test("Fire Lord shows Incinerate exactly once after cascade", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "neutral",
      query: "nfir",
      cardText: "Firelord",
      skipNavigate: true,
    });
    const alts = await commandCardSlotAlts(page);
    const count = alts.filter((alt) => alt === "Incinerate").length;
    expect(
      count,
      `Fire Lord must show Incinerate exactly once (tiles: ${alts.join(", ")})`,
    ).toBe(1);
  });
});
