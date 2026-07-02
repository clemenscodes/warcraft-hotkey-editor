import { expect, test, type Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

// Deep-linking on the Resolve (cascade plan) page: the selected move-category
// breadcrumb (Fights / Gap pulls / Spills / Swaps) rides in the `?entry=` URL
// param — the same generic selection slot the collision pages use — so the
// viewed section survives leaving the page (back/forward) and is restorable from
// a pasted/bookmarked URL. The Default template is applied first so the plan is
// the canonical collision-heavy baseline with several move categories.

async function applyDefaultTemplate(page: Page): Promise<void> {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();
  await page.locator('[aria-label="Browse layout templates"]').click();
  await page
    .locator(".template-card", { hasText: "Default" })
    .click();
  await page.locator('[role="alertdialog"]').first().waitFor();
}

async function openResolvePlan(page: Page): Promise<void> {
  await page.goto(`${APP}?view=resolve`);
  await page.locator('[data-resolve-state="plan"]').waitFor();
  await page.locator(".breadcrumbs [data-breadcrumb]").first().waitFor();
}

function categorySlugs(page: Page): Promise<string[]> {
  return page
    .locator(".breadcrumbs [data-breadcrumb]")
    .evaluateAll((elements) =>
      elements.map((element) => element.getAttribute("data-breadcrumb") ?? ""),
    );
}

test.describe("Resolve page move-category deep-linking", () => {
  // Landing on the plan via deep-link with no `entry` defaults to the first
  // section being active and no entry in the URL.
  test("the plan defaults to the first move category with no entry in the URL", async ({
    page,
  }) => {
    await applyDefaultTemplate(page);
    await openResolvePlan(page);

    const slugs = await categorySlugs(page);
    expect(slugs.length).toBeGreaterThan(0);

    await expect(
      page.locator(`.breadcrumbs [data-breadcrumb="${slugs[0]}"]`),
    ).toHaveAttribute("aria-current", "page");
    await expect(page).not.toHaveURL(/entry=/);
  });

  // Clicking a move-category breadcrumb writes its slug to the `?entry=` param
  // (via replace, so it does not spam history) and highlights it.
  test("selecting a move category writes it to the URL and highlights it", async ({
    page,
  }) => {
    await applyDefaultTemplate(page);
    await openResolvePlan(page);

    const slugs = await categorySlugs(page);
    // Pick the last category so the assertion is meaningful even when it is not
    // the default first section.
    const target = slugs[slugs.length - 1];

    const breadcrumb = page.locator(`.breadcrumbs [data-breadcrumb="${target}"]`);
    await breadcrumb.click();
    await expect(page).toHaveURL(new RegExp(`entry=${target}(&|$)`));
    await expect(breadcrumb).toHaveAttribute("aria-current", "page");
  });

  // A pasted/bookmarked deep-link to a move category selects that section on
  // load — the viewed category is restored from the URL, not reset to the first.
  test("a deep-linked move category is selected on load", async ({ page }) => {
    await applyDefaultTemplate(page);
    await openResolvePlan(page);

    const slugs = await categorySlugs(page);
    const target = slugs[slugs.length - 1];

    await page.goto(`${APP}?view=resolve&entry=${target}`);
    await expect(
      page.locator(`.breadcrumbs [data-breadcrumb="${target}"]`),
    ).toHaveAttribute("aria-current", "page");
  });

  // Selecting a category, leaving for the editor, then browser-back lands on the
  // plan with the same category still selected (popstate restore).
  test("the selected move category survives leaving and returning via back", async ({
    page,
  }) => {
    await applyDefaultTemplate(page);
    await openResolvePlan(page);

    const slugs = await categorySlugs(page);
    const target = slugs[slugs.length - 1];

    await page.locator(`.breadcrumbs [data-breadcrumb="${target}"]`).click();
    await expect(page).toHaveURL(new RegExp(`entry=${target}(&|$)`));

    // Leave for the editor (pushes a history entry), then go back.
    await page.locator('[data-action="view-editor"]').click();
    await page.locator(".unit-card").first().waitFor();

    await page.goBack();
    await page.locator('[data-resolve-state="plan"]').waitFor();
    await expect(page).toHaveURL(new RegExp(`entry=${target}(&|$)`));
    await expect(
      page.locator(`.breadcrumbs [data-breadcrumb="${target}"]`),
    ).toHaveAttribute("aria-current", "page");
  });
});
