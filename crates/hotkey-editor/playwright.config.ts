import { defineConfig, devices } from "@playwright/test";
import { join } from "node:path";

const staticDir = process.env["STATIC_DIR"];
const staticBasePath = process.env["STATIC_BASE_PATH"] ?? "";
const serverScript = join(__dirname, "e2e", "server.mjs");

export default defineConfig({
  globalSetup: "./e2e/global-setup.ts",
  testDir: "./e2e/tests",
  outputDir: "./dist/test-results",
  fullyParallel: true,
  forbidOnly: !!process.env["CI"],
  retries: process.env["CI"] ? 2 : 0,
  workers: process.env["CI"] ? 1 : undefined,
  timeout: 10_000,
  expect: { timeout: 1000 },
  reporter: [
    ["list"],
    ["html", { open: "never", outputFolder: "./dist/playwright-report" }],
  ],
  use: {
    baseURL: process.env["BASE_URL"] ?? "http://localhost:8080",
    actionTimeout: 1000,
    navigationTimeout: 10_000,
    trace: "on-first-retry",
  },
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"], viewport: { width: 1600, height: 900 } },
    },
  ],
  webServer: staticDir
    ? {
        command: `node ${serverScript} ${staticDir} 8080 ${staticBasePath}`,
        port: 8080,
        timeout: 10_000,
        stdout: "ignore",
        stderr: "pipe",
      }
    : {
        command: "dx serve --package hotkey-editor --platform web",
        port: 8080,
        timeout: 10 * 60 * 1000,
        reuseExistingServer: true,
        stdout: "ignore",
        stderr: "ignore",
      },
});
