import { Page } from '@playwright/test';

export const APP_PATH = '/SmartShoppingCalculator/';

/**
 * Page Object for Smart Shopping Calculator (dioxus build).
 * Wraps all selectors so tests stay readable when the markup changes.
 * The dioxus app renders only one page at a time (TP or protein), so
 * global `.item-card` / `.cards-grid` locators are unambiguous.
 */
export class CalcPage {
  constructor(readonly page: Page) {}

  // ── Navigation ───────────────────────────────────────────────────────────
  async goto() {
    // Note: the old port 4190 is on Firefox's banned-port list (ManageSieve),
    // so Firefox cancelled every navigation before connecting. Now 4200.
    // Keep waitUntil 'commit' + card wait: the wasm load can process-swap and
    // supersede the navigation; on "interrupted by another navigation" we
    // DON'T re-navigate (that re-triggers the race) — the superseding
    // navigation lands on the same APP_PATH url, so we wait for the app.
    const ctx = this.page.context() as { _options?: { baseURL?: string } };
    const base = ctx._options?.baseURL ?? 'http://localhost:4200';
    const url = new URL(APP_PATH, base).toString();
    for (let attempt = 0; attempt < 2; attempt++) {
      try {
        await this.page.goto(url, { waitUntil: 'commit', timeout: 10_000 });
        break;
      } catch (err) {
        const msg = err instanceof Error ? err.message : String(err);
        if (!/interrupted by another (navigation|one)/i.test(msg)) throw err;
      }
    }
    try {
      await this.page.waitForSelector('.cards-grid .item-card', { timeout: 10_000 });
    } catch {
      // Surface a real timeout with the page state for CI diagnosis.
      throw new Error(`App did not render after navigation. page.url()=${this.page.url()}`);
    }
  }

  async switchToToiletPaper() {
    await this.page.locator('.nav-tab').nth(0).click();
    await this.page.waitForSelector('.cards-grid .item-card');
  }

  async switchToProtein() {
    await this.page.locator('.nav-tab').nth(1).click();
    await this.page.waitForSelector('.cards-grid .item-card');
  }

  // ── Dark mode ────────────────────────────────────────────────────────────
  // The theme class lives on the app root div, not <body>.
  get darkBtn() {
    return this.page.locator('.header-controls .pill-btn').nth(1);
  }
  isDark() {
    return this.page.evaluate(() => document.querySelector('.app')!.classList.contains('dark'));
  }
  /** Force a known light state and reload so the app re-initialises. */
  async resetLight() {
    await this.page.evaluate(() => {
      document.querySelector('.app')!.classList.remove('dark');
      localStorage.setItem('theme', JSON.stringify('light'));
    });
    try {
      await this.page.reload();
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      if (!/interrupted by another (navigation|one)/i.test(msg)) throw err;
    }
    await this.page.waitForSelector('.cards-grid .item-card');
  }

  // ── Toilet Paper helpers ─────────────────────────────────────────────────
  async addRoll() { await this.page.locator('.top-controls .add-btn').click(); }

  async setTpMethod(method: 'weight' | 'sheets' | 'diameter' | 'hand') {
    const idx: Record<string, number> = { weight: 0, sheets: 1, diameter: 2, hand: 3 };
    await this.page.locator('.tab-btn').nth(idx[method]).click();
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

  tpResults() { return this.page.locator('.results-summary'); }
  tpCards()   { return this.page.locator('.cards-grid .item-card'); }

  // ── Protein Powder helpers ───────────────────────────────────────────────
  async addPowder() { await this.page.locator('.top-controls .add-btn').click(); }

  async fillPowder(cardIndex: number, fields: Partial<{
    price: string; weight: string; servings: string; protein: string;
  }>) {
    const card = this.page.locator('.cards-grid .item-card').nth(cardIndex);

    const setByLabel = async (labelText: string, value: string) => {
      const field = card.locator('.field').filter({ hasText: labelText });
      const input = field.locator('input[type="number"]').first();
      if (await input.count() > 0) {
        await input.fill(value);
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

  proSummary() { return this.page.locator('.results-summary'); }
  proCards()   { return this.page.locator('.cards-grid .item-card'); }

  // ── Shared helpers ───────────────────────────────────────────────────────
  winnerCard() { return this.page.locator('.item-card.winner'); }
  rankItems()  { return this.page.locator('.rank-list .rank-item'); }
}
