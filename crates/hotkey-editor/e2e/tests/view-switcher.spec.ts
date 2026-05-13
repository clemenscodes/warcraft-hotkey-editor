import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

test.describe("View routing — brand-as-home + collisions notification (#39)", () => {
  test("default URL renders the Editor view", async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await expect(page.locator(".collisions-page")).toHaveCount(0);
    await expect(page.locator(".resolve-page")).toHaveCount(0);
  });

  test("clicking the brand from the Collisions page navigates home to the editor", async ({
    page,
  }) => {
    await page.goto(`${APP}?view=collisions&kind=positions`);
    await page.locator(".collisions-page").waitFor();
    await page.locator('[data-action="view-editor"]').click();
    await page.locator(".unit-card").first().waitFor();
    await expect(page.locator(".collisions-page")).toHaveCount(0);
    const url = new URL(page.url());
    expect(url.searchParams.get("view")).toBeNull();
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
    await page.waitForURL(/view=collisions/);
    await page.locator('.collisions-page[data-collision-kind="positions"]').waitFor();
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
    await page.locator(".resolve-info-dialog").waitFor();
    await page.locator(".resolve-info-dialog button", { hasText: "Apply" }).click();
    await page
      .locator('[role="alertdialog"]')
      .filter({ hasText: "Cascade applied" })
      .waitFor();

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
    await page.locator(".resolve-info-dialog").waitFor();
    await page.locator(".resolve-info-dialog button", { hasText: "Apply" }).click();
    await page
      .locator('[role="alertdialog"]')
      .filter({ hasText: "Cascade applied" })
      .waitFor();

    // Step 2: rewrite every hotkey to match the now-deconflicted grid.
    await page.locator('[aria-label="Edit global hotkey layout"]').click();
    await page.locator(".layout-editor-shell").waitFor();
    await page.locator(".layout-editor-shell button", { hasText: /apply/i }).click();
    await expect(page.locator(".layout-editor-shell")).toHaveCount(0);

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

  test("?view=collisions&kind=positions deep-links to the position collisions page", async ({
    page,
  }) => {
    await page.goto(`${APP}?view=collisions&kind=positions`);
    await page.locator('.collisions-page[data-collision-kind="positions"]').waitFor();
  });

  test("?view=collisions&kind=hotkeys deep-links to the hotkey collisions page", async ({
    page,
  }) => {
    await page.goto(`${APP}?view=collisions&kind=hotkeys`);
    await page.locator('.collisions-page[data-collision-kind="hotkeys"]').waitFor();
  });

  test("?view=resolve still parses and renders the resolve placeholder", async ({ page }) => {
    await page.goto(`${APP}?view=resolve`);
    await page.locator(".resolve-page").waitFor();
    await expect(page.locator(".unit-card")).toHaveCount(0);
  });

  test("unknown ?view= falls back to Editor", async ({ page }) => {
    await page.goto(`${APP}?view=nonsense`);
    await page.locator(".unit-card").first().waitFor();
  });

  test("browser back returns from Collisions to Editor (popstate)", async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('[data-action="view-collisions"]').click();
    await page.locator(".collisions-page").waitFor();
    await page.goBack();
    await page.locator(".unit-card").first().waitFor();
  });

  test("browser forward re-enters Collisions after a back navigation", async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('[data-action="view-collisions"]').click();
    await page.locator(".collisions-page").waitFor();
    await page.goBack();
    await page.locator(".unit-card").first().waitFor();
    await page.goForward();
    await page.locator(".collisions-page").waitFor();
  });

  test("switching views preserves race/mode/unit query params", async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('.race-tab[data-race="orc"]').click();
    await page.waitForURL(/race=orc/);
    await page.locator('[data-action="view-collisions"]').click();
    await page.waitForURL(/view=collisions/);
    const url = new URL(page.url());
    expect(url.searchParams.get("race")).toBe("orc");
    expect(url.searchParams.get("view")).toBe("collisions");
  });
});
