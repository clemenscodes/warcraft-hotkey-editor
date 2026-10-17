import { expect, test, type Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

// One unit per race, including the Eredar Sorcerer whose portrait came up blank.
const UNITS_BY_RACE = [
  { race: "human", unit: "hpea" },
  { race: "orc", unit: "ogru" },
  { race: "nightelf", unit: "earc" },
  { race: "undead", unit: "ugho" },
  { race: "neutral", unit: "ners" },
];

// Phone widths across the mobile band, down to the narrowest common phone.
const PHONE_WIDTHS = [320, 375, 414];

async function openUnit(page: Page, race: string, unit: string): Promise<void> {
  await page.goto(`${APP}?race=${race}&mode=melee,campaign&unit=${unit}`);
  await page.locator(".pager-card").first().waitFor();
  await page.waitForTimeout(1000);
}

// The images of the card the pager has aligned that are actually on screen, once
// each has finished loading or failed. Images in an offscreen carousel pane are
// lazy and never start loading, so they are left out. Returns the source of
// every visible image that failed.
async function brokenImagesOnAlignedCard(page: Page): Promise<string[]> {
  await page.waitForFunction(() =>
    [...document.querySelectorAll<HTMLImageElement>(".pager-card-host img")]
      .filter((image) => {
        const box = image.getBoundingClientRect();
        return box.width > 0 && box.bottom > 0 && box.right > 0
          && box.top < window.innerHeight && box.left < window.innerWidth;
      })
      .every((image) => image.complete),
  );
  return page.evaluate(() => {
    const pager = document.querySelector('section[aria-label="Mobile editor"]');
    if (!pager) {
      return ["no pager"];
    }
    const pagerTop = pager.getBoundingClientRect().top;
    let aligned: Element | null = null;
    let closestDistance = Number.POSITIVE_INFINITY;
    for (const card of pager.querySelectorAll(".pager-card-host")) {
      const distance = Math.abs(card.getBoundingClientRect().top - pagerTop);
      if (distance < closestDistance) {
        closestDistance = distance;
        aligned = card;
      }
    }
    const images = [...(aligned?.querySelectorAll("img") ?? [])];
    return images
      .filter((image) => {
        const box = image.getBoundingClientRect();
        return box.width > 0 && box.bottom > 0 && box.right > 0
          && box.top < window.innerHeight && box.left < window.innerWidth;
      })
      .filter((image) => image.naturalWidth === 0)
      .map((image) => image.getAttribute("src") ?? "(no src)");
  });
}

// Opens the detail dialog of the card the pager has aligned.
async function openAlignedUnitDetail(page: Page): Promise<void> {
  const trigger = await page.evaluate(() => {
    const pager = document.querySelector('section[aria-label="Mobile editor"]');
    const pagerTop = pager?.getBoundingClientRect().top ?? 0;
    for (const card of pager?.querySelectorAll(".pager-card-host") ?? []) {
      if (Math.abs(card.getBoundingClientRect().top - pagerTop) < 2) {
        const box = card.querySelector(".pager-card-detail-button")?.getBoundingClientRect();
        return box ? { x: box.x + box.width / 2, y: box.y + box.height / 2 } : null;
      }
    }
    return null;
  });
  expect(trigger).not.toBeNull();
  await page.mouse.click(trigger!.x, trigger!.y);
  await page.locator(".unit-detail-dialog-body").waitFor();
}

// The heroes with the most stat rows: attributes, and for the Demon Hunter an
// evasion row on top.
const DETAIL_UNITS = [
  { race: "human", unit: "Hamg" },
  { race: "nightelf", unit: "Edem" },
];

test.describe("Mobile search dialog", () => {
  test("opens ready to type and lists compact, untruncated results", async ({ page }) => {
    await openUnit(page, "human", "hpea");
    await page.locator('[aria-label="Search units"]').click();
    await page.locator(".search-dialog-body").waitFor();

    // The cursor is in the field, so typing filters straight away.
    await expect(page.locator(".search-dialog-body input")).toBeFocused();
    await page.keyboard.type("gru");
    await expect(page.locator(".search-dialog-body .unit-card").first()).toBeVisible();
    await page.screenshot({ path: test.info().outputPath("search.png") });

    const results = await page.evaluate(() => {
      const body = document.querySelector(".search-dialog-body");
      const list = body?.querySelector(".category-scroll");
      const leaves = [...(list?.querySelectorAll("*") ?? [])].filter(
        (element) => element.children.length === 0 && (element.textContent ?? "").trim(),
      );
      return {
        truncated: leaves
          .filter((element) => element.scrollWidth > element.clientWidth + 1)
          .map((element) => (element.textContent ?? "").trim()),
        fontSizes: [...new Set(leaves.map((element) => getComputedStyle(element).fontSize))],
      };
    });
    expect(results.truncated).toEqual([]);
    expect(results.fontSizes).toHaveLength(1);
  });
});

test.describe("Mobile visual integrity", () => {
  for (const { race, unit } of DETAIL_UNITS) {
    test(`the ${unit} detail dialog fits the screen without scrolling`, async ({ page }) => {
      await openUnit(page, race, unit);
      await openAlignedUnitDetail(page);
      await page.screenshot({ path: test.info().outputPath(`${unit}-detail.png`) });

      const overflow = await page.evaluate(() => {
        const body = document.querySelector(".unit-detail-dialog-body");
        return body ? body.scrollHeight - body.clientHeight : Number.POSITIVE_INFINITY;
      });
      expect(overflow).toBeLessThanOrEqual(0);

      const truncatedLabels = await page.evaluate(() =>
        [...document.querySelectorAll(".unit-detail-dialog-body .matchup-label, .unit-detail-dialog-body .stat-label, .unit-detail-dialog-body .regen-label")]
          .filter((label) => label.scrollWidth > label.clientWidth + 1)
          .map((label) => (label.textContent ?? "").trim()),
      );
      expect(truncatedLabels).toEqual([]);
    });
  }

  for (const { race, unit } of UNITS_BY_RACE) {
    test(`every image on the ${unit} card loads`, async ({ page }) => {
      await openUnit(page, race, unit);
      await page.screenshot({ path: test.info().outputPath(`${unit}.png`) });

      expect(await brokenImagesOnAlignedCard(page)).toEqual([]);
    });
  }

  for (const width of PHONE_WIDTHS) {
    test(`nothing overflows or truncates at ${width}px`, async ({ page }) => {
      await page.setViewportSize({ width, height: 800 });
      await openUnit(page, "human", "hpea");
      await page.screenshot({ path: test.info().outputPath(`width-${width}.png`) });

      const pageOverflow = await page.evaluate(
        () => document.documentElement.scrollWidth - window.innerWidth,
      );
      expect(pageOverflow).toBeLessThanOrEqual(0);

      const truncatedRaceTabs = await page.evaluate(() =>
        [...document.querySelectorAll(".race-tab")]
          .filter((tab) => tab.scrollWidth > tab.clientWidth + 1)
          .map((tab) => (tab.textContent ?? "").trim()),
      );
      expect(truncatedRaceTabs).toEqual([]);

      // The brand shares the header row with the toolbar, so none of its parts
      // may reach under the first toolbar button.
      const brandOverlap = await page.evaluate(() => {
        const brand = document.querySelector("header h1")?.closest("button");
        const toolbarButtons = [...document.querySelectorAll("header button")]
          .filter((button) => button !== brand && button.getBoundingClientRect().width > 0);
        if (!brand || toolbarButtons.length === 0) {
          return Number.POSITIVE_INFINITY;
        }
        const toolbarLeft = Math.min(
          ...toolbarButtons.map((button) => button.getBoundingClientRect().left),
        );
        const brandRight = Math.max(
          ...[...brand.querySelectorAll("*")]
            .map((part) => part.getBoundingClientRect())
            .filter((box) => box.width > 0)
            .map((box) => box.right),
        );
        return brandRight - toolbarLeft;
      });
      expect(brandOverlap).toBeLessThanOrEqual(0);
    });
  }
});
