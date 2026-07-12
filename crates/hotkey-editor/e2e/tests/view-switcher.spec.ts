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
    await page.goto(`${APP}collisions?kind=positions`);
    await page.locator(".collisions-page").waitFor();
    await page.locator('.brand').click();
    await page.locator(".unit-card").first().waitFor();
    await expect(page.locator(".collisions-page")).toHaveCount(0);
    const url = new URL(page.url());
    expect(url.pathname).not.toContain("collisions");
  });

  test("the brand has an accessible label identifying it as a home link", async ({ page }) => {
    await page.goto(APP);
    const brand = page.locator('.brand').first();
    await expect(brand).toHaveAttribute("aria-label", /return to editor/i);
  });

  test("clicking the Collisions toolbar button navigates to the collisions page", async ({
    page,
  }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('.collisions-button').click();
    await page.waitForURL(/\/collisions/);
    await page.locator(".positions-content").waitFor();
  });

  test("Collisions button shows a numeric notification badge for the default keys", async ({
    page,
  }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    const collisionsButton = page.locator(".collisions-button");
    // The presence of the count badge is the attention state; its text is the
    // live count (or "99+" when the count is at or above 100).
    const badge = collisionsButton.locator(".collisions-button-badge");
    await expect(badge).toBeVisible();
    const badgeText = ((await badge.textContent()) ?? "").trim();
    expect(badgeText).toMatch(/^(99\+|\d+)$/);
    if (badgeText !== "99+") {
      expect(Number(badgeText)).toBeGreaterThan(0);
    }
  });

  test("Collisions count tracks position, command-card hotkey, and system-hotkey collisions", async ({
    page,
  }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    const collisionsButton = page.locator(".collisions-button");
    const badge = collisionsButton.locator(".collisions-button-badge");
    // The badge caps at "99+" (≥ 100); read it as a lower bound of 100.
    const badgeCount = async (): Promise<number> => {
      if ((await badge.count()) === 0) {
        return 0;
      }
      const text = ((await badge.textContent()) ?? "").trim();
      return text === "99+" ? 100 : Number(text);
    };
    const initialCount = await badgeCount();
    expect(initialCount).toBeGreaterThan(0);
    await expect(badge).toBeVisible();

    // Running Resolve fixes position collisions only — hotkey collisions
    // remain, so the count drops but stays > 0.  The button must still
    // reflect that state ("attention" with a strictly lower count).
    await page.locator('[aria-label="Resolve conflicts"]').click();
    await page.locator(".apply-button", { hasText: /apply/i }).click();
    await page
      .locator('[role="alertdialog"]')
      .filter({ hasText: "Cascade applied" })
      .waitFor();
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();

    const afterResolve = await badgeCount();
    expect(afterResolve).toBeLessThan(initialCount);
  });

  test("Collisions button reaches the clean state after Resolve + Apply Grid", async ({
    page,
  }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();

    // Step 1: resolve cross-unit position collisions.
    await page.locator('[aria-label="Resolve conflicts"]').click();
    await page.locator(".apply-button", { hasText: /apply/i }).click();
    await page
      .locator('[role="alertdialog"]')
      .filter({ hasText: "Cascade applied" })
      .waitFor();
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();

    // Step 2: rewrite every hotkey to match the now-deconflicted grid.
    await page.locator('[aria-label="Edit global hotkey layout"]').click();
    await page.locator(".grid-layout-editor-dialog-content").waitFor();
    await page.locator(".apply-button", { hasText: /apply/i }).click();
    await expect(page.locator(".grid-layout-editor-dialog-content")).toHaveCount(0);

    const collisionsButton = page.locator(".collisions-button");
    // The clean state drops the count badge and switches the button's aria-label
    // to the all-clear message.
    await expect(collisionsButton.locator(".collisions-button-badge")).toHaveCount(0);
    await expect(collisionsButton).toHaveAttribute("aria-label", /your config is clean/i);
  });

  test("Collisions button stays visible on smaller viewports (mobile)", async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 900 });
    await page.goto(APP);
    await page.locator('.collisions-button').waitFor();
    await expect(page.locator('.collisions-button')).toBeVisible();
  });

  test("/collisions?kind=positions deep-links to the position collisions page", async ({
    page,
  }) => {
    await page.goto(`${APP}collisions?kind=positions`);
    await page.locator(".positions-content").waitFor();
  });

  test("/collisions?kind=hotkeys deep-links to the hotkey collisions page", async ({
    page,
  }) => {
    await page.goto(`${APP}collisions?kind=hotkeys`);
    await page.locator(".hotkeys-content").waitFor();
  });

  test("/resolve renders the resolve placeholder", async ({ page }) => {
    await page.goto(`${APP}resolve`);
    await page.locator(".resolve-page").waitFor();
    await expect(page.locator(".unit-card")).toHaveCount(0);
  });

  test("an unknown path redirects to the Editor", async ({ page }) => {
    await page.goto(`${APP}nonsense`);
    await page.locator(".unit-card").first().waitFor();
  });

  test("browser back returns from Collisions to Editor (popstate)", async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('.collisions-button').click();
    await page.locator(".collisions-page").waitFor();
    await page.goBack();
    await page.locator(".unit-card").first().waitFor();
  });

  test("browser forward re-enters Collisions after a back navigation", async ({ page }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();
    await page.locator('.collisions-button').click();
    await page.locator(".collisions-page").waitFor();
    await page.goBack();
    await page.locator(".unit-card").first().waitFor();
    await page.goForward();
    await page.locator(".collisions-page").waitFor();
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
    await page.locator('.collisions-button').click();
    await page.waitForURL(/\/collisions/);
    expect(new URL(page.url()).pathname).toContain("collisions");
    expect(new URL(page.url()).searchParams.get("race")).toBeNull();
    // Returning to the editor restores the orc selection, back into the URL.
    await page.locator('.brand').click();
    await page.waitForURL(/race=orc/);
    await page.locator(".unit-card").first().waitFor();
  });
});
