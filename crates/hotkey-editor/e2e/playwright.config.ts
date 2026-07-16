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
}

const browsers: Browser[] = [
  {
    name: "chromium",
    use: {
      ...devices["Desktop Chrome"],
      launchOptions: { args: process.env["CI"] ? ciChromiumArgs : [] },
    },
  },
  { name: "firefox", use: { ...devices["Desktop Firefox"] } },
  { name: "webkit", use: { ...devices["Desktop Safari"] } },
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
      use: { ...browser.use, viewport: band.viewport },
    })),
  );

export default defineConfig({
  globalSetup: "./global-setup.ts",
  testDir: "./tests",
  outputDir: "./test-results",
  fullyParallel: true,
  forbidOnly: !!process.env["CI"],
  retries: 0,
  workers: 4,
  timeout: 5_000,
  expect: { timeout: 1_000 },
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
