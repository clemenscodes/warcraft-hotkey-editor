import { chromium } from "@playwright/test";

const COMPILE_TIMEOUT = 10 * 60 * 1000;

export default async function globalSetup(): Promise<void> {
  const port = process.env["E2E_PORT"] ?? "8124";
  const baseUrl = process.env["BASE_URL"] ?? `http://localhost:${port}`;
  const noSandboxArgs = process.env["CI"]
    ? ["--no-sandbox", "--disable-setuid-sandbox", "--disable-dev-shm-usage", "--no-zygote"]
    : [];
  const browser = await chromium.launch({ args: noSandboxArgs });
  const page = await browser.newPage();
  await page.goto(`${baseUrl}/warcraft-hotkey-editor/`, { timeout: COMPILE_TIMEOUT });
  await page.locator(".unit-card").first().waitFor({ timeout: COMPILE_TIMEOUT });
  await browser.close();
}
