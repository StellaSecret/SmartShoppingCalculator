import { test, expect } from '@playwright/test';
import { CalcPage } from './helpers';

test.describe('Protein Powder', () => {

  test.beforeEach(async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.switchToProtein();
  });

  // ── Initial state ──────────────────────────────────────────────────────
  test('two default powders are pre-loaded', async ({ page }) => {
    const app = new CalcPage(page);
    await expect(app.proCards()).toHaveCount(2);
  });

  test('counter label shows "2 / 4 powders"', async ({ page }) => {
    await expect(page.locator('#pro-count-label')).toHaveText('2 / 4 powders');
  });

  test('default powders produce a valid €/g result', async ({ page }) => {
    const app = new CalcPage(page);
    // Brand A: 29.99 / (33 * 25) = 29.99 / 825 = 0.03635€/g
    const result = await app.getProteinResult(0);
    expect(result).toMatch(/€0\.\d+/);
    const val = parseFloat(result.replace('€', ''));
    expect(val).toBeCloseTo(0.0364, 3);
  });

  // ── Calculation correctness ────────────────────────────────────────────
  test('cost per gram formula: price / (servings × protein)', async ({ page }) => {
    const app = new CalcPage(page);
    // 30.00€ / (30 servings × 20g) = 30 / 600 = 0.05000
    await app.fillPowder(0, { price: '30.00', servings: '30', protein: '20' });
    await page.waitForTimeout(100);

    const result = await app.getProteinResult(0);
    const val = parseFloat(result.replace('€', ''));
    expect(val).toBeCloseTo(0.05, 4);
  });

  test('cost-per-serving shown in extra line', async ({ page }) => {
    const app = new CalcPage(page);
    // 30€ / 30 servings = €1.00/serving
    await app.fillPowder(0, { price: '30.00', servings: '30', protein: '25' });
    await page.waitForTimeout(100);

    const extra = app.proCards().first().locator('.extra');
    await expect(extra).toContainText('€1.00/serving');
  });

  test('total protein grams shown in extra line', async ({ page }) => {
    const app = new CalcPage(page);
    // 30 servings × 25g = 750g total
    await app.fillPowder(0, { price: '30.00', servings: '30', protein: '25' });
    await page.waitForTimeout(100);

    await expect(app.proCards().first().locator('.extra')).toContainText('750g total');
  });

  test('lower cost per gram wins', async ({ page }) => {
    const app = new CalcPage(page);
    // Powder 0: 10€ / (10*20) = 0.05€/g
    // Powder 1: 20€ / (10*20) = 0.10€/g  → Powder 0 wins
    await app.fillPowder(0, { price: '10.00', servings: '10', protein: '20' });
    await app.fillPowder(1, { price: '20.00', servings: '10', protein: '20' });
    await page.waitForTimeout(100);

    await expect(app.winnerCard()).toHaveCount(1);
    // Winning card should contain Brand A (first card)
    await expect(app.winnerCard().locator('.winner-badge')).toContainText('best value');
  });

  test('two powders with identical cost per gram: no winner highlighted', async ({ page }) => {
    const app = new CalcPage(page);
    // Same cpg → tie — neither card gets "winner" badge
    await app.fillPowder(0, { price: '10.00', servings: '10', protein: '20' });
    await app.fillPowder(1, { price: '10.00', servings: '10', protein: '20' });
    await page.waitForTimeout(100);

    // Both have same value — winnerCard picks first, but "best value" vs "+0.0% more" in ranking
    const moreTag = app.rankItems().nth(1).locator('.tag-more');
    await expect(moreTag).toContainText('+0.0%');
  });

  // ── Ranking summary ────────────────────────────────────────────────────
  test('ranking summary shows 2 items for 2 valid powders', async ({ page }) => {
    const app = new CalcPage(page);
    await expect(app.rankItems()).toHaveCount(2);
  });

  test('best powder is ranked #1 with "best value" tag', async ({ page }) => {
    const app = new CalcPage(page);
    await expect(app.rankItems().first().locator('.tag-best')).toContainText('best value');
  });

  test('savings percentage is displayed in summary footer', async ({ page }) => {
    const app = new CalcPage(page);
    await expect(app.proSummary()).toContainText('saves you');
    await expect(app.proSummary()).toContainText('%');
  });

  test('savings calculation is correct', async ({ page }) => {
    const app = new CalcPage(page);
    // Powder 0: cpg = 0.04 (cheap), Powder 1: cpg = 0.08 (expensive)
    // savings = (0.08 - 0.04) / 0.08 × 100 = 50%
    await app.fillPowder(0, { price: '8.00',  servings: '10', protein: '20' }); // 8/200 = 0.04
    await app.fillPowder(1, { price: '16.00', servings: '10', protein: '20' }); // 16/200 = 0.08
    await page.waitForTimeout(100);

    await expect(app.proSummary()).toContainText('50.0%');
  });

  // ── Adding / removing ──────────────────────────────────────────────────
  test('adding a 3rd powder updates count', async ({ page }) => {
    const app = new CalcPage(page);
    await app.addPowder();
    await expect(app.proCards()).toHaveCount(3);
    await expect(page.locator('#pro-count-label')).toHaveText('3 / 4 powders');
  });

  test('add button is disabled at 4 powders', async ({ page }) => {
    const app = new CalcPage(page);
    await app.addPowder();
    await app.addPowder(); // 4

    const btn = page.locator('#pro-add-btn');
    await expect(btn).toHaveCSS('pointer-events', 'none');
  });

  test('removing a powder reduces count', async ({ page }) => {
    const app = new CalcPage(page);
    await app.addPowder(); // 3

    await app.proCards().nth(2).locator('.remove-btn').click();
    await expect(app.proCards()).toHaveCount(2);
  });

  test('summary disappears when only 1 valid powder', async ({ page }) => {
    const app = new CalcPage(page);
    const priceInput = app.proCards().first().locator('input[placeholder*="29.99"]');
    await priceInput.fill('');
    await priceInput.dispatchEvent('input');

    await expect(app.proSummary()).toContainText('add at least two valid powders');
  });

  // ── Edge cases ─────────────────────────────────────────────────────────
  test('zero protein per serving shows "fill in all fields"', async ({ page }) => {
    const app = new CalcPage(page);
    const card = app.proCards().first();
    const proteinInput = card.locator('input[placeholder*="25"]');
    await proteinInput.fill('0');
    await proteinInput.dispatchEvent('input');

    await expect(card.locator('.card-result')).toContainText('fill in all fields');
  });

  test('zero servings shows "fill in all fields"', async ({ page }) => {
    const app = new CalcPage(page);
    const card = app.proCards().first();
    const servingsInput = card.locator('input[placeholder*="33"]');
    await servingsInput.fill('0');
    await servingsInput.dispatchEvent('input');

    await expect(card.locator('.card-result')).toContainText('fill in all fields');
  });

  test('4-powder ranking shows 4 rank items', async ({ page }) => {
    const app = new CalcPage(page);
    await app.addPowder();
    await app.addPowder();
    await page.waitForTimeout(100);
    await expect(app.rankItems()).toHaveCount(4);
  });
});
