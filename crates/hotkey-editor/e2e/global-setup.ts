import { chromium } from "@playwright/test";

const COMPILE_TIMEOUT = 10 * 60 * 1000;

export default async function globalSetup(): Promise<void> {
  const baseUrl = process.env["BASE_URL"] ?? "http://localhost:8080";
  const browser = await chromium.launch();
  const page = await browser.newPage();
  await page.goto(`${baseUrl}/warcraft-hotkey-editor/`, { timeout: COMPILE_TIMEOUT });
  await page.locator(".unit-card").first().waitFor({ timeout: COMPILE_TIMEOUT });
  await browser.close();
}
