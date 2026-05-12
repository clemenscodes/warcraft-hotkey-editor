import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

async function applyTemplateAndCascade(page: import("@playwright/test").Page) {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();

  await page.locator('[aria-label="Browse layout templates"]').click();
  await page.locator(".templates-dialog-shell .wc3-dialog-body button", { hasText: "Default" }).click();
  await page.locator('[role="alertdialog"]').first().waitFor();

  await page.locator('[aria-label="Resolve conflicts"]').click();
  await page.locator(".resolve-info-dialog").waitFor();
  await page.locator(".resolve-info-dialog button", { hasText: "Apply" }).click();
  await page
    .locator('[role="alertdialog"]')
    .filter({ hasText: "Cascade applied" })
    .waitFor();
}

test.describe("Destroyer regression: abilList order respected after cascade", () => {
  test.beforeEach(async ({ page }) => {
    await applyTemplateAndCascade(page);
  });

  // Regression for the intra-unit cascade tiebreak:
  // Advm and Aabs both default to (0,2) on the Destroyer (ubsp).  Before the
  // fix, the alphabetical tiebreak displaced Advm AND Afak (2 moves).  The
  // correct behaviour is to use the unit's abilList order as the tiebreak so
  // that Advm (index 0) wins and only Aabs (index 3) is displaced.
  //
  // Expected layout on Destroyer (ubsp) after template + cascade:
  //   (0,2) Advm — Devour Magic        ← wins intra-unit fight (abilList index 0)
  //   (1,2) Afak — Orb of Annihilation ← stays at default
  //   (2,2) ACmi — Spell Immunity      ← stays at default
  //   (3,2) Aabs — Absorb Mana         ← displaced here by cascade
  test("Destroyer command card row 2 follows abilList priority after cascade", async ({
    page,
  }) => {
    await page.locator('.race-tab[data-race="undead"]').click();
    await page.locator('input[type="search"]').fill("ubsp");
    await page.locator(".unit-card").filter({ hasText: "Destroyer" }).waitFor();
    await page.locator(".unit-card").filter({ hasText: "Destroyer" }).click();
    await page.locator(".grid-tile.has-ability").first().waitFor();

    const cell = (col: number, row: number) =>
      page.locator(
        `[data-grid-section="Command card"][data-grid-col="${col}"][data-grid-row="${row}"]`,
      );

    await expect(cell(0, 2).locator("img")).toHaveAttribute("alt", "Devour Magic");
    await expect(cell(1, 2).locator("img")).toHaveAttribute("alt", "Orb of Annihilation");
    await expect(cell(2, 2).locator("img")).toHaveAttribute("alt", "Spell Immunity");
    await expect(cell(3, 2).locator("img")).toHaveAttribute("alt", "Absorb Mana");
  });
});
