import { expect, test, type Locator, type Page } from "@playwright/test";

// Reproduces the starting state for the "QWER / ASDF / YXCV" rearrange scenario
// on the Archmage (Hamg), the first unit shown by default.
//
// After applying the Default template, the Archmage command card looks like this
// (cells are addressed as col,row; col 0..3 left to right, row 0..2 top to bottom):
//
//   (0,0) Move      (1,0) Stop      (2,0) Hold      (3,0) Attack
//   (0,1) Patrol    (1,1) empty     (2,1) empty     (3,1) fixed slot
//   (0,2) Ability1  (1,2) Ability2  (2,2) Ability3  (3,2) Ability4
//
// The desired layout puts the four hero abilities on the top row, attack on the
// home key, and the basic orders on the bottom row:
//
//   (0,0) Ability1  (1,0) Ability2  (2,0) Ability3  (3,0) Ability4
//   (0,1) Attack    (1,1) empty     (2,1) empty     (3,1) fixed slot
//   (0,2) Stop      (1,2) Hold      (2,2) Patrol    (3,2) Move
//
// The (3,1) slot and the two empty cells (1,1) and (2,1) are never touched.
//
// This is a single 9-cycle. Dropping a tile onto an occupied cell SWAPS the two
// tiles, so the whole permutation is realized with 8 swaps. Steps 1..4 swap each
// top-row order with the ability directly below it; steps 5..8 rotate the rest
// into place. The sequence is verified below against the rendered grid and
// against localStorage (the canonical source of truth).

const APP = "/warcraft-hotkey-editor/";
const LS_KEY = "warcraft-hotkey-editor.custom-keys";
const SECTION = "Command card";

interface GridCoordinate {
  col: number;
  row: number;
}

// The 8 drag operations, source then target, both occupied (so each is a swap).
const SWAP_SEQUENCE: { from: GridCoordinate; to: GridCoordinate }[] = [
  { from: { col: 0, row: 0 }, to: { col: 0, row: 2 } },
  { from: { col: 1, row: 0 }, to: { col: 1, row: 2 } },
  { from: { col: 2, row: 0 }, to: { col: 2, row: 2 } },
  { from: { col: 3, row: 0 }, to: { col: 3, row: 2 } },
  { from: { col: 0, row: 1 }, to: { col: 3, row: 2 } },
  { from: { col: 0, row: 2 }, to: { col: 1, row: 2 } },
  { from: { col: 1, row: 2 }, to: { col: 2, row: 2 } },
  { from: { col: 2, row: 2 }, to: { col: 3, row: 2 } },
];

// Where the tile originally at each source cell must end up after the 8 swaps.
const EXPECTED_MOVES: { from: GridCoordinate; to: GridCoordinate }[] = [
  { from: { col: 0, row: 0 }, to: { col: 3, row: 2 } }, // Move
  { from: { col: 1, row: 0 }, to: { col: 0, row: 2 } }, // Stop
  { from: { col: 2, row: 0 }, to: { col: 1, row: 2 } }, // Hold
  { from: { col: 3, row: 0 }, to: { col: 0, row: 1 } }, // Attack
  { from: { col: 0, row: 1 }, to: { col: 2, row: 2 } }, // Patrol
  { from: { col: 0, row: 2 }, to: { col: 0, row: 0 } }, // Ability 1
  { from: { col: 1, row: 2 }, to: { col: 1, row: 0 } }, // Ability 2
  { from: { col: 2, row: 2 }, to: { col: 2, row: 0 } }, // Ability 3
  { from: { col: 3, row: 2 }, to: { col: 3, row: 0 } }, // Ability 4
];

const EMPTY_CELLS: GridCoordinate[] = [
  { col: 1, row: 1 },
  { col: 2, row: 1 },
];

function cell(page: Page, coordinate: GridCoordinate): Locator {
  return page
    .locator(".grid-editor", {
      has: page.locator(".grid-heading", { hasText: SECTION }),
    })
    .locator(".grid-editor-tile")
    .nth(coordinate.row * 4 + coordinate.col);
}

function coordinateKey(coordinate: GridCoordinate): string {
  return `${coordinate.col},${coordinate.row}`;
}

