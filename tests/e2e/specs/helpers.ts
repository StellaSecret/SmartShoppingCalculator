import { Page, Locator } from '@playwright/test';

/**
 * Page Object for Smart Shopping Calculator.
 * Wraps all selectors so tests stay readable when the HTML changes.
 */
export class CalcPage {
  constructor(readonly page: Page) {}

  // ── Navigation ───────────────────────────────────────────────────────────
  async goto() {
    await this.page.goto('');    // baseURL is already set to index.html
    await this.page.waitForSelector('.nav-tabs');
  }

  async switchToToiletPaper() {
    await this.page.click('button.nav-tab:has-text("toilet paper")');
  }

  async switchToProtein() {
    await this.page.click('button.nav-tab:has-text("protein powder")');
  }

  // ── Dark mode ────────────────────────────────────────────────────────────
  get darkBtn()   { return this.page.locator('#dark-btn'); }
  isDark()        { return this.page.evaluate(() => document.body.classList.contains('dark')); }

  // ── Toilet Paper helpers ─────────────────────────────────────────────────
  async addRoll()   { await this.page.click('#tp-add-btn'); }
  async setTpMethod(method: 'weight' | 'sheets' | 'diameter' | 'hand') {
    const labels: Record<string, string> = {
      weight: 'By weight', sheets: 'By sheet count',
      diameter: 'By diameter', hand: 'By hand',
    };
    await this.page.click(`.tab-btn:has-text("${labels[method]}")`);
  }

  /**
   * Fill the visible fields for a roll card by its 0-based index.
   * Only pass the keys you care about — others are left as-is (defaults).
   */
  async fillRoll(cardIndex: number, fields: Partial<{
    price: string; packs: string;
    // weight
    totalW: string; tubeW: string;
    // sheets
    sheets: string; sheetLen: string; sheetWid: string;
    // diameter
    outer: string; inner: string; width: string;
    // hand
    hOuter: string;
  }>) {
    const card = this.page.locator('.item-card').nth(cardIndex);

    const set = async (placeholder: string, value: string) => {
      const input = card.locator(`input[placeholder*="${placeholder}"]`).first();
      if (await input.count() > 0) {
        await input.fill(value);
        await input.dispatchEvent('input');
      }
    };

    if (fields.price    != null) await set('1.99',  fields.price);
    if (fields.packs    != null) await set('4',      fields.packs);
    if (fields.totalW   != null) await set('120',    fields.totalW);
    if (fields.tubeW    != null) await set('15',     fields.tubeW);
    if (fields.sheets   != null) await set('200',    fields.sheets);
    if (fields.sheetLen != null) await set('113',    fields.sheetLen);
    if (fields.sheetWid != null) await set('100',    fields.sheetWid);
    if (fields.outer    != null) await set('110',    fields.outer);
    if (fields.inner    != null) await set('40',     fields.inner);
    if (fields.width    != null) await set('100',    fields.width);
    if (fields.hOuter   != null) await set('6',      fields.hOuter);
  }

  /** Returns the displayed unit cost string from a roll card result area */
  async getRollResult(cardIndex: number): Promise<string> {
    return this.page.locator('.item-card').nth(cardIndex).locator('.cpg').innerText();
  }

  tpResults() { return this.page.locator('#tp-results'); }
  tpCards()   { return this.page.locator('#tp-grid .item-card'); }

  // ── Protein Powder helpers ───────────────────────────────────────────────
  async addPowder() { await this.page.click('#pro-add-btn'); }

  async fillPowder(cardIndex: number, fields: Partial<{
    price: string; weight: string; servings: string; protein: string;
  }>) {
    const card = this.page.locator('#pro-grid .item-card').nth(cardIndex);

    const setByLabel = async (labelText: string, value: string) => {
      const field = card.locator('.field').filter({ hasText: labelText });
      const input = field.locator('input[type="number"]').first();
      if (await input.count() > 0) {
        await input.fill(value);
        await input.dispatchEvent('input');
      }
    };

    if (fields.price    != null) await setByLabel('Price',    fields.price);
    if (fields.weight   != null) await setByLabel('weight',   fields.weight);
    if (fields.servings != null) await setByLabel('Servings', fields.servings);
    if (fields.protein  != null) await setByLabel('Protein',  fields.protein);
  }

  async getProteinResult(cardIndex: number): Promise<string> {
    return this.page.locator('.item-card').nth(cardIndex).locator('.cpg').innerText();
  }

  proSummary() { return this.page.locator('#pro-summary'); }
  proCards()   { return this.page.locator('#pro-grid .item-card'); }

  // ── Shared helpers ───────────────────────────────────────────────────────
  winnerCard() { return this.page.locator('.item-card.winner'); }
  rankItems()  { return this.page.locator('.page.visible .rank-item'); }
}
