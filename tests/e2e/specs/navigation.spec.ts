import { test, expect } from '@playwright/test';
import { CalcPage } from './helpers';

test.describe('Navigation & Dark Mode', () => {

  test('page loads with toilet paper tab active by default', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();

    await expect(page.locator('#page-tp')).toHaveClass(/visible/);
    await expect(page.locator('#page-pro')).not.toHaveClass(/visible/);
    await expect(page.locator('button.nav-tab.active')).toContainText('toilet paper');
  });

  test('switching to protein powder tab shows correct page', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.switchToProtein();

    await expect(page.locator('#page-pro')).toHaveClass(/visible/);
    await expect(page.locator('#page-tp')).not.toHaveClass(/visible/);
    await expect(page.locator('button.nav-tab.active')).toContainText('protein powder');
  });

  test('switching back to toilet paper tab restores view', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.switchToProtein();
    await app.switchToToiletPaper();

    await expect(page.locator('#page-tp')).toHaveClass(/visible/);
    await expect(page.locator('button.nav-tab.active')).toContainText('toilet paper');
  });

  test('dark mode toggle adds dark class to body', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();

    // Start from known light state
    await page.evaluate(() => {
      document.body.classList.remove('dark');
      localStorage.setItem('theme', 'light');
    });

    await app.darkBtn.click();
    expect(await app.isDark()).toBe(true);
    await expect(app.darkBtn).toContainText('light');
  });

  test('dark mode toggle is reversible', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();

    await page.evaluate(() => {
      document.body.classList.remove('dark');
      localStorage.setItem('theme', 'light');
    });

    await app.darkBtn.click(); // → dark
    await app.darkBtn.click(); // → light
    expect(await app.isDark()).toBe(false);
    await expect(app.darkBtn).toContainText('dark');
  });

  test('dark mode preference is saved to localStorage', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await page.evaluate(() => {
      document.body.classList.remove('dark');
      localStorage.setItem('theme', 'light');
    });
    await app.darkBtn.click();

    const saved = await page.evaluate(() => localStorage.getItem('theme'));
    expect(saved).toBe('dark');
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
});
