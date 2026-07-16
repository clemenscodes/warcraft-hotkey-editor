import { expect, test, type Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

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
