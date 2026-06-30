import { expect, test, type Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";
const LS_KEY = "warcraft-hotkey-editor.custom-keys";

// Multi-level upgrades bind one hotkey token per research tier
// (`Hotkey=Q,Q,Q`). A single token binds only tier 1, so the follow-up upgrade
// levels — graveyard attack/armor, Banshee/Necromancer training — silently
// lose their hotkey in game. normalize() must replicate the token to the
// upgrade's tier count, and localStorage is the canonical export, so we assert
// against it directly.

// Each Undead multi-level upgrade and its tier count.
const UPGRADES: Array<{ id: string; tiers: number; name: string }> = [
  { id: "Rume", tiers: 3, name: "Creature Attack (graveyard)" },
  { id: "Ruar", tiers: 3, name: "Creature Carapace (graveyard)" },
  { id: "Rune", tiers: 2, name: "Necromancer training" },
  { id: "Ruba", tiers: 2, name: "Banshee training" },
];

function hotkeyForSection(stored: string, sectionId: string): string | null {
  const lines = stored.split(/\r?\n/);
  const headerIndex = lines.indexOf(`[${sectionId}]`);
  if (headerIndex === -1) {
    return null;
  }
  for (let index = headerIndex + 1; index < lines.length; index++) {
    if (lines[index].startsWith("[")) {
      break;
    }
    const match = lines[index].match(/^Hotkey=(.*)$/);
    if (match) {
      return match[1];
    }
  }
  return null;
}

async function readStored(page: Page): Promise<string> {
  // The persistence effect writes the normalized text on the next tick; poll
  // until a known multi-tier upgrade lands in storage so the read is stable.
  await expect
    .poll(async () => {
      const stored = await page.evaluate(
        (key) => localStorage.getItem(key),
        LS_KEY,
      );
      return stored ? hotkeyForSection(stored, "Rume") : null;
    })
    .toMatch(/^.+,.+,.+$/);
  const stored = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
  expect(stored).not.toBeNull();
  return stored as string;
}

function assertUpgradeTiers(stored: string): void {
  for (const upgrade of UPGRADES) {
    const hotkey = hotkeyForSection(stored, upgrade.id);
    expect(
      hotkey,
      `${upgrade.name} (${upgrade.id}) must have a hotkey`,
    ).not.toBeNull();
    const tokens = (hotkey as string).split(",").map((token) => token.trim());
    expect(
      tokens.length,
      `${upgrade.name} (${upgrade.id}) must bind ${upgrade.tiers} tiers, got Hotkey=${hotkey}`,
    ).toBe(upgrade.tiers);
    const uniqueTokens = new Set(tokens);
    expect(
      uniqueTokens.size,
      `${upgrade.name} (${upgrade.id}) tiers must share one key, got Hotkey=${hotkey}`,
    ).toBe(1);
  }
}

test.describe("Upgrade hotkey tiers", () => {
  // The real bug: importing a third-party CustomKeys.txt whose multi-level
  // upgrades carry a single hotkey token. normalize() must restore one token
  // per tier on export. This guards the materialization regardless of what the
  // bundled templates ship.
  test("importing single-token multi-level upgrades replicates them per tier on export", async ({
    page,
  }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();

    const importedFile = [
      "[Rume]",
      "Hotkey=Q",
      "Buttonpos=0,0",
      "",
      "[Ruar]",
      "Hotkey=A",
      "Buttonpos=0,1",
      "",
      "[Rune]",
      "Hotkey=Y",
      "Buttonpos=0,2",
      "",
      "[Ruba]",
      "Hotkey=X",
      "Buttonpos=1,2",
      "",
    ].join("\r\n");

    await page.locator("#upload-customkeys-input").setInputFiles({
      name: "CustomKeys.txt",
      mimeType: "text/plain",
      buffer: Buffer.from(importedFile),
    });
    await page.locator('[role="alertdialog"]').first().waitFor();

    const stored = await readStored(page);
    assertUpgradeTiers(stored);
  });

  // Companion guard: the bundled templates themselves must ship complete
  // per-tier upgrade hotkeys, and applying one must keep them complete on
  // export. Catches a template regressing back to single-token upgrades.
  test("applying the Clemens (QWERTZ) template exports complete per-tier upgrade hotkeys", async ({
    page,
  }) => {
    await page.goto(APP);
    await page.locator(".unit-card").first().waitFor();

    await page.locator('[aria-label="Browse layout templates"]').click();
    await page
      .locator(".template-card", {
        hasText: "Clemens (QWERTZ)",
      })
      .click();
    await page.locator('[role="alertdialog"]').first().waitFor();

    const stored = await readStored(page);
    assertUpgradeTiers(stored);
  });
});
