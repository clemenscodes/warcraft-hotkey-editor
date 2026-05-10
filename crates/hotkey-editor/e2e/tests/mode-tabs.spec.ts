import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

test.describe("Mode tabs (Melee / Campaign)", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(APP);
    await page.waitForSelector(".unit-card");
  });

  test("Melee mode is active by default", async ({ page }) => {
    const meleeButton = page.locator("button", { hasText: "Melee" });
    await expect(meleeButton).toHaveAttribute("data-active", "true");
  });

  test("clicking Campaign switches to campaign mode", async ({ page }) => {
    const campaignButton = page.locator("button", { hasText: "Campaign" });
    await campaignButton.click();
    await expect(campaignButton).toHaveAttribute("data-active", "true");
    const meleeButton = page.locator("button", { hasText: "Melee" });
    await expect(meleeButton).toHaveAttribute("data-active", "false");
  });

  test("switching to Campaign loads campaign units", async ({ page }) => {
    await page.locator("button", { hasText: "Campaign" }).click();
    await expect(page.locator(".unit-card").first()).toBeVisible();
  });

  test("switching back to Melee restores melee units", async ({ page }) => {
    await page.locator("button", { hasText: "Campaign" }).click();
    await page.locator("button", { hasText: "Melee" }).click();
    await expect(page.locator("button", { hasText: "Melee" })).toHaveAttribute("data-active", "true");
    await expect(page.locator(".unit-card").first()).toBeVisible();
  });
});
