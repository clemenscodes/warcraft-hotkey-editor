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

async function openResolvePlan(page: Page): Promise<void> {
  await page.goto(`${APP}resolve`);
  await page.locator('.resolve-page').waitFor();
  await page.locator(".breadcrumbs button").first().waitFor();
}

function breadcrumbs(page: Page) {
  return page.locator(".breadcrumbs button");
}

// A breadcrumb tab is identified by its visible label text (its slug is no longer
// exposed as a DOM attribute). Section titles are unique and not substrings of one
// another, so `hasText` uniquely and stably picks the tab; the count child does
// not interfere.
function breadcrumbByLabel(page: Page, label: string) {
  return page.locator(".breadcrumbs button", { hasText: label });
}

// The move-category slug is not a DOM attribute; it is what the app writes to
// `?entry=` when a tab is picked. Click the last tab, then read back both its
// visible label (to re-find it) and its slug (from the URL). This mirrors how the
// collision suite discovers an entry's key by reading it out of the URL.
async function selectLastCategory(
  page: Page,
): Promise<{ slug: string; label: string }> {
  const last = breadcrumbs(page).last();
  const label = ((await last.locator(".breadcrumb-label").textContent()) ?? "").trim();
  await last.click();
  await expect(page).toHaveURL(/entry=/);
  const slug = new URL(page.url()).searchParams.get("entry") ?? "";
  return { slug, label };
}

test.describe("Resolve page move-category deep-linking", () => {
  // Landing on the plan via deep-link with no `entry` defaults to the first
  // section being active and no entry in the URL.
  test("the plan defaults to the first move category with no entry in the URL", async ({
    page,
  }) => {
    await applyDefaultTemplate(page);
    await openResolvePlan(page);

    const tabs = breadcrumbs(page);
    expect(await tabs.count()).toBeGreaterThan(0);

    await expect(tabs.first()).toHaveAttribute("aria-current", "page");
    await expect(page).not.toHaveURL(/entry=/);
  });

  // Clicking a move-category breadcrumb writes its slug to the `?entry=` param
  // (via replace, so it does not spam history) and highlights it.
  test("selecting a move category writes it to the URL and highlights it", async ({
    page,
  }) => {
    await applyDefaultTemplate(page);
    await openResolvePlan(page);

    // Pick the last category so the assertion is meaningful even when it is not
    // the default first section.
    const { slug, label } = await selectLastCategory(page);
    await expect(page).toHaveURL(new RegExp(`entry=${slug}(&|$)`));
    await expect(breadcrumbByLabel(page, label)).toHaveAttribute(
      "aria-current",
      "page",
    );
  });

  // A pasted/bookmarked deep-link to a move category selects that section on
  // load — the viewed category is restored from the URL, not reset to the first.
  test("a deep-linked move category is selected on load", async ({ page }) => {
    await applyDefaultTemplate(page);
    await openResolvePlan(page);

    const { slug, label } = await selectLastCategory(page);

    await page.goto(`${APP}resolve?entry=${slug}`);
    await expect(breadcrumbByLabel(page, label)).toHaveAttribute(
      "aria-current",
      "page",
    );
  });

  // Selecting a category, leaving for the editor, then browser-back lands on the
  // plan with the same category still selected (popstate restore).
  test("the selected move category survives leaving and returning via back", async ({
    page,
  }) => {
    await applyDefaultTemplate(page);
    await openResolvePlan(page);

    const { slug, label } = await selectLastCategory(page);
    await expect(page).toHaveURL(new RegExp(`entry=${slug}(&|$)`));

    // Leave for the editor (pushes a history entry), then go back.
    await page.locator('.brand').click();
    await page.locator(".unit-card").first().waitFor();

    await page.goBack();
    await page.locator('.resolve-page').waitFor();
    await expect(page).toHaveURL(new RegExp(`entry=${slug}(&|$)`));
    await expect(breadcrumbByLabel(page, label)).toHaveAttribute(
      "aria-current",
      "page",
    );
  });
});
