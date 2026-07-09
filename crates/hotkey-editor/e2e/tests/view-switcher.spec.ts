import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

test.describe("View routing — brand-as-home + collisions notification (#39)", () => {
  test("default URL renders the Editor view", async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await expect(page.locator("[data-collision-kind]")).toHaveCount(0);
    await expect(page.locator("[data-resolve-state]")).toHaveCount(0);
  });

  test("clicking the brand from the Collisions page navigates home to the editor", async ({
    page,
  }) => {
    await page.goto(`${APP}collisions?kind=positions`);
    await page.locator("[data-collision-kind]").waitFor();
    await page.locator('[data-action="view-editor"]').click();
    await page.locator(".unit-card").first().waitFor();
    await expect(page.locator("[data-collision-kind]")).toHaveCount(0);
    const url = new URL(page.url());
    expect(url.pathname).not.toContain("collisions");
  });

  test("the brand has an accessible label identifying it as a home link", async ({ page }) => {
    await page.goto(APP);
    const brand = page.locator('[data-action="view-editor"]').first();
    await expect(brand).toHaveAttribute("aria-label", /return to editor/i);
  });

  test("clicking the Collisions toolbar button navigates to the collisions page", async ({
    page,
  }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('[data-action="view-collisions"]').click();
    await page.waitForURL(/\/collisions/);
    await page.locator('[data-collision-kind="positions"]').waitFor();
  });

  test("Collisions button shows a numeric notification badge for the default keys", async ({
    page,
  }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    const collisionsButton = page.locator('[data-action="view-collisions"]');
    const countText = await collisionsButton.getAttribute("data-collision-count");
    const count = Number(countText);
    expect(Number.isFinite(count)).toBe(true);
    expect(count).toBeGreaterThan(0);
    await expect(collisionsButton).toHaveAttribute("data-collision-state", "attention");
    const badge = collisionsButton.locator('[data-collision-badge="true"]');
    await expect(badge).toBeVisible();
    await expect(badge).toHaveText(count >= 100 ? "99+" : String(count));
  });

  test("Collisions count tracks position, command-card hotkey, and system-hotkey collisions", async ({
    page,
  }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    const collisionsButton = page.locator('[data-action="view-collisions"]');
    const initialCountText = await collisionsButton.getAttribute("data-collision-count");
    const initialCount = Number(initialCountText);
    expect(initialCount).toBeGreaterThan(0);
    await expect(collisionsButton).toHaveAttribute("data-collision-state", "attention");

    // Running Resolve fixes position collisions only — hotkey collisions
    // remain, so the count drops but stays > 0.  The button must still
    // reflect that state ("attention" with a strictly lower count).
    await page.locator('[aria-label="Resolve conflicts"]').click();
    await page.locator('[data-action="apply-cascade"]').click();
    await page
      .locator('[role="alertdialog"]')
      .filter({ hasText: "Cascade applied" })
      .waitFor();
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();

    const afterResolveText = await collisionsButton.getAttribute("data-collision-count");
    const afterResolve = Number(afterResolveText);
    expect(afterResolve).toBeLessThan(initialCount);
  });

  test("Collisions button reaches the clean state after Resolve + Apply Grid", async ({
    page,
  }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();

    // Step 1: resolve cross-unit position collisions.
    await page.locator('[aria-label="Resolve conflicts"]').click();
    await page.locator('[data-action="apply-cascade"]').click();
    await page
      .locator('[role="alertdialog"]')
      .filter({ hasText: "Cascade applied" })
      .waitFor();
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();

    // Step 2: rewrite every hotkey to match the now-deconflicted grid.
    await page.locator('[aria-label="Edit global hotkey layout"]').click();
    await page.locator(".layout-editor-content").waitFor();
    await page.locator(".apply-button", { hasText: /apply/i }).click();
    await expect(page.locator(".layout-editor-content")).toHaveCount(0);

    const collisionsButton = page.locator('[data-action="view-collisions"]');
    await expect(collisionsButton).toHaveAttribute("data-collision-state", "clear");
    await expect(collisionsButton).toHaveAttribute("data-collision-count", "0");
    await expect(collisionsButton.locator('[data-collision-badge="true"]')).toHaveCount(0);
    await expect(collisionsButton).toHaveAttribute("aria-label", /your config is clean/i);
  });

  test("Collisions button stays visible on smaller viewports (mobile)", async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 900 });
    await page.goto(APP);
    await page.locator('[data-action="view-collisions"]').waitFor();
    await expect(page.locator('[data-action="view-collisions"]')).toBeVisible();
  });

  test("/collisions?kind=positions deep-links to the position collisions page", async ({
    page,
  }) => {
    await page.goto(`${APP}collisions?kind=positions`);
    await page.locator('[data-collision-kind="positions"]').waitFor();
  });

  test("/collisions?kind=hotkeys deep-links to the hotkey collisions page", async ({
    page,
  }) => {
    await page.goto(`${APP}collisions?kind=hotkeys`);
    await page.locator('[data-collision-kind="hotkeys"]').waitFor();
  });

  test("/resolve renders the resolve placeholder", async ({ page }) => {
    await page.goto(`${APP}resolve`);
    await page.locator("[data-resolve-state]").waitFor();
    await expect(page.locator(".unit-card")).toHaveCount(0);
  });

  test("an unknown path redirects to the Editor", async ({ page }) => {
    await page.goto(`${APP}nonsense`);
    await page.locator(".unit-card").first().waitFor();
  });

  test("browser back returns from Collisions to Editor (popstate)", async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('[data-action="view-collisions"]').click();
    await page.locator("[data-collision-kind]").waitFor();
    await page.goBack();
    await page.locator(".unit-card").first().waitFor();
  });

  test("browser forward re-enters Collisions after a back navigation", async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('[data-action="view-collisions"]').click();
    await page.locator("[data-collision-kind]").waitFor();
    await page.goBack();
    await page.locator(".unit-card").first().waitFor();
    await page.goForward();
    await page.locator("[data-collision-kind]").waitFor();
  });

  test("the editor selection survives a round-trip through the collisions page", async ({
    page,
  }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('.race-tabs [class*="orc-race-tab"]').click();
    await page.waitForURL(/race=orc/);
    // Switching to collisions does not carry the editor's race in the URL — that is
    // the editor's state, not the collisions page's. It is preserved in memory.
    await page.locator('[data-action="view-collisions"]').click();
    await page.waitForURL(/\/collisions/);
    expect(new URL(page.url()).pathname).toContain("collisions");
    expect(new URL(page.url()).searchParams.get("race")).toBeNull();
    // Returning to the editor restores the orc selection, back into the URL.
    await page.locator('[data-action="view-editor"]').click();
    await page.waitForURL(/race=orc/);
    await page.locator(".unit-card").first().waitFor();
  });
});
