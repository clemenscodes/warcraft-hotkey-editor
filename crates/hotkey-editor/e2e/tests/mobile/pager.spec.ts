import { expect, test, type Page } from "@playwright/test";

const APP = "/warcraft-hotkey-editor/";

// The pager snaps each card's top to its own top, so the card on screen is the
// one whose top sits closest to the pager's top.
async function alignedUnit(page: Page): Promise<string | null> {
  return page.evaluate(() => {
    const pager = document.querySelector('section[aria-label="Mobile editor"]');
    if (!pager) {
      return null;
    }
    const pagerTop = pager.getBoundingClientRect().top;
    let closest: Element | null = null;
    let closestDistance = Number.POSITIVE_INFINITY;
    for (const card of pager.querySelectorAll(".pager-card-host")) {
      const distance = Math.abs(card.getBoundingClientRect().top - pagerTop);
      if (distance < closestDistance) {
        closestDistance = distance;
        closest = card;
      }
    }
    return closest?.querySelector(".pager-card-id")?.textContent?.trim() ?? null;
  });
}

function urlUnit(page: Page): string | null {
  return new URL(page.url()).searchParams.get("unit");
}

// Every unit the app writes into the URL from now on, in order, so a test can
// prove the pager went straight to its target instead of walking through others.
async function recordNavigatedUnits(page: Page): Promise<void> {
  await page.evaluate(() => {
    const recorded: string[] = [];
    (window as unknown as { navigatedUnits: string[] }).navigatedUnits = recorded;
    for (const method of ["pushState", "replaceState"] as const) {
      const original = history[method].bind(history);
      history[method] = (data: unknown, unused: string, url?: string | URL | null) => {
        if (url) {
          const unit = new URL(url, location.href).searchParams.get("unit");
          if (unit && recorded[recorded.length - 1] !== unit) {
            recorded.push(unit);
          }
        }
        original(data, unused, url);
      };
    }
  });
}

async function navigatedUnits(page: Page): Promise<string[]> {
  return page.evaluate(
    () => (window as unknown as { navigatedUnits: string[] }).navigatedUnits,
  );
}

async function openUnit(page: Page, unitId: string): Promise<void> {
  await page.goto(`${APP}?race=human&mode=melee&unit=${unitId}`);
  await page.locator(".pager-card").first().waitFor();
}

