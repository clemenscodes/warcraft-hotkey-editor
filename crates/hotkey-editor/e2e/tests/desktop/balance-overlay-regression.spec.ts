import { expect, test, type Page, type Locator } from "@playwright/test";

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

function commandCardSlotAlts(page: Page): Promise<string[]> {
  // Grids carry no positional attributes; a grid is located structurally by its
  // `.grid-heading` text, and each tile is the row-major nth `.grid-editor-tile`
  // inside it. Address the filled tiles as descendants of the identified grid.
  return page
    .locator(".grid-editor", {
      has: page.locator(".grid-heading", { hasText: "Command card" }),
    })
    .locator(".filled-tile img")
    .evaluateAll((nodes: Element[]) =>
      nodes
        .map((node) => node.getAttribute("alt"))
        .filter((alt): alt is string => Boolean(alt)),
    );
}

function commandCardCell(page: Page, column: number, row: number): Locator {
  return page
    .locator(".grid-editor", {
      has: page.locator(".grid-heading", { hasText: "Command card" }),
    })
    .locator(".grid-editor-tile")
    .nth(row * 4 + column);
}

// The balance overlays under `war3.w3mod:_balance/<variant>.w3mod:units/` are
// where Shadow Strike (ACss) appears on Maiden of Pain and where the proper
// uppercase casing of `ACvs` on Earth Borer lives. The extractor reads both
// the base and the overlay `unitabilities.slk` so these abilities land on the
// unit. Regression for the "missing arachnathid units" / "Maiden of Pain has
// no Shadow Strike" reports.
test.describe("Balance overlay regression: undead/neutral abilities", () => {
  // Maiden of Pain (ndqp) carries Life Drain (ACdr) and Shadow Strike (ACss)
  // in the base unitabilities.slk. ACss was missing from the catalog before
  // the case-insensitive union in `UnitAbilitiesEntry::merge_additive` and
  // the balance-dir reading on the SLK matcher.
  test("Maiden of Pain command card shows Shadow Strike and Life Drain", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "neutral",
      query: "ndqp",
      cardText: "Maiden of Pain",
    });
    const alts = await commandCardSlotAlts(page);
    expect(alts).toContain("Shadow Strike");
    expect(alts).toContain("Life Drain");
  });

  // Earth Borer (nane) has `Acvs` in the base file and `ACvs,ACss` in
  // custom_v1. The merge unions them case-insensitively so only one
  // Envenomed Weapons entry shows, and Shadow Strike appears too.
  test("Arachnathid Earth-borer shows Envenomed Weapons and Shadow Strike", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "neutral",
      query: "nane",
      cardText: "Arachnathid Earth-borer",
    });
    const alts = await commandCardSlotAlts(page);
    expect(alts).toContain("Envenomed Weapons");
    expect(alts).toContain("Shadow Strike");
  });

  // Burrowed Barbed Arachnathid (nbnb) is the already-burrowed form, so its
  // toggle shows Unburrow (Abu5's off-state): a burrowed unit can only come
  // back up, it cannot burrow again. It ships with `inEditor=1` and has always
  // been visible, so this also guards against the catalog filter cutting it.
  test("Burrowed Barbed Arachnathid command card shows Unburrow", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "neutral",
      query: "nbnb",
      cardText: "Burrowed Barbed Arachnathid",
    });
    const alts = await commandCardSlotAlts(page);
    expect(alts).toContain("Unburrow");
  });

  // Tavern mercenaries ship with `unitui.slk::inEditor=0` because they're
  // not in the World Editor's unit-picker. `nanm` (Barbed Arachnathid merc)
  // carries Burrow, so the relaxed `passes_filter` in `unit_kind.rs` lets
  // it through. A future tightening of that filter would drop nanm and
  // leave Burrow unbindable on its merc form — this test catches that.
  test("Barbed Arachnathid (merc, inEditor=0) is selectable and has Burrow", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "neutral",
      query: "nanm",
      cardText: /Barbed Arachnathid/i,
    });
    const alts = await commandCardSlotAlts(page);
    expect(alts).toContain("Burrow");
  });
});