// The visible label of whatever occupies a cell (icon alt text), or null if empty.
async function labelAt(page: Page, coordinate: GridCoordinate): Promise<string | null> {
  const target = cell(page, coordinate);
  if ((await target.locator(".filled-tile").count()) === 0) return null;
  const icon = target.locator("img");
  if ((await icon.count()) > 0) return (await icon.getAttribute("alt"))?.trim() ?? null;
  const label = target.locator(".tile-label");
  if ((await label.count()) > 0) return (await label.textContent())?.trim() ?? null;
  return null;
}

// The object id (rawcode) of the tile in a cell, read from the override panel.
async function objectIdAt(page: Page, coordinate: GridCoordinate): Promise<string> {
  const target = cell(page, coordinate);
  const label = await labelAt(page, coordinate);
  await target.click();
  const namePanel = page.locator(".ability-name");
  if (label) await expect(namePanel).toContainText(label);
  const idPanel = page.locator(".ability-id");
  await expect(idPanel).toBeVisible();
  return (await idPanel.textContent())?.trim() ?? "";
}

function fieldInSection(content: string, section: string, field: string): string | null {
  const lower = content.toLowerCase();
  const start = lower.indexOf(`[${section.toLowerCase()}]`);
  if (start === -1) return null;
  const end = lower.indexOf("[", start + 1);
  const chunk = end === -1 ? content.slice(start) : content.slice(start, end);
  const match = chunk.match(new RegExp(`${field}=([^\\r\\n]+)`, "i"));
  return match ? match[1].trim() : null;
}

async function applyDefaultTemplate(page: Page): Promise<void> {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();

  // Defensive: the suite seeds the onboarding-seen flag, but close the Help
  // dialog if it is open anyway so this spec also runs from a fresh profile.
  const helpDialog = page.getByRole("dialog");
  if (await helpDialog.isVisible()) {
    await helpDialog.getByRole("button", { name: "Got it, don't show this again" }).click();
    await expect(helpDialog).toBeHidden();
  }

  await page.locator('[aria-label="Browse layout templates"]').click();
  await page
    .locator(".template-card", { hasText: "Default" })
    .click();
  // Applying the template writes the new layout to localStorage and raises a
  // conflict prompt. The collisions are the resolver page's concern, so this
  // scenario does NOT cascade. Reload to land on the editor with the template
  // applied and un-resolved.
  await page.locator('[role="alertdialog"]').first().waitFor();
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();
}

async function selectArchmage(page: Page): Promise<void> {
  const archmage = page.locator(".unit-card").first();
  await expect(archmage).toContainText("Archmage");
  await archmage.click();
  await page.locator(".filled-tile").first().waitFor();
}

// Drag one command-card tile onto another. The grid uses a custom pointer-based
// drag (pointerdown/move/up with a start threshold and a cursor-following ghost),
// not native HTML5 drag. Playwright's `dragTo()` emits a single move, which is
// enough for Chromium but under-steps Firefox: the drag never crosses the start
// threshold, so the drop lands on the wrong cell. Driving the pointer manually with
// intermediate moves registers the drag identically in both engines.
async function dragCell(
  page: Page,
  from: GridCoordinate,
  to: GridCoordinate,
): Promise<void> {
  const sourceBox = await cell(page, from).boundingBox();
  const targetBox = await cell(page, to).boundingBox();
  if (!sourceBox || !targetBox) {
    throw new Error(
      `drag tiles missing: ${coordinateKey(from)} -> ${coordinateKey(to)}`,
    );
  }
  const sourceX = sourceBox.x + sourceBox.width / 2;
  const sourceY = sourceBox.y + sourceBox.height / 2;
  const targetX = targetBox.x + targetBox.width / 2;
  const targetY = targetBox.y + targetBox.height / 2;
  await page.mouse.move(sourceX, sourceY);
  await page.mouse.down();
  // One nudge past the drag-start threshold, then a few tracked moves onto the
  // target so the drop registers on the right cell. Kept minimal: every extra
  // move re-renders the WASM grid, which is the slow part in Firefox.
  await page.mouse.move(sourceX, sourceY + 8);
  await page.mouse.move(targetX, targetY, { steps: 4 });
  await page.mouse.up();
}

async function performSwaps(page: Page): Promise<void> {
  for (const swap of SWAP_SEQUENCE) {
    await dragCell(page, swap.from, swap.to);
  }
}