test.describe("Mobile unit pager", () => {
  test("opens on the unit in the URL and stays there", async ({ page }) => {
    await openUnit(page, "hpea");
    await page.waitForTimeout(2000);

    expect(urlUnit(page)).toBe("hpea");
    expect(await alignedUnit(page)).toBe("hpea");
  });

  test("a race tab jumps straight to that race's first unit", async ({ page }) => {
    await openUnit(page, "hpea");
    await page.waitForTimeout(1000);
    await recordNavigatedUnits(page);

    await page.locator(".orc-race-tab .race-tab").click();
    await expect(page).toHaveURL(/race=orc/);
    await page.waitForTimeout(2000);

    const units = await navigatedUnits(page);
    expect(units).toHaveLength(1);
    expect(urlUnit(page)).toBe(units[0]);
    expect(await alignedUnit(page)).toBe(units[0]);
  });

  test("swiping past one card advances exactly one unit", async ({ page }) => {
    await openUnit(page, "hpea");
    await page.waitForTimeout(1000);

    const swipe = await page.evaluate(() => {
      const pager = document.querySelector('section[aria-label="Mobile editor"]');
      const cards = [...(pager?.querySelectorAll(".pager-card-host") ?? [])];
      const current = cards.find((card) =>
        card.querySelector(".pager-card-id")?.textContent?.trim() === "hpea",
      );
      const next = current ? cards[cards.indexOf(current) + 1] : undefined;
      if (!pager || !current) {
        return null;
      }
      const box = pager.getBoundingClientRect();
      return {
        nextUnit: next?.querySelector(".pager-card-id")?.textContent?.trim() ?? null,
        cardHeight: (current as HTMLElement).offsetHeight,
        pointerX: box.left + box.width / 2,
        pointerY: box.top + box.height / 2,
      };
    });

    expect(swipe).not.toBeNull();
    // A wheel turns one card on every engine, along the same spring a swipe
    // ends in, so it proves the settle without driving touch input.
    await page.mouse.move(swipe!.pointerX, swipe!.pointerY);
    await page.mouse.wheel(0, swipe!.cardHeight);
    const nextUnit = swipe!.nextUnit;

    expect(nextUnit).not.toBeNull();
    await expect.poll(() => urlUnit(page)).toBe(nextUnit);
    expect(await alignedUnit(page)).toBe(nextUnit);
  });

  // A swipe that starts while the previous card is still snapping interrupts
  // that snap, so the pager never settles between quick swipes. Every swipe must
  // still count. Real touch input that interrupts a snap only exists through the
  // Chromium DevTools protocol, so this runs on Chromium alone.
  test("quick swipes that interrupt each other's snap all count", async ({
    page,
    browserName,
  }) => {
    test.skip(browserName !== "chromium", "touch input is driven through CDP");
    const devtools = await page.context().newCDPSession(page);
    await devtools.send("Emulation.setTouchEmulationEnabled", {
      enabled: true,
      maxTouchPoints: 1,
    });
    const viewport = page.viewportSize();
    const fingerX = (viewport?.width ?? 375) / 2;
    const swipeUp = async (): Promise<void> => {
      const fromY = 650;
      const toY = 300;
      const steps = 6;
      await devtools.send("Input.dispatchTouchEvent", {
        type: "touchStart",
        touchPoints: [{ x: fingerX, y: fromY }],
      });
      for (let step = 1; step <= steps; step++) {
        const y = fromY + ((toY - fromY) * step) / steps;
        await devtools.send("Input.dispatchTouchEvent", {
          type: "touchMove",
          touchPoints: [{ x: fingerX, y }],
        });
        await page.waitForTimeout(16);
      }
      await devtools.send("Input.dispatchTouchEvent", {
        type: "touchEnd",
        touchPoints: [],
      });
    };

    await openUnit(page, "hpea");
    await page.waitForTimeout(1000);
    const roster = await page.evaluate(() =>
      [...document.querySelectorAll(".pager-card-id")].map((element) =>
        (element.textContent ?? "").trim(),
      ),
    );
    const expectedUnit = roster[roster.indexOf("hpea") + 3];

    for (let swipe = 0; swipe < 3; swipe++) {
      await swipeUp();
      await page.waitForTimeout(120);
    }

    await expect.poll(() => urlUnit(page)).toBe(expectedUnit);
    expect(await alignedUnit(page)).toBe(expectedUnit);
  });

  // The grids sit in a sideways carousel, its own scroll container, so the
  // touch-action that leaves vertical swipes to the pager must hold inside it
  // too; otherwise the browser takes the swipe and the card never moves.
  test("a swipe that starts on a grid tile turns the card", async ({ page, browserName }) => {
    test.skip(browserName !== "chromium", "touch input is driven through CDP");
    const devtools = await page.context().newCDPSession(page);
    await devtools.send("Emulation.setTouchEmulationEnabled", {
      enabled: true,
      maxTouchPoints: 1,
    });
    await openUnit(page, "hpea");
    await page.waitForTimeout(1000);
    const start = await page.evaluate(() => {
      const pager = document.querySelector('section[aria-label="Mobile editor"]');
      const pagerTop = pager?.getBoundingClientRect().top ?? 0;
      const cards = [...(pager?.querySelectorAll(".pager-card-host") ?? [])];
      const aligned = cards.find((card) => Math.abs(card.getBoundingClientRect().top - pagerTop) < 2);
      const next = aligned ? cards[cards.indexOf(aligned) + 1] : undefined;
      const tile = aligned?.querySelector(".grid-editor-tile")?.getBoundingClientRect();
      return tile
        ? {
            x: tile.x + tile.width / 2,
            y: tile.y + tile.height / 2,
            nextUnit: next?.querySelector(".pager-card-id")?.textContent?.trim() ?? null,
          }
        : null;
    });
    expect(start).not.toBeNull();
    await devtools.send("Input.dispatchTouchEvent", {
      type: "touchStart",
      touchPoints: [{ x: start!.x, y: start!.y }],
    });
    for (let step = 1; step <= 12; step++) {
      await devtools.send("Input.dispatchTouchEvent", {
        type: "touchMove",
        touchPoints: [{ x: start!.x, y: start!.y - 25 * step }],
      });
      await page.waitForTimeout(16);
    }
    await devtools.send("Input.dispatchTouchEvent", { type: "touchEnd", touchPoints: [] });

    await expect.poll(() => urlUnit(page)).toBe(start!.nextUnit);
  });
});
