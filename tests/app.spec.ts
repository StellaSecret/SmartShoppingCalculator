import { test, expect } from '@playwright/test';

const APP_PATH = '/SmartShoppingCalculator/';

test.beforeEach(async ({ page }) => {
  await page.goto(APP_PATH);
  await page.evaluate(() => localStorage.clear());
  await page.reload();
});

test('loads the calculator with two default toilet-paper rolls', async ({ page }) => {
  await expect(page.locator('h1')).toContainText('smart shopping calculator');
  await expect(page.locator('.nav-tab')).toHaveCount(2);
  await expect(page.locator('.item-card')).toHaveCount(2);
});

test('default rolls produce a ranking with a best value', async ({ page }) => {
  await expect(page.locator('.rank-list .rank-item')).toHaveCount(2);
  await expect(page.locator('.tag-best')).toHaveText('best value');
});

test('switching to the protein page shows powder cards', async ({ page }) => {
  await page.getByRole('button', { name: 'protein powder' }).click();
  await expect(page.locator('.item-card')).toHaveCount(2);
  await expect(page.getByPlaceholder('e.g. 29.99').first()).toBeVisible();
  await expect(page.locator('.rank-list .rank-item')).toHaveCount(2);
});

test('editing a price updates the cards and keeps the ranking', async ({ page }) => {
  await page.getByPlaceholder('e.g. 1.99').first().fill('0.99');
  await expect(page.locator('.rank-list .rank-item')).toHaveCount(2);
  await expect(page.locator('.cpg').first()).toContainText('0.009');
});

test('language toggle switches the UI to French', async ({ page }) => {
  await page.getByRole('button', { name: 'FR' }).click();
  await expect(page.locator('.nav-tab').first()).toContainText('papier toilette');
});

test('currency select switches the unit label', async ({ page }) => {
  await page.locator('.currency-select').selectOption('$');
  await expect(page.locator('.rank-val').first()).toContainText('$');
});
