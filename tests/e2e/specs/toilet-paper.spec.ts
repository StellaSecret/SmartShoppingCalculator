import { test, expect } from '@playwright/test';
import { CalcPage } from './helpers';

test.describe('Toilet Paper — By Weight', () => {

  test('two default rolls are pre-loaded on startup', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await expect(app.tpCards()).toHaveCount(2);
  });

  test('counter label shows "2 / 4 rolls"', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await expect(page.locator('#tp-count-label')).toHaveText('2 / 4 rolls');
  });

  test('default rolls produce a valid cost-per-gram result', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    // Defaults: Roll A → 1.50 / (120-15)g = 0.01429€/g
    const result = await app.getRollResult(0);
    expect(result).toMatch(/0\.0\d+/);
  });

  test('ranking summary appears with 2 valid rolls', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await expect(app.tpResults()).not.toContainText('add at least two');
    await expect(app.tpResults().locator('.rank-item')).toHaveCount(2);
  });

  test('winner card is highlighted with class "winner"', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await expect(app.winnerCard()).toHaveCount(1);
  });

  test('winner badge shows "best value"', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await expect(app.winnerCard().locator('.winner-badge')).toContainText('best value');
  });

  test('rank list shows "+X% more" for non-winner', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await expect(app.rankItems().nth(1).locator('.tag-more')).toContainText('%');
  });

  test('savings summary text is displayed', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await expect(app.tpResults()).toContainText('saves you');
    await expect(app.tpResults()).toContainText('%');
  });

  test('cheaper roll per gram wins — verified manually', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();

    // Roll 0: 1.00€ / 100g net = 0.01000
    // Roll 1: 2.00€ / 100g net = 0.02000  → Roll 0 must win
    await app.fillRoll(0, { price: '1.00', packs: '1', totalW: '115', tubeW: '15' });
    await app.fillRoll(1, { price: '2.00', packs: '1', totalW: '115', tubeW: '15' });
    await page.waitForTimeout(100);

    const winner = app.winnerCard();
    await expect(winner.locator('input[value="Roll A"], input').first()).toHaveValue('Roll A');
  });

  test('multi-pack: price is divided by pack count', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();

    // 4-pack at 6.00€ → 1.50€/roll  (same as single at 1.50€)
    await app.fillRoll(0, { price: '1.50', packs: '1', totalW: '120', tubeW: '15' });
    await app.fillRoll(1, { price: '6.00', packs: '4', totalW: '120', tubeW: '15' });
    await page.waitForTimeout(100);

    const r0 = await app.getRollResult(0);
    const r1 = await app.getRollResult(1);
    expect(r0).toBe(r1);
  });

  test('adding a 3rd roll increases counter and card count', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.addRoll();

    await expect(app.tpCards()).toHaveCount(3);
    await expect(page.locator('#tp-count-label')).toHaveText('3 / 4 rolls');
  });

  test('add button is disabled at 4 rolls', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.addRoll();
    await app.addRoll(); // now 4

    const btn = page.locator('#tp-add-btn');
    await expect(btn).toHaveCSS('pointer-events', 'none');
  });

  test('removing a roll decreases count', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.addRoll(); // 3 rolls

    await app.tpCards().nth(2).locator('.remove-btn').click();
    await expect(app.tpCards()).toHaveCount(2);
  });

  test('result shows "fill in all fields" when price is empty', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();

    // Clear price of first roll
    const priceInput = app.tpCards().first().locator('input[placeholder*="1.99"]');
    await priceInput.fill('');
    await priceInput.dispatchEvent('input');

    await expect(app.tpCards().first().locator('.card-result')).toContainText('fill in all fields');
  });

  test('ranking disappears when only 1 valid roll remains', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();

    const priceInput = app.tpCards().first().locator('input[placeholder*="1.99"]');
    await priceInput.fill('');
    await priceInput.dispatchEvent('input');

    await expect(app.tpResults()).toContainText('add at least two valid rolls');
  });
});


test.describe('Toilet Paper — By Sheet Count', () => {

  test('switching to sheets mode shows sheet fields', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.setTpMethod('sheets');

    await expect(page.locator('#tpsec-sheets-0')).toHaveClass(/visible/);
    await expect(page.locator('#tpsec-weight-0')).not.toHaveClass(/visible/);
  });

  test('cost per 100cm² is computed correctly', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.setTpMethod('sheets');
    await page.waitForTimeout(50);

    // Roll A defaults: 1.50€, 200 sheets, 113mm × 100mm
    // area = 200 * 113 * 100 / 10000 = 2260 cm²
    // cost/100cm² = (1.50 / 2260) * 100 = 0.06637
    const result = await app.getRollResult(0);
    const val = parseFloat(result);
    expect(val).toBeCloseTo(0.0664, 3);
  });

  test('unit label shows €/100cm²', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.setTpMethod('sheets');
    await page.waitForTimeout(50);
    await expect(app.tpCards().first().locator('.cpg-label')).toContainText('€/100cm²');
  });
});


test.describe('Toilet Paper — By Diameter', () => {

  test('switching to diameter mode shows diameter fields', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.setTpMethod('diameter');

    await expect(page.locator('#tpsec-diameter-0')).toHaveClass(/visible/);
    await expect(page.locator('#tpsec-weight-0')).not.toHaveClass(/visible/);
  });

  test('volume is computed from outer/inner/width cylinders', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.setTpMethod('diameter');
    await page.waitForTimeout(50);

    // Roll A defaults: outer=110, inner=40, width=100
    // vol = π * ((55)²-(20)²) * 100 / 1000 = π * (3025-400) * 0.1 = π * 262.5 ≈ 824.67 cm³
    // cpg = 1.50 / 824.67 ≈ 0.001819
    const result = await app.getRollResult(0);
    const val = parseFloat(result);
    expect(val).toBeCloseTo(0.00182, 4);
  });

  test('unit label shows €/cm³', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.setTpMethod('diameter');
    await page.waitForTimeout(50);
    await expect(app.tpCards().first().locator('.cpg-label')).toContainText('€/cm³');
  });
});


test.describe('Toilet Paper — By Hand', () => {

  test('switching to hand mode shows calibration panel', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.setTpMethod('hand');

    await expect(page.locator('#hand-calibration')).toBeVisible();
  });

  test('hand calibration panel hides when switching away', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.setTpMethod('hand');
    await app.setTpMethod('weight');

    await expect(page.locator('#hand-calibration')).toBeHidden();
  });

  test('hand fields are visible when method is "hand"', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.setTpMethod('hand');

    await expect(page.locator('#tpsec-hand-0')).toHaveClass(/visible/);
  });

  test('unit label shows €/cm³ est.', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.setTpMethod('hand');
    await page.waitForTimeout(50);
    await expect(app.tpCards().first().locator('.cpg-label')).toContainText('est.');
  });

  test('changing finger width updates result', async ({ page }) => {
    const app = new CalcPage(page);
    await app.goto();
    await app.setTpMethod('hand');
    await page.waitForTimeout(50);

    const before = await app.getRollResult(0);

    const fingerInput = page.locator('#hand-finger');
    await fingerInput.fill('22');
    await fingerInput.dispatchEvent('input');
    await page.waitForTimeout(50);

    const after = await app.getRollResult(0);
    expect(before).not.toBe(after);
  });
});
