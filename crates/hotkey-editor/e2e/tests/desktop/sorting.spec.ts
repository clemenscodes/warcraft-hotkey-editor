import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

const EXPECTED_KIND_ORDER = ["hero", "building", "worker", "soldier"];

function assertKindOrder(kinds: (string | null)[]) {
  const present = EXPECTED_KIND_ORDER.filter((k) => kinds.includes(k));
  for (let i = 0; i < present.length - 1; i++) {
    expect(kinds.indexOf(present[i])).toBeLessThan(kinds.indexOf(present[i + 1]));
  }
}

const LABEL_TO_KIND: Record<string, string> = {
  Heroes: "hero",
  Buildings: "building",
  Workers: "worker",
  Units: "soldier",
};

async function headingKinds(page: any) {
  const headings = page.locator(".unit-category-heading");
  await headings.first().waitFor();
  const texts: string[] = await headings.evaluateAll((els: Element[]) =>
    els.map((el) => el.textContent ?? ""),
  );
  return texts.map((text) => {
    const match = Object.keys(LABEL_TO_KIND).find((label) => text.includes(label));
    return match ? LABEL_TO_KIND[match] : null;
  });
}

test.describe("Category sort order (#26)", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
  });

  test("Buildings section appears before Units section when browsing Human", async ({ page }) => {
    const kinds = await headingKinds(page);
    const buildingIdx = kinds.indexOf("building");
    const soldierIdx = kinds.indexOf("soldier");
    expect(buildingIdx).toBeGreaterThanOrEqual(0);
    expect(soldierIdx).toBeGreaterThanOrEqual(0);
    expect(buildingIdx).toBeLessThan(soldierIdx);
  });

  test("Buildings section appears before Units section for all races", async ({ page }) => {
    for (const race of ["orc", "nightelf", "undead"]) {
      await page.locator(`.race-tabs [class*="${race}-race-tab"]`).click();
      await page.locator(`.race-tabs .${race}-race-tab .active-race-tab`).waitFor();
      const kinds = await headingKinds(page);
      const buildingIdx = kinds.indexOf("building");
      const soldierIdx = kinds.indexOf("soldier");
      expect(buildingIdx, `${race}: building before soldier`).toBeGreaterThanOrEqual(0);
      expect(soldierIdx, `${race}: soldier present`).toBeGreaterThanOrEqual(0);
      expect(buildingIdx, `${race}: building < soldier`).toBeLessThan(soldierIdx);
    }
  });

  test("full category order is Heroes > Buildings > Workers > Units for all races", async ({ page }) => {
    for (const race of ["human", "orc", "nightelf", "undead"]) {
      if (race !== "human") {
        await page.locator(`.race-tabs [class*="${race}-race-tab"]`).click();
        await page.locator(`.race-tabs .${race}-race-tab .active-race-tab`).waitFor();
      }
      assertKindOrder(await headingKinds(page));
    }
  });

  test("search results category order is Heroes > Buildings > Workers > Units", async ({ page }) => {
    await page.locator('input[type="search"]').fill("elf");
    await expect(page).toHaveURL(/search_query=elf/);
    const kinds = await headingKinds(page);
    expect(kinds.filter(Boolean).length).toBeGreaterThanOrEqual(2);
    assertKindOrder(kinds);
  });
});

test.describe("Passive-only units hidden from unit list (#29)", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
  });

  test("Farm is not visible when browsing Human race", async ({ page }) => {
    await expect(page.locator(".unit-card").filter({ hasText: "Farm" })).toHaveCount(0);
  });

  test("Farm does not appear in search results either", async ({ page }) => {
    await page.locator('input[type="search"]').fill("Farm");
    await expect(page).toHaveURL(/search_query=Farm/);
    await expect(page.locator(".unit-card").filter({ hasText: /^Farm$/ })).toHaveCount(0);
  });

  test("Water Elemental (passive summon) is not visible in search", async ({ page }) => {
    await page.locator('input[type="search"]').fill("Water Elemental");
    await expect(page).toHaveURL(/search_query=Water/);
    await expect(page.locator(".unit-card").filter({ hasText: "Water Elemental" })).toHaveCount(0);
  });

  test("units with real abilities remain visible (Archmage is present for Human)", async ({ page }) => {
    await page.locator(".unit-card").filter({ hasText: "Archmage" }).first().waitFor();
  });

  test("buildings with train buttons remain visible (Barracks is present for Human)", async ({ page }) => {
    await page.locator(".unit-card").filter({ hasText: "Barracks" }).first().waitFor();
  });
});
