import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

const EXPECTED_KIND_ORDER = ["hero", "building", "worker", "soldier"];

function assertKindOrder(kinds: (string | null)[]) {
  const present = EXPECTED_KIND_ORDER.filter((k) => kinds.includes(k));
  for (let i = 0; i < present.length - 1; i++) {
    expect(kinds.indexOf(present[i])).toBeLessThan(kinds.indexOf(present[i + 1]));
  }
}

test.describe("Category sort order", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
  });

  test("default browsing order is Heroes > Buildings > Workers > Units", async ({ page }) => {
    const headings = page.locator(".unit-category-heading");
    await headings.first().waitFor();
    const kinds = await headings.evaluateAll((els) =>
      els.map((el) => el.getAttribute("data-unit-kind")),
    );
    assertKindOrder(kinds);
  });

  test("switching races preserves Heroes > Buildings > Workers > Units order", async ({ page }) => {
    for (const race of ["orc", "nightelf", "undead"]) {
      await page.locator(`.race-tab[data-race="${race}"]`).click();
      await page.locator(`.race-tab[data-race="${race}"][data-active="true"]`).waitFor();
      const headings = page.locator(".unit-category-heading");
      await headings.first().waitFor();
      const kinds = await headings.evaluateAll((els) =>
        els.map((el) => el.getAttribute("data-unit-kind")),
      );
      assertKindOrder(kinds);
    }
  });

  test("search results category order is Heroes > Buildings > Workers > Units", async ({ page }) => {
    await page.locator('input[type="search"]').fill("elf");
    await page.locator('.unit-list[data-search-active="true"]').waitFor();
    const headings = page.locator(".unit-category-heading");
    await headings.first().waitFor();
    const kinds = await headings.evaluateAll((els) =>
      els.map((el) => el.getAttribute("data-unit-kind")),
    );
    expect(kinds.filter(Boolean).length).toBeGreaterThanOrEqual(2);
    assertKindOrder(kinds);
  });
});
