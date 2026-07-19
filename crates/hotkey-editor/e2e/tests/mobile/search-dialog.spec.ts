import { expect, test, type Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

async function openDialog(page: Page): Promise<void> {
  await page.goto(APP);
  await page.locator(".pager-card").first().waitFor();
  await page.locator('button[aria-label="Search units"]').click();
  await page.locator(".search-dialog-body").waitFor();
  await page.locator(".unit-card").first().waitFor();
}

async function openPanel(page: Page): Promise<void> {
  await openDialog(page);
  await page.locator('button[aria-label="Filter"]').click();
  await page.locator(".search-dialog-filters").waitFor();
}

test.describe("Mobile search dialog", () => {
  test("opens with the search field first and the filters folded", async ({
    page,
  }) => {
    await openDialog(page);
    await expect(page.locator(".unit-list-search-input")).toBeVisible();
    await expect(page.locator(".search-dialog-filters")).toHaveCount(0);
  });

  test("the filter icon opens the panel over the results", async ({ page }) => {
    await openPanel(page);
    await expect(page.locator(".find-units-by-group")).toBeVisible();
    await expect(page.locator(".mode-group")).toBeVisible();
    await expect(page.locator(".also-include-switch")).toHaveCount(2);
    await expect(page.locator(".race-scope-trigger")).toBeVisible();
  });

  test("mode is a single choice of melee, campaign, or both", async ({ page }) => {
    await openPanel(page);
    await expect(page).toHaveURL(/mode=melee/);

    await page.locator(".mode-group button").filter({ hasText: /^Both$/ }).click();
    await expect(page).toHaveURL(/mode=melee,campaign/);

    await page
      .locator(".mode-group button")
      .filter({ hasText: /^Campaign$/ })
      .click();
    await expect(page).toHaveURL(/mode=campaign(&|$)/);
    await expect(page.locator(".mode-group .active-segment-option")).toHaveCount(1);
  });

  test("searching by ability finds units that carry it", async ({ page }) => {
    await openPanel(page);
    await page
      .locator(".find-units-by-group button")
      .filter({ hasText: /^Ability$/ })
      .click();

    await page.locator(".unit-list-search-input").fill("burrow");
    await expect(page).toHaveURL(/search_query=burrow/);
    await expect(page.locator(".unit-card").first()).toBeVisible();
  });

  test("the default race scope spans every race", async ({ page }) => {
    await openDialog(page);
    await page.locator(".unit-list-search-input").fill("demon");

    await expect(
      page.locator(".unit-card").filter({ hasText: "Demon Hunter" }),
    ).toBeVisible();
  });

  test("narrowing the race scope drops that race from the search", async ({
    page,
  }) => {
    await openPanel(page);
    await page.locator(".race-scope-trigger").click();
    await page.locator(".race-scope-panel").waitFor();

    await page
      .locator(".race-scope-panel .race-tab")
      .filter({ hasText: /night elf/i })
      .click();

    await page.locator(".unit-list-search-input").fill("demon");
    await expect(page).toHaveURL(/search_query=demon/);
    await expect(
      page.locator(".unit-card").filter({ hasText: "Demon Hunter" }),
    ).toHaveCount(0);
  });

  test("the last race in scope stays on", async ({ page }) => {
    await openPanel(page);
    await page.locator(".race-scope-trigger").click();
    await page.locator(".race-scope-panel").waitFor();
    const races = page.locator(".race-scope-panel .race-tab");

    for (const name of ["Human", "Orc", "Undead", "Neutral"]) {
      await races.filter({ hasText: new RegExp(`^${name}$`, "i") }).click();
    }
    await expect(page.locator(".race-scope-panel .active-race-tab")).toHaveCount(1);

    await page.locator(".race-scope-panel .active-race-tab .race-tab").click();
    await expect(page.locator(".race-scope-panel .active-race-tab")).toHaveCount(1);
  });

  test("results stack vertically and never scroll the dialog sideways", async ({
    page,
  }) => {
    await openDialog(page);
    const cards = page.locator(".unit-card");
    await cards.first().waitFor();

    const boxes = await cards.evaluateAll((nodes) =>
      nodes.slice(0, 3).map((node) => {
        const rect = node.getBoundingClientRect();
        return { x: Math.round(rect.x), y: Math.round(rect.y) };
      }),
    );
    expect(boxes.length).toBeGreaterThan(1);
    expect(boxes[1].x).toBe(boxes[0].x);
    expect(boxes[1].y).toBeGreaterThan(boxes[0].y);

    const overflows = await page.evaluate(() => {
      const body = document.querySelector(".search-dialog-body");
      if (!body) return true;
      return body.scrollWidth > body.clientWidth + 1;
    });
    expect(overflows).toBe(false);
  });

  test("picking a result opens that unit", async ({ page }) => {
    await openDialog(page);
    await page.locator(".unit-list-search-input").fill("demon");
    await expect(page).toHaveURL(/search_query=demon/);
    const hit = page.locator(".unit-card").filter({ hasText: "Demon Hunter" });
    await hit.waitFor();

    await hit.click();
    await expect(page).toHaveURL(/unit=Edem/i);
    await expect(page).toHaveURL(/race=nightelf/);
  });
});
