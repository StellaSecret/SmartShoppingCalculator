import { test, expect } from '@playwright/test';
import { CalcPage } from './helpers';

test.describe('Edge Cases & Boundary Conditions', () => {

  test('negative price input — result is invalid (shown as "fill in all fields")', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.switchToProtein();

    const card = app.proCards().first();
    const priceInput = card.locator('input[placeholder*="29.99"]');
    await priceInput.fill('-5');
    await priceInput.dispatchEvent('input');

    // Negative price yields cpg <= 0 → treated as invalid
    await expect(card.locator('.card-result')).toContainText('fill in all fields');
  });

  test('very large price values do not crash the app', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.switchToProtein();

    await app.fillPowder(0, { price: '99999.99', servings: '100', protein: '30' });
    await page.waitForTimeout(100);

    // Should still render a number, not NaN or crash
    const result = await app.getProteinResult(0);
    expect(result).toMatch(/€\d+\.\d+/);
  });

  test('decimal servings (e.g. 33.5) are handled', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.switchToProtein();

    await app.fillPowder(0, { price: '30.00', servings: '33.5', protein: '25' });
    await page.waitForTimeout(100);

    const result = await app.getProteinResult(0);
    expect(result).toMatch(/€\d+\.\d+/);
  });

  test('toilet paper: tube heavier than roll → invalid result', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();

    // tube weight > total weight → net paper weight is 0 or negative
    await app.fillRoll(0, { price: '1.50', packs: '1', totalW: '10', tubeW: '50' });
    await page.waitForTimeout(100);

    await expect(app.tpCards().first().locator('.card-result')).toContainText('fill in all fields');
  });

  test('toilet paper: inner diameter larger than outer → invalid result', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.setTpMethod('diameter');

    await app.fillRoll(0, { price: '1.50', packs: '1', outer: '30', inner: '80', width: '100' });
    await page.waitForTimeout(100);

    await expect(app.tpCards().first().locator('.card-result')).toContainText('fill in all fields');
  });

  test('rank items do not exceed the number of valid rolls', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.addRoll();
    await app.addRoll(); // 4 rolls
    await page.waitForTimeout(100);

    const rankCount = await app.rankItems().count();
    const cardCount = await app.tpCards().count();
    expect(rankCount).toBeLessThanOrEqual(cardCount);
  });

  test('method switch resets the ranking label', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();

    await app.setTpMethod('sheets');
    await page.waitForTimeout(50);
    await expect(app.tpResults()).toContainText('cost per 100cm²');

    await app.setTpMethod('weight');
    await page.waitForTimeout(50);
    await expect(app.tpResults()).toContainText('cost per gram of paper');
  });

  test('protein: renaming a powder updates rank list name', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.switchToProtein();

    const nameInput = app.proCards().first().locator('input[placeholder="Name"]');
    await nameInput.fill('MyProtein');
    await nameInput.dispatchEvent('input');
    await page.waitForTimeout(50);

    await expect(app.proSummary()).toContainText('MyProtein');
  });

  test('toilet paper: renaming a roll updates rank list', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();

    const nameInput = app.tpCards().first().locator('input[placeholder="Name"]');
    await nameInput.fill('SuperRoll');
    await nameInput.dispatchEvent('input');
    await page.waitForTimeout(50);

    await expect(app.tpResults()).toContainText('SuperRoll');
  });
});


test.describe('Mobile Viewport', () => {
  // These run on the Pixel 7 project defined in playwright.config.ts

  test('all tab buttons are visible and tappable on mobile', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();

    await expect(page.locator('.tab-btn')).toHaveCount(4);
    for (const btn of await page.locator('.tab-btn').all()) {
      await expect(btn).toBeVisible();
    }
  });

  test('cards grid renders in 2 columns on narrow viewport', async ({ page }) => {
    // Pixel 7 is ~393px wide — CSS sets 1fr 1fr at ≤520px
    const app = new CalcPage(page);
    await app.goto();

    const grid = page.locator('#tp-grid');
    const gridStyle = await grid.evaluate(el => getComputedStyle(el).gridTemplateColumns);
    // Should have two columns (two measurements in the string)
    expect(gridStyle.split(' ').length).toBe(2);
  });

  test('nav tabs fit on screen without overflow', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();

    const navBox = await page.locator('.nav-tabs').boundingBox();
    const viewportWidth = page.viewportSize()!.width;
    expect(navBox!.width).toBeLessThanOrEqual(viewportWidth);
  });
});