interface StuckCard {
  objectId: string;
  name: string;
  col: number;
  row: number;
}

// Read every STUCK card on the resolver page: its object id, name, and the
// single highlighted mini-grid cell (the position it is stuck at). Mini-grid
// cells render row-major, so the highlighted index maps to col = i % 4,
// row = floor(i / 4).
async function stuckCards(page: Page): Promise<StuckCard[]> {
  return page.locator(".resolve-move-row-stuck").evaluateAll((cards) =>
    cards.map((card) => {
      const objectId = card.querySelector(".conflict-object-id")?.textContent?.trim() ?? "";
      const name = card.querySelector(".resolve-move-name")?.textContent?.trim() ?? "";
      const cells = Array.from(card.querySelectorAll(".resolve-mini-grid .mini-cell"));
      const stuckIndex = cells.findIndex((miniCell) => miniCell.classList.contains("collision"));
      return { objectId, name, col: stuckIndex % 4, row: Math.floor(stuckIndex / 4) };
    }),
  );
}

test("rearranging the Archmage command card into the QWER/ASDF/YXCV layout", async ({ page }) => {
  // Eight sequential pointer-drags of the WASM grid; Firefox re-renders each move,
  // so this legitimately needs more than the default per-test budget.
  test.slow();
  await applyDefaultTemplate(page);
  await selectArchmage(page);

  // Sanity-check the starting layout matches the post-template screenshot: the
  // two middle-row cells are empty, everything else on the card is occupied.
  for (const empty of EMPTY_CELLS) {
    expect(await labelAt(page, empty)).toBeNull();
  }

  // Capture the object id and label of every tile we are about to move, keyed by
  // its starting cell. Reading the object id selects the tile, which is fine.
  const objectIdByStartCell = new Map<string, string>();
  const labelByStartCell = new Map<string, string>();
  for (const move of EXPECTED_MOVES) {
    const label = await labelAt(page, move.from);
    expect(label, `cell ${coordinateKey(move.from)} should be occupied before the rearrange`).not.toBeNull();
    labelByStartCell.set(coordinateKey(move.from), label!);
    objectIdByStartCell.set(coordinateKey(move.from), await objectIdAt(page, move.from));
  }

  // Surface the captured mapping in the report so it documents the real
  // object ids and start positions of each Archmage command-card tile.
  for (const move of EXPECTED_MOVES) {
    const key = coordinateKey(move.from);
    test.info().annotations.push({
      type: "tile",
      description: `${labelByStartCell.get(key)} (${objectIdByStartCell.get(key)}) ${key} -> ${coordinateKey(move.to)}`,
    });
  }

  // Perform the 8 swaps.
  await performSwaps(page);

  // The empty cells stay empty.
  for (const empty of EMPTY_CELLS) {
    expect(await labelAt(page, empty)).toBeNull();
  }

  // Each moved tile now sits in its target cell on the rendered grid.
  for (const move of EXPECTED_MOVES) {
    const expectedLabel = labelByStartCell.get(coordinateKey(move.from));
    expect(
      await labelAt(page, move.to),
      `expected ${expectedLabel} at ${coordinateKey(move.to)}`,
    ).toBe(expectedLabel);
  }

  // localStorage (the canonical source of truth) records the new positions: the
  // object id captured at each start cell now carries the target Buttonpos.
  const stored = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
  expect(stored).not.toBeNull();
  for (const move of EXPECTED_MOVES) {
    const objectId = objectIdByStartCell.get(coordinateKey(move.from))!;
    expect(
      fieldInSection(stored!, objectId, "Buttonpos"),
      `${objectId} should be at ${coordinateKey(move.to)} in localStorage`,
    ).toBe(coordinateKey(move.to));
  }
});

