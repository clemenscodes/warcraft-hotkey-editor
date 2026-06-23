import { defineConfig, devices } from "@playwright/test";
import { join } from "node:path";

const staticDir = process.env["STATIC_DIR"];
const staticBasePath = process.env["STATIC_BASE_PATH"] ?? "";
const serverScript = join(__dirname, "e2e", "server.mjs");

const baseUrl = process.env["BASE_URL"] ?? "http://localhost:8123";

// Suppress the first-visit onboarding Help dialog for the whole suite by
// seeding its "already seen" flag. The dialog is correct production behavior
// for fresh visitors, but as a modal overlay it would intercept the clicks
// every interacting test relies on. Tests that exercise onboarding itself
// override this with an empty storageState. Seeded against the same origin the
// suite navigates to, so a BASE_URL override stays consistent.
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

export default defineConfig({
  globalSetup: "./e2e/global-setup.ts",
  testDir: "./e2e/tests",
  outputDir: "./dist/test-results",
  fullyParallel: true,
  forbidOnly: !!process.env["CI"],
  retries: process.env["CI"] ? 2 : 0,
  workers: process.env["CI"] ? 1 : 4,
  timeout: 10_000,
  expect: { timeout: process.env["CI"] ? 1000 : 5000 },
  reporter: [
    ["list"],
    ["html", { open: "never", outputFolder: "./dist/playwright-report" }],
  ],
  use: {
    baseURL: baseUrl,
    storageState: onboardingSuppressedState,
    actionTimeout: process.env["CI"] ? 1000 : 5000,
    navigationTimeout: 10_000,
    trace: "on-first-retry",
  },
  projects: [
    {
      name: "chromium",
      use: {
        ...devices["Desktop Chrome"],
        viewport: { width: 1600, height: 900 },
        launchOptions: {
          args: process.env["CI"]
            ? ["--no-sandbox", "--disable-setuid-sandbox", "--disable-dev-shm-usage", "--no-zygote"]
            : [],
        },
      },
    },
  ],
  webServer: staticDir
    ? {
        command: `node ${serverScript} ${staticDir} 8123 ${staticBasePath}`,
        port: 8123,
        timeout: 10_000,
        // Reuse a server already on 8123 (e.g. a running dev server) so the CI
        // playwright flow can be exercised locally without a port conflict. In
        // real CI nothing else holds 8123, so the static server still starts.
        reuseExistingServer: true,
        stdout: "ignore",
        stderr: "pipe",
      }
    : {
        command: "dx serve --package hotkey-editor --platform web --port 8123",
        port: 8123,
        timeout: 10 * 60 * 1000,
        reuseExistingServer: true,
        stdout: "ignore",
        stderr: "ignore",
      },
});
