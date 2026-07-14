import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

async function visibleUnitIds(page: any): Promise<string[]> {
  await page.locator(".unit-card").first().waitFor();
  const ids = await page
    .locator(".unit-card code")
    .evaluateAll((els: Element[]) => els.map((el) => el.textContent?.trim() ?? ""));
  return ids.filter((id: string) => id.length > 0);
}

async function browseRace(page: any, race: string) {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();
  await page.locator(`.race-tabs [class*="${race}-race-tab"]`).click();
  await page.locator(`.race-tabs .${race}-race-tab .active-race-tab`).waitFor();
}

test.describe("Variant unit dedup (#27 upgrade-swaps + #28 tiers)", () => {
  test("Carrion Beetle tiers collapse to the strongest (ucs3), hiding ucs1/ucs2", async ({
    page,
  }) => {
    await browseRace(page, "undead");
    const ids = await visibleUnitIds(page);
    expect(ids).toContain("ucs3");
    expect(ids).not.toContain("ucs2");
    expect(ids).not.toContain("ucs1");
    // The burrowed beetle forms collapse the same way: only ucsC, not ucsB.
    expect(ids).toContain("ucsC");
    expect(ids).not.toContain("ucsB");
  });

  test("Orc upgrade-swap and summon tiers list only the strongest unit", async ({ page }) => {
    await browseRace(page, "orc");
    const ids = await visibleUnitIds(page);
    // Headhunter `ohun` upgrades into Berserker `otbk`: only the upgraded unit.
    expect(ids).toContain("otbk");
    expect(ids).not.toContain("ohun");
    // Spiderling summon tiers `osp1..osp4` collapse to the strongest `osp4`.
    expect(ids).toContain("osp4");
    expect(ids).not.toContain("osp1");
    expect(ids).not.toContain("osp2");
    expect(ids).not.toContain("osp3");
  });

  test("Human Siege Engine collapses onto its barrage-upgraded form (hrtt), hiding hmtt", async ({
    page,
  }) => {
    await browseRace(page, "human");
    const ids = await visibleUnitIds(page);
    expect(ids).toContain("hrtt");
    expect(ids).not.toContain("hmtt");
  });

  test("heroes with duplicate ids list once, as the produced hero", async ({ page }) => {
    await browseRace(page, "neutral");
    const ids = await visibleUnitIds(page);
    // Alchemist: only the produced Nalc, not the campaign/form variants.
    expect(ids).toContain("Nalc");
    expect(ids).not.toContain("Nal2");
    expect(ids).not.toContain("Nal3");
    expect(ids).not.toContain("Nalm");
    // Tinker: only Ntin, not the Robo Goblin form.
    expect(ids).toContain("Ntin");
    expect(ids).not.toContain("Nrob");
    // Exactly one "Alchemist" card remains.
    const alchemistCards = page.locator(".unit-card", { hasText: "Alchemist" });
    await expect(alchemistCards).toHaveCount(1);
  });

  test("Clockwerk Goblin variants collapse to a single entry (ncgb), hiding ncg1/ncg2/ncg3", async ({
    page,
  }) => {
    await browseRace(page, "neutral");
    const ids = await visibleUnitIds(page);
    expect(ids).toContain("ncgb");
    expect(ids).not.toContain("ncg1");
    expect(ids).not.toContain("ncg2");
    expect(ids).not.toContain("ncg3");
    // Exactly one "Clockwerk Goblin" card remains in curated browsing.
    const clockwerkCards = page.locator(".unit-card", { hasText: "Clockwerk Goblin" });
    await expect(clockwerkCards).toHaveCount(1);
  });

  test("'All variants' expands the Clockwerk Goblin group to all four ids", async ({ page }) => {
    await browseRace(page, "neutral");
    const allVariants = page.getByRole("button", { name: "All variants" });
    await allVariants.click();
    await expect(allVariants).toHaveAttribute("aria-pressed", "true");
    const ids = await visibleUnitIds(page);
    expect(ids).toContain("ncg1");
    expect(ids).toContain("ncg2");
    expect(ids).toContain("ncg3");
    expect(ids).toContain("ncgb");
    // All four "Clockwerk Goblin" cards list once the group is expanded.
    const clockwerkCards = page.locator(".unit-card", { hasText: "Clockwerk Goblin" });
    await expect(clockwerkCards).toHaveCount(4);
  });

  test("searching a weaker variant surfaces its canonical, never the weaker id", async ({
    page,
  }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('input[type="search"]').fill("osp1");
    await expect(page).toHaveURL(/search_query=osp1/);
    const ids = await visibleUnitIds(page);
    expect(ids).toContain("osp4");
    expect(ids).not.toContain("osp1");
  });
});