// The rearrange above moves the global system commands off the top row. CmdStop
// lands on 0,2, CmdHoldPos on 1,2, CmdPatrol on 2,2 and CmdMove on 3,2 — the
// whole bottom row of every unit's command card is now occupied by high-carrier
// pinned commands. On worker units the pinned build command (CmdBuild and its
// per-race siblings) defaults to 0,2, and the pinned Ancient root toggle (Aro1 /
// Aro2) sits on the bottom row too. These pinned slots collide with the moved
// commands, cascade rightward across the full bottom row, and overflow at 3,2.
//
// Before the fix this left 7 moves wedged at 3,2 with a STUCK badge, because the
// spill resolver refused to rehome pinned slots. The fix gives an overflowed
// pinned slot the same full-grid best-fit search as any other slot, so it finds
// a free cell elsewhere instead of giving up. The resolver must now report no
// stuck moves at all.
test("rearranged layout resolves with no stuck build/root commands on the resolver", async ({
  page,
}) => {
  test.slow();
  await applyDefaultTemplate(page);
  await selectArchmage(page);
  await performSwaps(page);

  await page.locator('[aria-label="Resolve conflicts"]').click();
  await page.locator('.resolve-page').waitFor();

  const cards = await stuckCards(page);
  expect(
    cards,
    `expected no stuck moves, got: ${cards.map((card) => `${card.objectId}@${card.col},${card.row}`).join(", ")}`,
  ).toEqual([]);
});

// After applying the cascade, the resolver reports "nothing to resolve". The
// collision page must agree: every cross-unit and intra-unit POSITION collision
// is gone. This used to fail for toggle abilities (Burrow, Bear/Crow form,
// Submerge, Robo-Goblin, Web, Call to Arms) because the cascade collapsed an
// ability's on-state and off-state into one node and only resolved one of them,
// leaving the other sitting on a moved command. Hotkey collisions are a separate
// concern (cleared by applying the grid) and are not asserted here.
test("applying the cascade clears every position collision, including toggle off-states", async ({
  page,
}) => {
  test.slow();
  await applyDefaultTemplate(page);
  await selectArchmage(page);
  await performSwaps(page);

  await page.locator('[aria-label="Resolve conflicts"]').click();
  await page.locator('.resolve-page').waitFor();
  await page.locator(".apply-button", { hasText: /apply/i }).click();
  await page.locator('[role="alertdialog"]').filter({ hasText: "Cascade applied" }).waitFor();

  await page.goto(`${APP}collisions?kind=positions`);
  // The breadcrumb bar is always present on the collisions page; the two-pane
  // `.positions-content` collapses to the all-clear state once the counts hit
  // zero (exactly what this test asserts), so wait on the always-present bar.
  await page.locator(".breadcrumbs").waitFor();

  // The cross- and intra-unit position tabs are picked by their visible label
  // text; their count child reports the live collision total.
  const crossCount = page
    .locator(".breadcrumbs button", { hasText: "Cross Collisions" })
    .locator(".breadcrumb-count");
  const intraCount = page
    .locator(".breadcrumbs button", { hasText: "Intra Collisions" })
    .locator(".breadcrumb-count");
  await expect(crossCount).toHaveText("0");
  await expect(intraCount).toHaveText("0");
});

// A non-morph toggle (Frost Armor, ACf2) keeps both states on one grid where
// only the off-state is drawn; the on-state lives in a separate dialog. The NEO
// (QWERTZ) template moves the grid position (Buttonpos) but not the dialog-only
// one (Unbuttonpos), so the hidden state used to drift and later haunt the
// cascade as an invisible blocker that blocked Heal. Normalizing the imported
// template must already pull the two positions together.
test("a non-morph toggle's two positions coincide after applying a template", async ({ page }) => {
  await page.goto(APP);
  await page.locator(".unit-card").first().waitFor();
  const helpDialog = page.getByRole("dialog");
  if (await helpDialog.isVisible()) {
    await helpDialog.getByRole("button", { name: "Got it, don't show this again" }).click();
    await expect(helpDialog).toBeHidden();
  }

  await page.locator('[aria-label="Browse layout templates"]').click();
  await page
    .locator(".template-card", { hasText: "NEO (QWERTZ)" })
    .click();
  await page.locator('[role="alertdialog"]').first().waitFor();

  const stored = await page.evaluate((key) => localStorage.getItem(key), LS_KEY);
  expect(stored).not.toBeNull();
  const buttonpos = fieldInSection(stored!, "ACf2", "Buttonpos");
  const unbuttonpos = fieldInSection(stored!, "ACf2", "Unbuttonpos");
  expect(buttonpos).not.toBeNull();
  expect(
    unbuttonpos,
    `Frost Armor on-state (${unbuttonpos}) must coincide with its off-state (${buttonpos})`,
  ).toBe(buttonpos);
});
