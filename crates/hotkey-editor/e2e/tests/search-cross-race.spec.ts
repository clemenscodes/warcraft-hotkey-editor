import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

test.describe("Search ignores mode filter (melee/campaign)", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
  });

  test("searching a campaign-only unit while in Melee mode still returns it", async ({ page }) => {
    // "Demigod" (Ecen) is a Night Elf campaign unit that never appears in Melee browsing.
    await page.locator('input[type="search"]').fill("Demigod");
    await page.locator(".unit-card").filter({ hasText: "Demigod" }).first().waitFor();
  });

  test("clearing search in Melee mode does not show campaign units", async ({ page }) => {
    await page.locator('input[type="search"]').fill("Demigod");
    await page.locator(".unit-card").filter({ hasText: "Demigod" }).first().waitFor();
    await page.locator('input[type="search"]').clear();
    await expect(page.locator(".unit-card").filter({ hasText: "Demigod" })).toHaveCount(0);
  });
});

test.describe("Trailing-space narrows search to word boundary", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
  });

  test("'dark' matches both Dark Ranger and Altar of Darkness", async ({ page }) => {
    await page.locator('input[type="search"]').fill("dark");
    await page.locator('.unit-list[data-search-active="true"]').waitFor();
    await page.locator(".unit-card").filter({ hasText: "Dark Ranger" }).first().waitFor();
    await page.locator(".unit-card").filter({ hasText: "Altar of Darkness" }).first().waitFor();
  });

  test("'dark ' (trailing space) matches Dark Ranger but not Altar of Darkness", async ({ page }) => {
    await page.locator('input[type="search"]').fill("dark ");
    await page.locator('.unit-list[data-search-active="true"]').waitFor();
    await page.locator(".unit-card").filter({ hasText: "Dark Ranger" }).first().waitFor();
    await expect(page.locator(".unit-card").filter({ hasText: "Altar of Darkness" })).toHaveCount(0);
  });
});

test.describe("Fuzzy / subsequence search", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
  });

  test("'ft' finds Footman (two-char subsequence)", async ({ page }) => {
    await page.locator('input[type="search"]').fill("ft");
    await page.locator(".unit-card").filter({ hasText: "Footman" }).first().waitFor();
  });

  test("'ftma' finds Footman (longer subsequence)", async ({ page }) => {
    await page.locator('input[type="search"]').fill("ftma");
    await page.locator(".unit-card").filter({ hasText: "Footman" }).first().waitFor();
  });

  test("'dmhtr' finds Demon Hunter (subsequence across multiple words)", async ({ page }) => {
    await page.locator('input[type="search"]').fill("dmhtr");
    await page.locator(".unit-card").filter({ hasText: "Demon Hunter" }).first().waitFor();
  });
});

test.describe("Search ignores race filter (#23)", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
  });

  test("searching a Human unit while Orc is selected still returns it", async ({ page }) => {
    await page.locator('.race-tab[data-race="orc"]').click();
    await page.locator('.race-tab[data-race="orc"][data-active="true"]').waitFor();
    await page.locator('input[type="search"]').fill("Footman");
    await page.locator(".unit-card").filter({ hasText: "Footman" }).waitFor();
  });

  test("searching an Orc unit while Night Elf is selected still returns it", async ({ page }) => {
    await page.locator('.race-tab[data-race="nightelf"]').click();
    await page.locator('.race-tab[data-race="nightelf"][data-active="true"]').waitFor();
    await page.locator('input[type="search"]').fill("Grunt");
    await page.locator(".unit-card").filter({ hasText: "Grunt" }).first().waitFor();
  });

  test("clearing the query restores the race-filtered list", async ({ page }) => {
    await page.locator('.race-tab[data-race="orc"]').click();
    await page.locator('.race-tab[data-race="orc"][data-active="true"]').waitFor();
    const orcCount = await page.locator(".unit-card").count();

    await page.locator('input[type="search"]').fill("Footman");
    await page.locator(".unit-card").filter({ hasText: "Footman" }).waitFor();

    await page.locator('input[type="search"]').clear();
    await expect(page.locator(".unit-card")).toHaveCount(orcCount);
  });

  test("searching 'footman' does not return units whose name merely contains 'man' as a substring", async ({ page }) => {
    await page.locator('input[type="search"]').fill("footman");
    await page.locator('.unit-list[data-search-active="true"]').waitFor();
    const cards = page.locator(".unit-card");
    await cards.first().waitFor();
    const count = await cards.count();
    for (let i = 0; i < count; i++) {
      const text = await cards.nth(i).textContent();
      expect(text?.toLowerCase()).toContain("footman");
    }
  });

  test("searching 'Footman' while Undead is selected returns results from Human race", async ({ page }) => {
    await page.locator('.race-tab[data-race="undead"]').click();
    await page.locator('.race-tab[data-race="undead"][data-active="true"]').waitFor();
    await page.locator('input[type="search"]').fill("Footman");
    await page.locator(".unit-card").filter({ hasText: "Footman" }).first().waitFor();
    const count = await page.locator(".unit-card").count();
    expect(count).toBeGreaterThanOrEqual(1);
  });
});
