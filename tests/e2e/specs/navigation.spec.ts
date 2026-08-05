import { test, expect } from '@playwright/test';
import { CalcPage } from './helpers';

test.describe('Navigation & Dark Mode', () => {

  test('page loads with toilet paper tab active by default', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();

    // dioxus renders only the active page — TP page is identifiable by its method tabs
    await expect(page.locator('.method-tabs')).toBeVisible();
    await expect(page.locator('input[placeholder="e.g. 29.99"]')).toHaveCount(0);
    await expect(page.locator('button.nav-tab.active')).toContainText('toilet paper');
  });

  test('switching to protein powder tab shows correct page', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.switchToProtein();

    await expect(page.locator('input[placeholder="e.g. 29.99"]')).toHaveCount(2);
    await expect(page.locator('.method-tabs')).toHaveCount(0);
    await expect(page.locator('button.nav-tab.active')).toContainText('protein powder');
  });

  test('switching back to toilet paper tab restores view', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.switchToProtein();
    await app.switchToToiletPaper();

    await expect(page.locator('.method-tabs')).toBeVisible();
    await expect(page.locator('button.nav-tab.active')).toContainText('toilet paper');
  });

  test('dark mode toggle adds dark class to app', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();

    // Start from known light state
    await app.resetLight();

    await app.darkBtn.click();
    expect(await app.isDark()).toBe(true);
    await expect(app.darkBtn).toContainText('light');
  });

  test('dark mode toggle is reversible', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();

    await app.resetLight();

    await app.darkBtn.click(); // → dark
    await app.darkBtn.click(); // → light
    expect(await app.isDark()).toBe(false);
    await expect(app.darkBtn).toContainText('dark');
  });

  test('dark mode preference is saved to localStorage', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.resetLight();
    await app.darkBtn.click();

    const saved = await page.evaluate(() => localStorage.getItem('theme'));
    expect(JSON.parse(saved!)).toBe('dark');
  });

  test('page title is correct', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await expect(page).toHaveTitle(/smart shopping calculator/i);
  });

  test('footer shows privacy notice', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await expect(page.locator('footer')).toContainText('no data is sent anywhere');
  });

  // ── Localisation (added for the Rust build) ────────────────────────────
  test('language toggle switches the UI to French', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await page.locator('.header-controls .pill-btn').nth(0).click();
    await expect(page.locator('.nav-tab').first()).toContainText('papier toilette');
  });

  test('currency select switches the unit label', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await page.locator('.currency-select').selectOption('$');
    await expect(page.locator('.rank-val').first()).toContainText('$');
  });
});
