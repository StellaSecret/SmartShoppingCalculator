import { defineConfig, devices } from '@playwright/test';
import path from 'path';

// Serve the HTML directly from assets folder — no server needed
const htmlPath = path.resolve(__dirname, '../../app/src/main/assets/index.html');

export default defineConfig({
  testDir: './specs',
  fullyParallel: true,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: [
    ['html', { outputFolder: 'playwright-report', open: 'never' }],
    ['list'],
  ],
  use: {
    // Load the local HTML file directly — matches how Android WebView loads it
    baseURL: `file://${htmlPath}`,
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
  },
  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'] } },
    { name: 'firefox',  use: { ...devices['Desktop Firefox'] } },
    // Mobile viewport — closest to Android WebView
    { name: 'mobile-chrome', use: { ...devices['Pixel 7'] } },
  ],
});
