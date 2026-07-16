import { defineConfig, devices } from "@playwright/test";
import { existsSync, readdirSync } from "node:fs";
import { join } from "node:path";

const staticDir = process.env["STATIC_DIR"];
const staticBasePath = process.env["STATIC_BASE_PATH"] ?? "";
const serverScript = join(__dirname, "server.mjs");
const port = process.env["E2E_PORT"] ?? "8124";
const baseUrl = process.env["BASE_URL"] ?? `http://localhost:${port}`;

const onboardingSuppressedState = {
  cookies: [],
  origins: [
    {
      origin: baseUrl,
      localStorage: [
        { name: "warcraft-hotkey-editor.onboarding-seen", value: "true" },
      ],
    },
  ],
};

// The six responsive bands from crates/hotkey-editor/tailwind.css (see
// docs/COMPONENTS.md "Responsive bands"). They are DISJOINT width ranges, so a
// spec must run at a viewport that falls inside the band it targets. Each band's
// specs live under tests/<band>/ and run at the width below (chosen inside the
// band's range); a band with no specs yet generates no project.
interface Band {
  name: string;
  viewport: { width: number; height: number };
}

const bands: Band[] = [
  { name: "mobile", viewport: { width: 375, height: 812 } }, //   < 768
  { name: "tablet", viewport: { width: 900, height: 1200 } }, //  768–1279
  { name: "laptop", viewport: { width: 1600, height: 900 } }, //  1280–1919
  { name: "desktop", viewport: { width: 1920, height: 1080 } }, // 1920–2559
  { name: "qhd", viewport: { width: 2560, height: 1440 } }, //     2560–3839
  { name: "uhd", viewport: { width: 3840, height: 2160 } }, //     ≥ 3840
];

// Cross-browser is a second, independent axis: every band runs on every engine.
// The Nix dev shell (PLAYWRIGHT_BROWSERS_PATH, flake.nix) provisions all three.
const ciChromiumArgs = [
  "--no-sandbox",
  "--disable-setuid-sandbox",
  "--disable-dev-shm-usage",
  "--no-zygote",
];

interface Browser {
  name: string;
  use: Record<string, unknown>;
  // Per-engine timeout budget, sized for `workers: "50%"` (16 here). At that
  // concurrency the per-action cost is memory-bandwidth-bound: the app renders a
  // CQI-dense DOM (the resolve plan is ~46 move cards × ~66 nodes = 3227 nodes of
  // deeply-nested flex/grid) that scales super-linearly under load, so an action
  // that is ~300ms serial can exceed 2s at 16-wide — in EVERY engine, not just
  // WebKit (the domain/cascade itself is ~0.5ms; the cost is purely render).
  // A strict 1s action budget only holds at low worker counts. WebKit (WPE on
  // Linux) is the slowest renderer so it gets the widest budget. The real fix is
  // to cut the MoveCard node count; these budgets keep 16-wide green until then.
  timeout: number;
  actionTimeout: number;
  navigationTimeout: number;
}

const parallelBudget = { timeout: 45_000, actionTimeout: 15_000, navigationTimeout: 30_000 };
const webkitBudget = { timeout: 60_000, actionTimeout: 20_000, navigationTimeout: 40_000 };

const browsers: Browser[] = [
  {
    name: "chromium",
    use: {
      ...devices["Desktop Chrome"],
      launchOptions: { args: process.env["CI"] ? ciChromiumArgs : [] },
    },
    ...parallelBudget,
  },
  { name: "firefox", use: { ...devices["Desktop Firefox"] }, ...parallelBudget },
  { name: "webkit", use: { ...devices["Desktop Safari"] }, ...webkitBudget },
];

const testsRoot = join(__dirname, "tests");

function bandHasSpecs(band: string): boolean {
  const dir = join(testsRoot, band);
  if (!existsSync(dir)) {
    return false;
  }
  const entries = readdirSync(dir, { recursive: true });
  return entries.some((entry) => entry.toString().endsWith(".spec.ts"));
}

const projects = bands
  .filter((band) => bandHasSpecs(band.name))
  .flatMap((band) =>
    browsers.map((browser) => ({
      name: `${band.name}-${browser.name}`,
      testDir: join("./tests", band.name),
      timeout: browser.timeout,
      use: {
        ...browser.use,
        viewport: band.viewport,
        actionTimeout: browser.actionTimeout,
        navigationTimeout: browser.navigationTimeout,
      },
    })),
  );

export default defineConfig({
  globalSetup: "./global-setup.ts",
  testDir: "./tests",
  outputDir: "./test-results",
  fullyParallel: true,
  forbidOnly: !!process.env["CI"],
  retries: 0,
  // 8 workers is the highest count at which the full 3-engine matrix runs green
  // on this app (verified 423/423). At "50%" (16 here) the 5.4MB-wasm compile and
  // the resolve view's ~3227-node render saturate memory bandwidth so hard that
  // tail latencies hit multi-second hangs in every engine — no timeout absorbs it.
  // Reaching a green 16 needs the resolve MoveCard node count cut (a UX change),
  // not a config tweak; until then 8 is the stable ceiling.
  workers: 8,
  // Per-project `timeout`/`use.actionTimeout` (set in `projects` above) are the
  // real budget knobs; these are fallbacks. `expect.timeout` is global-only in
  // Playwright, so it is set wide enough for the parallel load — it only ever
  // delays a *failing* assertion, so green runs are unaffected.
  timeout: 5_000,
  expect: { timeout: 15_000 },
  reporter: [
    ["list"],
    ["html", { open: "never", outputFolder: "./playwright-report" }],
  ],
  use: {
    baseURL: baseUrl,
    storageState: onboardingSuppressedState,
    actionTimeout: 1000,
    navigationTimeout: 5_000,
    trace: "on-first-retry",
  },
  projects,
  webServer: staticDir
    ? {
        command: `node ${serverScript} ${staticDir} ${port} ${staticBasePath}`,
        port: Number(port),
        timeout: 5_000,
        // e2e owns its own server on `port` (8124), isolated from the 8123 dev
        // port, so a running dev server never collides with the suite. Reuse an
        // existing server on this port if one is somehow up; in CI nothing holds
        // it, so the static server starts fresh.
        reuseExistingServer: true,
        stdout: "ignore",
        stderr: "pipe",
      }
    : {
        command: `dx serve --package hotkey-editor --platform web --port ${port}`,
        port: Number(port),
        timeout: 10 * 60 * 1000,
        reuseExistingServer: true,
        stdout: "ignore",
        stderr: "ignore",
      },
});
