import { expect, test, type Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

// Below 768px the editor page mounts the pager instead of the aside, and nothing
// in that tree calls select_race or select_mode. The search dialog is therefore
// the ONLY way a phone can change race, change mode, or find a unit at all —
// without it you swipe through every unit in the game, in one flat list, until
// you cross a race boundary. These tests guard that navigation surface.

async function openSearchDialog(page: Page): Promise<void> {
  await page.goto(APP);
  await page.locator(".pager-card").first().waitFor();
  await page.locator('button[aria-label="Search units"]').click();
  await page.locator(".search-dialog-body").waitFor();
  // The body mounts before its rows do. Clicking a chip that is still arriving
  // hits whatever the layout settles into, not the chip that was asked for.
  await page.locator(".race-chip-row button").first().waitFor();
  await page.locator(".mode-chip-row button").first().waitFor();
  await page.locator(".unit-card").first().waitFor();
}

test.describe("Mobile search dialog", () => {
  test("is the phone's only race switch, and picking a race jumps to it", async ({
    page,
  }) => {
    await openSearchDialog(page);
    await expect(page).toHaveURL(/race=human/);

    await page.locator(".race-chip-row button").filter({ hasText: /^Orc$/i }).click();

    // select_race resolves the race's default unit and pushes it, so the chip is
    // the jump, not just a filter.
    await expect(page).toHaveURL(/race=orc/);
    await expect(page).toHaveURL(/unit=/);
  });

  // This was the very first requirement and the easiest to quietly get wrong:
  // melee and campaign are two independent filters, not one either/or. A unit
  // that only exists in one of them must be findable without first knowing which
  // half of the game it lives in.
  test("melee and campaign are independent, not a switch", async ({ page }) => {
    await openSearchDialog(page);
    await expect(page).toHaveURL(/mode=melee/);
    const meleeOnly = await page.locator(".unit-card").count();

    await page
      .locator(".mode-chip-row button")
      .filter({ hasText: /campaign/i })
      .click();

    // Campaign is added to melee, not swapped for it.
    await expect(page).toHaveURL(/mode=melee,campaign/);
    await expect(page.locator(".mode-chip-row .active-toggle-button")).toHaveCount(2);
    await expect
      .poll(() => page.locator(".unit-card").count())
      .toBeGreaterThan(meleeOnly);
  });

  test("a mode can be turned back off while the other stays on", async ({
    page,
  }) => {
    await openSearchDialog(page);
    const campaignChip = page
      .locator(".mode-chip-row button")
      .filter({ hasText: /campaign/i });

    await campaignChip.click();
    await expect(page).toHaveURL(/mode=melee,campaign/);

    await page
      .locator(".mode-chip-row button")
      .filter({ hasText: /melee/i })
      .click();
    await expect(page).toHaveURL(/mode=campaign(&|$)/);
    await expect(page.locator(".mode-chip-row .active-toggle-button")).toHaveCount(1);
  });

  test("search spans every race, not just the selected one", async ({ page }) => {
    await openSearchDialog(page);
    await expect(page).toHaveURL(/race=human/);

    await page.locator(".unit-list-search-input").fill("demon");

    // The Demon Hunter is a Night Elf. The domain drops the race filter while a
    // search runs, so it must surface even under the Human theme.
    await expect(
      page.locator(".unit-card").filter({ hasText: "Demon Hunter" }),
    ).toBeVisible();
  });

  test("the race chips report themselves inactive while a search runs", async ({
    page,
  }) => {
    await openSearchDialog(page);
    await expect(page.locator(".race-chip-row .active-toggle-button")).toHaveCount(1);

    await page.locator(".unit-list-search-input").fill("demon");
    await page.locator(".unit-card").filter({ hasText: "Demon Hunter" }).waitFor();

    // A search ignores the race, so no chip may claim to be filtering by one.
    await expect(page.locator(".race-chip-row .active-toggle-button")).toHaveCount(0);
  });

  // Mobile must not be a reduced edition of the desktop: every filter the aside
  // offers has to be reachable here too, or the phone silently cannot express
  // things the desktop can.
  test("offers every filter the desktop aside offers", async ({ page }) => {
    await openSearchDialog(page);

    // Search by unit name xor by ability. It rides inside the search component
    // as a dropdown, because it is part of the question being typed rather than a
    // filter of its own.
    await expect(page.locator(".search-scope-picker")).toBeVisible();
    await expect(
      page.locator(".closed-search-scope-trigger, .open-search-scope-trigger"),
    ).toBeVisible();

    // Five races in their own row.
    await expect(page.locator(".race-chip-row button")).toHaveCount(5);

    // The four independent toggles share one row, because they are one kind of
    // control: both modes, plus the two catalog toggles.
    await expect(page.locator(".mode-chip-row button")).toHaveCount(4);
  });

  test("searching by ability finds units that carry it", async ({ page }) => {
    await openSearchDialog(page);
    await page.locator(".closed-search-scope-trigger").click();
    await page
      .locator(".active-search-scope-option, .idle-search-scope-option")
      .filter({ hasText: /ability/i })
      .click();

    await page.locator(".unit-list-search-input").fill("burrow");
    await expect(page).toHaveURL(/search_query=burrow/);

    // Nothing is named "burrow" — these units only match through the ability they
    // carry, so a hit proves the toggle actually switched the search field.
    await expect(page.locator(".unit-card").first()).toBeVisible();
  });

  test("listing every variant widens the results", async ({ page }) => {
    await openSearchDialog(page);

    // Human melee is the one race/mode pair of ten where expanding variants
    // changes nothing (33 either way), so testing the default would prove the
    // toggle works while it quietly did not. Orc melee goes 34 -> 39.
    await page.locator(".race-chip-row button").filter({ hasText: /^Orc$/i }).click();
    await expect(page).toHaveURL(/race=orc/);
    await page.locator(".unit-card").first().waitFor();
    const collapsed = await page.locator(".unit-card").count();

    await page
      .locator(".mode-chip-row button")
      .filter({ hasText: /tiers/i })
      .click();

    await expect
      .poll(() => page.locator(".unit-card").count())
      .toBeGreaterThan(collapsed);
  });

  test("results stack vertically and never scroll the dialog sideways", async ({
    page,
  }) => {
    await openSearchDialog(page);
    const cards = page.locator(".unit-card");
    await cards.first().waitFor();

    // The list is shared with the desktop aside, whose tablet band turns it into
    // a horizontal rail. A dialog wants the plain vertical list, so the rail must
    // not follow it here.
    const boxes = await cards.evaluateAll((nodes) =>
      nodes.slice(0, 3).map((node) => {
        const rect = node.getBoundingClientRect();
        return { x: Math.round(rect.x), y: Math.round(rect.y) };
      }),
    );
    expect(boxes.length).toBeGreaterThan(1);
    expect(boxes[1].x).toBe(boxes[0].x);
    expect(boxes[1].y).toBeGreaterThan(boxes[0].y);

    const overflows = await page.evaluate(() => {
      const body = document.querySelector(".search-dialog-body");
      if (!body) return true;
      return body.scrollWidth > body.clientWidth + 1;
    });
    expect(overflows).toBe(false);
  });

  // Changing the URL is not the job — bringing the card on screen is. The pager
  // stays mounted behind the dialog and follows the navigation by scrolling, so
  // closing the dialog has to leave you ON the unit you asked for, not wherever
  // the pager happened to be.
  test("picking a race leaves the pager on that race's first unit", async ({
    page,
  }) => {
    await openSearchDialog(page);
    await page.locator(".race-chip-row button").filter({ hasText: /^Orc$/i }).click();
    await expect(page).toHaveURL(/race=orc/);

    const target = await page.evaluate(
      () => new URL(location.href).searchParams.get("unit") ?? "",
    );
    expect(target).not.toBe("");

    await page.locator(".dialog-close").click();
    await expect(page.locator(".search-dialog-body")).toHaveCount(0);

    // The card filling the pager's viewport must be the unit the race switch
    // resolved, not a neighbour the scroll settled on.
    await expect
      .poll(async () =>
        page.evaluate(() => {
          const section = document.querySelector(".mobile-editor");
          if (!section) return null;
          const middle = section.clientHeight / 2;
          const cards = [...document.querySelectorAll(".pager-card")];
          const onScreen = cards.find((card) => {
            const rect = card.getBoundingClientRect();
            return rect.top <= middle && rect.bottom >= middle;
          });
          return onScreen?.querySelector(".pager-card-id")?.textContent?.trim() ?? null;
        }),
      )
      .toBe(target);
  });

  test("picking a result opens that unit", async ({ page }) => {
    await openSearchDialog(page);
    await page.locator(".unit-list-search-input").fill("demon");

    // The query is debounced into the URL, and the list re-renders when it lands.
    // Clicking before it settles clicks whatever card the re-render puts under
    // the pointer, not the one that was matched.
    await expect(page).toHaveURL(/search_query=demon/);
    const hit = page.locator(".unit-card").filter({ hasText: "Demon Hunter" });
    await hit.waitFor();

    await hit.click();

    // open_unit takes the race and the mode from the unit itself, so a cross-race
    // hit lands under its own theme rather than stranding under Human.
    await expect(page).toHaveURL(/unit=Edem/i);
    await expect(page).toHaveURL(/race=nightelf/);
  });
});
