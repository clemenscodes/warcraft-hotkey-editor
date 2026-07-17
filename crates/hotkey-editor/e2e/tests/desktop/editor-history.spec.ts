import { expect, test, type Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

function unitParam(page: Page): Promise<string | null> {
  return page.evaluate(() => new URL(location.href).searchParams.get("unit"));
}

test.describe("Editor selection history (back/forward)", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
  });

  test("back/forward steps through race selections and restores the unit list", async ({
    page,
  }) => {
    await expect(page).toHaveURL(/race=human/);

    await page.locator('.race-tabs [class*="orc-race-tab"]').click();
    await expect(page).toHaveURL(/race=orc/);
    await page.locator(".unit-card").filter({ hasText: "Grunt" }).first().waitFor();

    await page.locator('.race-tabs [class*="undead-race-tab"]').click();
    await expect(page).toHaveURL(/race=undead/);

    await page.goBack();
    await expect(page).toHaveURL(/race=orc/);
    await page.locator(".unit-card").filter({ hasText: "Grunt" }).first().waitFor();

    await page.goBack();
    await expect(page).toHaveURL(/race=human/);
    await page.locator(".unit-card").filter({ hasText: "Footman" }).first().waitFor();

    await page.goForward();
    await expect(page).toHaveURL(/race=orc/);
  });

  // The modes are independent, not a switch: turning campaign on adds it to
  // melee rather than replacing it, so the URL carries both.
  test("back steps through mode selections", async ({ page }) => {
    await expect(page).toHaveURL(/mode=melee/);

    await page.locator("button", { hasText: "Campaign" }).click();
    await expect(page).toHaveURL(/mode=melee,campaign/);

    await page.goBack();
    await expect(page).toHaveURL(/mode=melee(&|$)/);
  });

  test("turning off the last mode is refused, so the list is never empty", async ({
    page,
  }) => {
    await expect(page).toHaveURL(/mode=melee/);

    // Melee is the only mode on, so clicking it off would list nothing at all —
    // which is never what clicking a filter means.
    await page.locator("button", { hasText: "Melee" }).click();
    await expect(page).toHaveURL(/mode=melee/);
    await expect(page.locator(".unit-card").first()).toBeVisible();
  });

  test("back steps through selected units", async ({ page }) => {
    const first = await unitParam(page);

    await page.locator(".unit-card").nth(3).click();
    await expect.poll(() => unitParam(page)).not.toBe(first);
    const second = await unitParam(page);

    await page.locator(".unit-card").nth(6).click();
    await expect.poll(() => unitParam(page)).not.toBe(second);
    const third = await unitParam(page);
    expect(third).not.toBeNull();

    await page.goBack();
    await expect.poll(() => unitParam(page)).toBe(second);

    await page.goBack();
    await expect.poll(() => unitParam(page)).toBe(first);

    await page.goForward();
    await expect.poll(() => unitParam(page)).toBe(second);
  });

  test("typing a search adds a single coalesced history entry", async ({ page }) => {
    const search = page.locator('input[type="search"]');
    const before = await page.evaluate(() => history.length);

    await search.pressSequentially("Footman", { delay: 60 });
    await expect(page).toHaveURL(/search_query=Footman/);

    const after = await page.evaluate(() => history.length);
    expect(after - before).toBe(1);

    await page.goBack();
    await expect(page).not.toHaveURL(/search_query=/);
    await expect(search).toHaveValue("");
  });

  test("distinct searches after a pause are separate history entries", async ({
    page,
  }) => {
    const search = page.locator('input[type="search"]');

    await search.fill("Footman");
    await expect(page).toHaveURL(/search_query=Footman/);
    await page.waitForTimeout(700);

    await search.fill("Grunt");
    await expect(page).toHaveURL(/search_query=Grunt/);

    await page.goBack();
    await expect(page).toHaveURL(/search_query=Footman/);
    await expect(search).toHaveValue("Footman");

    await page.goBack();
    await expect(page).not.toHaveURL(/search_query=/);
    await expect(search).toHaveValue("");

    await page.goForward();
    await expect(page).toHaveURL(/search_query=Footman/);
    await expect(search).toHaveValue("Footman");
  });
});
