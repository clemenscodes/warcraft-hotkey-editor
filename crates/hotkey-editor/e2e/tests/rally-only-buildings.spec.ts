import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

// Buildings whose rendered command card carries no ability of their own and
// whose only command is the rally point. They are dead placeholders for hotkey
// editing — there is nothing to rebind but the rally point — so the catalog
// hides them in curated browsing and surfaces them only under "No abilities".
// All seven are Neutral campaign buildings, so they live in Campaign mode.
const RALLY_ONLY_BUILDINGS = ["ndmg", "ndke", "ndkw", "ndrb", "ndh3", "ndh4", "nheb"];

// The unit id renders in a <code> inside each .unit-card; match a card by its
// exact id so two same-named cards (the two Dimensional Gates) stay distinct.
function unitCardById(page: any, unitId: string) {
  return page.locator(".unit-card").filter({
    has: page.locator("code", { hasText: new RegExp(`^${unitId}$`) }),
  });
}

async function browseNeutralCampaign(page: any) {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();
  await page.locator('.race-tabs [class*="neutral-race-tab"]').click();
  await page.locator('.race-tabs .neutral-race-tab .active-race-tab').waitFor();
  const campaign = page.getByRole("button", { name: "Campaign" });
  await campaign.click();
  await expect(campaign).toHaveAttribute("aria-pressed", "true");
  await page.locator(".unit-card").first().waitFor();
}

test.describe("Rally-only buildings are hidden by default", () => {
  test("Demon Gate and its kin do not list in curated Neutral/Campaign browse", async ({
    page,
  }) => {
    await browseNeutralCampaign(page);
    for (const unitId of RALLY_ONLY_BUILDINGS) {
      await expect(
        unitCardById(page, unitId),
        `${unitId} (rally-only building) must be hidden by default`,
      ).toHaveCount(0);
    }
  });

  test("the 'No abilities' toggle reveals the rally-only buildings", async ({ page }) => {
    await browseNeutralCampaign(page);
    const noAbilities = page.getByRole("button", { name: "No abilities" });
    await noAbilities.click();
    await expect(noAbilities).toHaveAttribute("aria-pressed", "true");
    for (const unitId of RALLY_ONLY_BUILDINGS) {
      await expect(
        unitCardById(page, unitId),
        `${unitId} (rally-only building) must surface once ability-less units are shown`,
      ).toHaveCount(1);
    }
  });
});
