import { expect, test, type Page, type Locator } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

// Apply the Default template and resolve cascade — same dance as
// destroyer-regression.spec.ts. Without this, abilities whose default
// `Buttonpos=` collides with another ability's default on the same unit
// only render once; e.g. Devour Magic and Absorb Mana both default to
// (0,2) on the Destroyer. Post-cascade every ability has a unique cell so
// the command card shows the unit's full ability list.
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
  await page
    .locator(".resolve-info-dialog button", { hasText: "Apply" })
    .click();
  await page
    .locator('[role="alertdialog"]')
    .filter({ hasText: "Cascade applied" })
    .waitFor();
}

// Pick a unit from a given race + search query, then click it. Returns once
// the command grid has finished rendering at least one ability tile.
//
// Does not apply a template — abilities surface at their default
// `Buttonpos=` cells and units with intra-unit collisions render
// fewer than their full ability list. For tests that need every
// ability visible, call `applyTemplateAndCascade` first.
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
  // `data-grid-section`, `data-grid-col`, `data-grid-row`, and the
  // `has-ability` class all live on the same `<div>` (the tile itself) —
  // see `command_grid/grid_cell/mod.rs::class_name`. A descendant selector
  // (`[data-grid-section="Command card"] .has-ability`) would never match,
  // so combine the attribute and the class on a single compound selector.
  return page
    .locator('[data-grid-section="Command card"].has-ability img')
    .evaluateAll((nodes: Element[]) =>
      nodes
        .map((node) => node.getAttribute("alt"))
        .filter((alt): alt is string => Boolean(alt)),
    );
}

function commandCardCell(page: Page, column: number, row: number): Locator {
  return page.locator(
    `[data-grid-section="Command card"][data-grid-col="${column}"][data-grid-row="${row}"]`,
  );
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

  // Burrowed Barbed Arachnathid (nbnb) carries Burrow (Abu5). It ships with
  // `inEditor=1` so it has always been visible — guard against the catalog
  // filter accidentally cutting it.
  test("Burrowed Barbed Arachnathid command card shows Burrow", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "neutral",
      query: "nbnb",
      cardText: "Burrowed Barbed Arachnathid",
    });
    const alts = await commandCardSlotAlts(page);
    expect(alts).toContain("Burrow");
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
// `_balance/*/units/` use a DIFFERENT column layout from the live files —
// `inEditor` is column 10 there, column 9 in the live `unitui.slk`. A
// matcher that read the notused files would corrupt every flag for every
// unit row that appears in both. `is_war3_units_path` excludes them.
test.describe("Stale notused_*.slk files stay excluded", () => {
  // Owl Scout (nowl) has `inEditor=1` in every live `unitui.slk` variant.
  // When the extractor accidentally read `notused_unitui.slk`, its differing
  // column layout collapsed nowl's flag to `inEditor=0`, removed it from
  // the catalog, and stopped the Druid of the Talon ability cascade from
  // resolving its summon target.
  test("Owl Scout (nowl, Sentry Wards summon) is in the catalog as a Neutral unit", async ({
    page,
  }) => {
    await pickUnit(page, {
      race: "neutral",
      query: "nowl",
      cardText: "Owl Scout",
    });
    // Selection successful + at least one ability tile means the unit
    // wasn't filtered out and its abilities resolved correctly.
    await page.locator(".grid-tile.has-ability").first().waitFor();
  });

  // Destroyer (ubsp) carries five abilities in `unitabilities.slk` —
  // Devour Magic (Advm), Orb of Annihilation (Afak), Avenger Form (Aave),
  // Absorb Mana (Aabs), and Spell Immunity (ACmi). When notused_*.slk
  // pollution corrupted the transform-target filter, four of them got
  // stripped and only ACmi survived. Template + cascade is required
  // because Advm and Aabs both default to (0,2). Avenger Form is the
  // transform-back-to-Obsidian-Statue toggle on the Destroyer — it lives
  // at the off-state position (3,2) which Absorb Mana also occupies
  // post-cascade, so only Aabs visibly renders there. We verify the
  // other four; the Rust regression test
  // (`destroyer_carries_full_base_ability_set`) covers the data layer for
  // Aave separately.
  test("Destroyer command card surfaces the non-morph base abilities", async ({
    page,
  }) => {
    await applyTemplateAndCascade(page);
    await pickUnit(page, {
      race: "undead",
      query: "ubsp",
      cardText: "ubsp",
      skipNavigate: true,
    });
    const alts = await commandCardSlotAlts(page);
    expect(alts).toContain("Devour Magic");
    expect(alts).toContain("Orb of Annihilation");
    expect(alts).toContain("Absorb Mana");
    expect(alts).toContain("Spell Immunity");
  });
});

// The catalog filter weeds out placeholder unit rows (no abilities + no
// production) but must NOT drop campaign-only units in their dedicated
// mode — and must NOT show them in Melee mode.
test.describe("Campaign units stay out of Melee mode", () => {
  test("Demigod (Ecen, NE campaign) does not appear in Melee Night Elf browse", async ({
    page,
  }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('.race-tab[data-race="nightelf"]').click();
    await page
      .locator('.race-tab[data-race="nightelf"][data-active="true"]')
      .waitFor();
    await expect(
      page.locator(".unit-card").filter({ hasText: "Demigod" }),
    ).toHaveCount(0);
  });

  test("Naga Myrmidon (campaign neutral) does not appear in Melee Neutral browse", async ({
    page,
  }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('.race-tab[data-race="neutral"]').click();
    await page
      .locator('.race-tab[data-race="neutral"][data-active="true"]')
      .waitFor();
    await expect(
      page.locator(".unit-card").filter({ hasText: /^Naga Myrmidon$/ }),
    ).toHaveCount(0);
  });
});
