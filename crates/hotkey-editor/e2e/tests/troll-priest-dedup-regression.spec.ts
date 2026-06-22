import { expect, test, type Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

// Regression: the CASC data for troll high priests contains two variants of
// Abolish Magic (ACdm in custom_v0/melee_v0, ACd2 in custom_v1/base) and two
// variants of Frost Armor (ACfu / ACf2) that share the same `.code` field.
// Both variants in each pair share the same display name: ACdm/ACd2 are both
// called "Abolish Magic", ACfu/ACf2 are both "Frost Armor".
// Before the fix, `merge_additive` included all four CASC files, so troll
// priests showed the same ability button twice.  Rule 4 in
// `WarcraftDataAggregation::unit_abilities_for_unit` keeps only the LAST
// occurrence per code (competitive-balance variants ACd2/ACf2 from
// custom_v1/base win; alternative-mode variants ACdm/ACfu are dropped).
//
// Cascade must be applied before checking: on `nfsh`, ACd2 and Anh2 both
// default to position (1,2), so ACd2 is moved to (0,2) by the cascade.

async function applyTemplateAndCascade(page: Page): Promise<void> {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();

  await page.locator('[aria-label="Browse layout templates"]').click();
  await page
    .locator(".templates-dialog-shell .wc3-dialog-body button", { hasText: "Default" })
    .click();
  await page.locator('[role="alertdialog"]').first().waitFor();

  await page.locator('[aria-label="Resolve conflicts"]').click();
  await page.locator('[data-action="apply-cascade"]').click();
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

test.describe("Troll High Priest: balance overlay dedup regression", () => {
  test.beforeEach(async ({ page }) => {
    await applyTemplateAndCascade(page);
  });

  // Forest Troll High Priest (nfsh): ACdm (alternative mode) and ACd2
  // (competitive balance) both display as "Abolish Magic".  Before the fix
  // both appeared on the command card; after the fix only ACd2 remains.
  // Cascade is applied before this test to settle cross-unit position
  // conflicts (ACd2 moves from (1,2) to (0,2); Anh2 moves to (1,2)).
  test("Forest Troll High Priest shows Abolish Magic exactly once after cascade", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "neutral",
      query: "nfsh",
      cardText: "Forest Troll High Priest",
      skipNavigate: true,
    });
    const alts = await commandCardSlotAlts(page);
    const abolishCount = alts.filter((alt) => alt === "Abolish Magic").length;
    expect(
      abolishCount,
      `Forest Troll High Priest must show Abolish Magic exactly once (tiles found: ${alts.join(", ")})`,
    ).toBe(1);
  });

  // Ice Troll High Priest (nith): ACdm/ACd2 ("Abolish Magic") AND
  // ACfu/ACf2 ("Frost Armor") each had duplicate entries.  After the fix
  // each ability name appears exactly once.
  test("Ice Troll High Priest shows Abolish Magic and Frost Armor each exactly once after cascade", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "neutral",
      query: "nith",
      cardText: "Ice Troll High Priest",
      skipNavigate: true,
    });
    const alts = await commandCardSlotAlts(page);
    const abolishCount = alts.filter((alt) => alt === "Abolish Magic").length;
    const frostArmorCount = alts.filter((alt) => alt === "Frost Armor").length;
    expect(
      abolishCount,
      `Ice Troll High Priest must show Abolish Magic exactly once (tiles found: ${alts.join(", ")})`,
    ).toBe(1);
    expect(
      frostArmorCount,
      `Ice Troll High Priest must show Frost Armor exactly once (tiles found: ${alts.join(", ")})`,
    ).toBe(1);
  });
});
