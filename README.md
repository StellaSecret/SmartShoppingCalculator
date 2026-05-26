# 🛒 Smart Shopping Calculator

Compare everyday products by their **real unit cost** — toilet paper by €/g of paper, protein powder by €/g of protein. No ads, no account, runs entirely offline.

→ **[Try it on the web](https://falltrades.github.io/SmartShoppingCalculator/)** · [Privacy Policy](https://falltrades.github.io/SmartShoppingCalculator/privacy.html)

---

## Project structure

```
├── index.html          ← Shell HTML + CSS (web & Android)
├── darkmode.js         ← Dark mode toggle + localStorage
├── nav.js              ← Tab navigation, page switching
├── tp.js               ← Toilet paper logic (4 methods: weight/sheets/diameter/hand)
├── protein.js          ← Protein powder logic (cost per gram, ranking)
├── init.js             ← Bootstrap: add default rolls/powders
│
├── app/
│   └── src/main/
│       ├── assets/     ← Exact copy of root web files (served by Android WebView)
│       ├── java/com/stellasecret/smartshoppingcalculator/
│       │   ├── MainActivity.kt    ← WebView host, lifecycle, dark mode injection
│       │   └── AndroidBridge.kt  ← JS ↔ Kotlin bridge (Toast, dark mode, version)
│       └── res/                   ← Icons, layouts, themes
│
├── tests/
│   ├── e2e/            ← Playwright end-to-end tests (65 tests, 3 browsers)
│   ├── unit/           ← JUnit5 + Mockk Kotlin unit tests (71 tests)
│   └── .husky/pre-commit ← Pre-commit checks (secrets, keys, sync, YAML)
│
└── .github/workflows/build.yml  ← CI/CD pipeline
```

> **Rule:** `index.html` and the 5 JS files must always be **identical** between the repo root and `app/src/main/assets/`. The pre-commit hook enforces this.

---

## Running locally

### Web (no setup needed)
Open `index.html` directly in a browser — everything is local, no server required.

### Android
1. Open the repo root in **Android Studio Hedgehog (2023.1+)**
2. Let Gradle sync finish
3. `Run` (debug build, no signing needed)

### Command line
```bash
# Debug APK
./gradlew assembleDebug

# Release APK + AAB (signing injected by CI — see below)
./gradlew assembleRelease bundleRelease
```

---

## Tests

### Playwright — E2E (`tests/e2e/`)

Covers all business logic in the JS modules: 4 toilet paper methods, protein powder, ranking, edge cases, mobile viewport (Pixel 7).

```bash
cd tests/e2e
npm ci
npx playwright install --with-deps chromium firefox
npx playwright test           # headless
npx playwright test --headed  # visible browser
npx playwright test --ui      # interactive mode
npx playwright show-report    # HTML report
```

| Suite | File | Tests |
|-------|------|-------|
| Navigation & Dark Mode | `navigation.spec.ts` | 8 |
| Toilet Paper (4 methods) | `toilet-paper.spec.ts` | 22 |
| Protein Powder | `protein.spec.ts` | 18 |
| Edge Cases & Mobile | `edge-cases.spec.ts` | 11 |

### Kotlin unit tests (`tests/unit/`)

Covers `AndroidBridge`, all calculation formulae, ranking, and `MainActivity` lifecycle.

```bash
./gradlew :tests:unit:test --no-daemon
# Report: tests/unit/build/reports/tests/test/index.html
```

| File | What it tests |
|------|---------------|
| `AndroidBridgeTest.kt` | Toast, system dark mode, app version |
| `CalcLogicTest.kt` | Weight/sheets/diameter/protein formulae, ranking, savings |
| `MainActivityTest.kt` | WebView lifecycle, JS injection, swipe refresh, back nav |

---

## CI/CD pipeline

```
push to main
  ├── test-kotlin    (JUnit5 — 71 tests)
  ├── test-playwright (Playwright — 65 tests × 3 browsers)
  │
  └── [both pass] → build
                      ├── assembleRelease + bundleRelease (signed via -P flags)
                      ├── deploy-pages (GitHub Pages)
                      └── release (GitHub Release with APK + AAB)
```

PRs get a debug APK only — no signing, no release.

### Required GitHub secrets

| Secret | How to get it |
|--------|---------------|
| `KEYSTORE_BASE64` | `base64 -w0 your-release.jks` |
| `KEYSTORE_PASSWORD` | Store password set when creating the keystore |
| `KEY_ALIAS` | Run `keytool -list -keystore your-release.jks` — first word before the comma |
| `KEY_PASSWORD` | Key password (often same as store password) |

---

## Adding a new product category

1. Create `yourcategory.js` in the repo root (follow the pattern in `tp.js` or `protein.js`)
2. Copy it to `app/src/main/assets/yourcategory.js`
3. Add `<script src="yourcategory.js"></script>` to `index.html` (and copy to assets)
4. Add a tab button in `index.html` calling `showPage('yourcategory', this)`
5. Write Playwright specs in `tests/e2e/specs/yourcategory.spec.ts`

---

## JavaScript ↔ Android bridge

The app exposes `window.AndroidBridge` in the WebView:

```javascript
AndroidBridge.showToast("Saved!");          // native Toast
AndroidBridge.isSystemDarkMode();           // boolean — system dark theme?
AndroidBridge.getAppVersion();              // "1.0.42"
```

---

## Pre-commit hooks

Install once per machine:
```bash
cd tests && npm install
```

Checks on every commit (< 3 seconds):

| Check | Why |
|-------|-----|
| No secret patterns | Secrets in git history are permanent |
| No `.jks`/`.keystore` files | Signing keys must never be committed |
| `gradle-wrapper.jar` not deleted | Required by CI |
| No `console.log` in HTML | Debug noise in production |
| No files > 1MB | Catches accidental APK/zip commits |
| YAML/JSON syntax | Broken workflow = wasted CI runner |
| Web files in sync | root == assets, always |
| AndroidBridge drift | Test copy must match production signatures |
