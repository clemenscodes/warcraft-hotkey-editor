import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

async function collisionCount(
  page: import("@playwright/test").Page,
): Promise<number> {
  const text = await page
    .locator('[data-action="view-collisions"]')
    .getAttribute("data-collision-count");
  return Number(text);
}

async function collisionState(
  page: import("@playwright/test").Page,
): Promise<string | null> {
  return page
    .locator('[data-action="view-collisions"]')
    .getAttribute("data-collision-state");
}

// Regression test: the Collisions button's count must shrink across the
// three canonical user actions (boot defaults → run Resolve → apply
// Grid Layout) and bottom out at zero with the clear/affirmative state.
// Hard-coded counts intentionally — they are the ground truth for the
// algorithm and any change in them should be a conscious decision.
test.describe("Collision count progression across the resolve workflow", () => {
  test("default template → resolve → apply grid drops the count from 99+ to 62 to 0", async ({
    page,
  }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();

    // Step 0 — apply the Default template so the working state matches
    // the canonical baseline (not whatever bundled boot state happens
    // to be loaded).  The other tests in this suite use the same
    // template-then-resolve pattern.
    await page.locator('[aria-label="Browse layout templates"]').click();
    await page
      .locator(".templates-dialog-shell .wc3-dialog-body button", { hasText: "Default" })
      .click();
    await page.locator('[role="alertdialog"]').first().waitFor();

    // After applying the Default template, the badge must read "99+"
    // (i.e. ≥ 100 collisions across the three classes) and the button
    // must be in its attention state.
    const initialCount = await collisionCount(page);
    expect(initialCount).toBeGreaterThanOrEqual(100);
    await expect(
      page
        .locator('[data-action="view-collisions"]')
        .locator('[data-collision-badge="true"]'),
    ).toHaveText("99+");
    expect(await collisionState(page)).toBe("attention");

    // Step 1 — Resolve cascade.  Cross-unit + intra-unit position
    // collisions go away, hotkey collisions remain.
    await page.locator('[aria-label="Resolve conflicts"]').click();
    await page.locator(".resolve-info-dialog").waitFor();
    await page.locator(".resolve-info-dialog button", { hasText: "Apply" }).click();
    await page
      .locator('[role="alertdialog"]')
      .filter({ hasText: "Cascade applied" })
      .waitFor();

    const afterResolve = await collisionCount(page);
    // 62 hotkey collisions remain after the cascade — these are
    // per-unit hotkey letter clashes (including ability vs. Cmd* on
    // the command card) that the position-only cascade does not
    // touch.  Apply Grid clears them in the next step.  Ability /
    // AbilityOff pairs at the same letter are deduped at the source
    // — they're the same button by design, not a collision.
    expect(afterResolve).toBe(62);
    expect(afterResolve).toBeLessThan(initialCount);
    expect(await collisionState(page)).toBe("attention");

    // Step 2 — Apply Grid: rewrites every ability hotkey so it lines up
    // with the (now deconflicted) positions.  Final state must be zero
    // collisions, the clear/affirmative button styling, and no badge.
    await page.locator('[aria-label="Edit global hotkey layout"]').click();
    await page.locator(".layout-editor-shell").waitFor();
    await page.locator(".layout-editor-shell button", { hasText: /apply/i }).click();
    await expect(page.locator(".layout-editor-shell")).toHaveCount(0);

    const finalCount = await collisionCount(page);
    expect(finalCount).toBe(0);
    expect(await collisionState(page)).toBe("clear");
    await expect(
      page
        .locator('[data-action="view-collisions"]')
        .locator('[data-collision-badge="true"]'),
    ).toHaveCount(0);
    await expect(page.locator('[data-action="view-collisions"]')).toHaveAttribute(
      "aria-label",
      /your config is clean/i,
    );
  });
});
