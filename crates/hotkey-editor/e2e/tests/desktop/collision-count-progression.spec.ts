import { expect, test } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

async function collisionCount(
  page: import("@playwright/test").Page,
): Promise<number> {
  const badge = page.locator(".collisions-button .collisions-button-badge");
  if ((await badge.count()) === 0) {
    return 0;
  }
  const text = ((await badge.textContent()) ?? "").trim();
  return text === "99+" ? 100 : Number(text);
}

async function inAttentionState(
  page: import("@playwright/test").Page,
): Promise<boolean> {
  return (
    (await page
      .locator(".collisions-button .collisions-button-badge")
      .count()) > 0
  );
}

test.describe("Collision count progression across the resolve workflow", () => {
  test("default template → resolve → apply grid drops the count from 99+ to 55 to 0", async ({
    page,
  }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();

    await page.locator('[aria-label="Browse layout templates"]').click();
    await page
      .locator(".template-card", { hasText: "Default" })
      .click();
    await page.locator('[role="alertdialog"]').first().waitFor();

    const initialCount = await collisionCount(page);
    expect(initialCount).toBeGreaterThanOrEqual(100);
    await expect(
      page
        .locator('.collisions-button')
        .locator('.collisions-button-badge'),
    ).toHaveText("99+");
    expect(await inAttentionState(page)).toBe(true);

    await page.locator('[aria-label="Resolve conflicts"]').click();
    await page.locator(".apply-button", { hasText: /apply/i }).click();
    await page
      .locator('[role="alertdialog"]')
      .filter({ hasText: "Cascade applied" })
      .waitFor();
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();

    const afterResolve = await collisionCount(page);
    expect(afterResolve).toBe(55);
    expect(afterResolve).toBeLessThan(initialCount);
    expect(await inAttentionState(page)).toBe(true);

    await page.locator('[aria-label="Edit global hotkey layout"]').click();
    await page.locator(".grid-layout-editor-dialog-content").waitFor();
    await page.locator(".apply-button", { hasText: /apply/i }).click();
    await expect(page.locator(".grid-layout-editor-dialog-content")).toHaveCount(0);

    const finalCount = await collisionCount(page);
    expect(finalCount).toBe(0);
    expect(await inAttentionState(page)).toBe(false);
    await expect(
      page
        .locator('.collisions-button')
        .locator('.collisions-button-badge'),
    ).toHaveCount(0);
    await expect(page.locator('.collisions-button')).toHaveAttribute(
      "aria-label",
      /your config is clean/i,
    );
  });
});
