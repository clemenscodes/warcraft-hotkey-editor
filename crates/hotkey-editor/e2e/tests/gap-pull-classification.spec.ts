import { expect, test, type Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

// Regression for the cascade-plan move classification. Force of Nature (ACfr)
// is pushed off the right end of row 2 during phase 1, becomes unresolved, then
// phase 2 rehomes it into a freed cell in the SAME row (it lands at (0,2),
// having been stuck at (3,2)). A move that never leaves its row is a gap pull,
// not a cross-row spill — "Spill" is reserved for the genuine cross-row
// fallback. ACfr must therefore show up under the "Gap pulls" section and never
// under "Spills". The Default template gives the canonical collision-heavy plan.

async function applyDefaultTemplate(page: Page): Promise<void> {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();
  await page.locator('[aria-label="Browse layout templates"]').click();
  await page
    .locator(".template-card", { hasText: "Default" })
    .click();
  await page.locator('[role="alertdialog"]').first().waitFor();
}

async function openResolveSection(page: Page, slug: string): Promise<void> {
  await page.goto(`${APP}resolve?entry=${slug}`);
  await page.locator('.resolve-page').waitFor();
}

const ACFR = /^ACfr$/;

test.describe("Cascade plan move classification", () => {
  test("a same-row reflow (ACfr) is listed under Gap pulls", async ({ page }) => {
    await applyDefaultTemplate(page);
    await openResolveSection(page, "gap-pulls");

    // The "Gap pulls" tab (?entry=gap-pulls) is the active one, identified by its
    // visible label text rather than a DOM slug attribute.
    await expect(
      page.locator(".breadcrumbs button", { hasText: "Gap pulls" }),
    ).toHaveAttribute("aria-current", "page");

    const gapPulls = page.locator(".move-list");
    await expect(gapPulls).toBeVisible();
    await expect(
      gapPulls.locator("code.object-id", { hasText: ACFR }),
    ).toHaveCount(1);
  });

  test("ACfr is never listed under Spills", async ({ page }) => {
    await applyDefaultTemplate(page);
    await openResolveSection(page, "spills");

    // The default plan still has genuine cross-row spills, so the section must
    // exist — that is what makes the absence of ACfr meaningful rather than a
    // missing tab.
    const spills = page.locator(".move-list");
    await expect(spills).toBeVisible();
    await expect(
      spills.locator("code.object-id", { hasText: ACFR }),
    ).toHaveCount(0);
  });
});