// `.txt`-based files in the balance overlays are alternative gameplay
// presets, not strict supersets of the base — `_balance/melee_v0`'s Goblin
// Merchant lists eight different items, none of which should leak into the
// default (custom-balance) command card. These tests guard against the
// regression where the extractor was unioning `.txt` fields across overlays.
test.describe("Base-only `.txt` data — overlay variants must not leak", () => {
  // Goblin Merchant base ships with these 11 sell items in this order:
  // stwp, bspd, dust, tret, prvt, cnob, stel, pnvl, shea, spro, pinv.
  // The melee_v0 overlay redefines the row to phea,pman,pinv,shea,spro,
  // wneg,gemt,stwp. A union of both produced 15 items and pushed three
  // off the 12-cell command card.
  test("Goblin Merchant command card holds the base 11 sell items", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "neutral",
      query: "ngme",
      cardText: "Goblin Merchant",
    });
    const alts = await commandCardSlotAlts(page);
    const required = [
      "Scroll of Town Portal",
      "Boots of Speed",
      "Dust of Appearance",
      "Tome of Retraining",
      "Periapt of Vitality",
      "Circlet of Nobility",
      "Staff of Teleportation",
      "Potion of Lesser Invulnerability",
      "Scroll of Healing",
      "Scroll of Protection",
      "Potion of Invisibility",
    ];
    for (const itemName of required) {
      expect(alts, `missing sell item: ${itemName}`).toContain(itemName);
    }
    // melee_v0-only entries should NOT appear — they belong to a different
    // balance preset entirely.
    const overlay_only = [
      "Potion of Healing",
      "Potion of Mana",
      "Wand of Negation",
      "Gem of True Seeing",
    ];
    for (const item of overlay_only) {
      expect(
        alts,
        `melee_v0-only item leaked into base preset: ${item}`,
      ).not.toContain(item);
    }
  });

  // Town Hall has `Researches=Rhpm` and `Upgrade=hkee` in the base
  // humanunitfunc.txt. The custom_v0 overlay only has `Upgrade=hkee` —
  // when the extractor was reading overlays and doing first-wins merging,
  // it lost `Researches=Rhpm` (Backpack) entirely.
  test("Town Hall command card shows Peasant, Backpack research, and Keep upgrade", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "human",
      query: "htow",
      cardText: "Town Hall",
    });
    const alts = await commandCardSlotAlts(page);
    expect(alts, "Town Hall must train Peasant").toContain("Peasant");
    expect(alts, "Town Hall must offer Backpack research (Rhpm)").toContain(
      "Backpack",
    );
    expect(alts, "Town Hall must offer Keep upgrade (hkee)").toContain("Keep");
  });

  // Orc Barracks (obar) base trains Grunt (ogru) and Headhunter (ohun) at
  // tier 1, plus Berserker (otbk) and Demolisher (ocat) gated behind
  // researches. The command card only surfaces the tier-1 trains plus the
  // four researches (Robs, Rotr, Robk, Robf). Asserting the tier-1 trains
  // plus the Brute Strength / Troll Regeneration researches catches the
  // regression where base unitfunc.txt entries got lost behind balance
  // overlays. `hasText: "obar"` matches the unit_id chip so we don't
  // accidentally select Human Barracks (hbar) whose card also contains
  // the word "Barracks".
  test("Orc Barracks command card publishes the base tier-1 trains and researches", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "orc",
      query: "obar",
      cardText: "obar",
    });
    const alts = await commandCardSlotAlts(page);
    expect(alts).toContain("Grunt");
    expect(alts).toContain("Headhunter");
    expect(alts).toContain("Brute Strength");
    expect(alts).toContain("Troll Regeneration");
  });
});

// The `notused_unitui.slk` / `notused_unitdata.slk` files shipped under
// `_balance
