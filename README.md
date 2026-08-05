# Smart Shopping Calculator — Rust + Dioxus

A rewrite of the original Android/WebView app (HTML/CSS/JS in
`app/src/main/assets/index.html`) as a Rust web app using
[Dioxus](https://dioxuslabs.com/) 0.7, deployed to GitHub Pages at
`/SmartShoppingCalculator/`.

It keeps the same core idea: compare products by the unit that
actually matters instead of sticker price — toilet paper by cost per
gram of paper (or per 100cm² of sheet area, per cm³ of roll volume, or
a "measure with your hand" estimate), and protein powder by cost per
gram of protein.

## What's included

- Both calculators (toilet paper + protein powder), up to 4 items each
- All four toilet-paper comparison methods: by weight, by sheet count,
  by diameter, and the hand-estimation method
- Live ranking list with a "best value" tag, cost bars, and a
  "choosing X saves you Y%" callout
- Lifetime cost projection (rolls/servings per week → cost per year)
- Session save/restore/clear via browser `localStorage`
- Dark mode (persisted, and defaults to the OS `prefers-color-scheme`)
- English/French UI toggle (persisted)
- Currency selector (persisted)

## What's not included

- **Barcode scanning.** The original Android app used a native
  `AndroidBridge` JS bridge to talk to a system camera/barcode scanner;
  there's no browser equivalent, so this was left out rather than
  faked.

## Project layout

```
core/           pure logic workspace crate (no UI)
  src/models.rs TpRoll / Powder / Lang / TpMethod + session structs
  src/calc.rs   all the unit-cost math (ported from tp.js / protein.js)
src/            web app (Dioxus)
  main.rs       entry point (dioxus::launch)
  app.rs        root component, pages, cards, results/ranking rendering
  i18n.rs       English + French UI strings
  storage.rs    localStorage helpers (theme, language, currency, sessions)
  css.rs        the app's stylesheet (CSS variables toggled by `.dark`)
public/         static site merged over the WASM bundle at deploy time
tests/          Playwright e2e smoke tests
scripts/        SPA test server (serves the built bundle with the base path)
```

## Running it

This targets the browser (WebAssembly), via Dioxus's `web` renderer.
You'll need a reasonably recent Rust toolchain (rustc 1.80+):

```bash
# 1. Install/refresh Rust (https://rustup.rs)
rustup update stable
rustup target add wasm32-unknown-unknown

# 2. Install the Dioxus CLI (v0.7.x)
cargo install dioxus-cli

# 3. From this directory:
dx serve
```

Then open the URL it prints (http://localhost:8080/SmartShoppingCalculator/ — the base path is set in `Dioxus.toml` `[web] base_path`).

For a static production build:

```bash
dx build --release --base-path /SmartShoppingCalculator
```

The bundled output lands in `target/dx/smart-shopping-calculator/release/web/public/` — deploy that folder anywhere that serves static files.

## Tests

Core math unit tests (Rust):

```bash
cargo test -p smart-shopping-calculator-core
```

End-to-end smoke tests (Playwright, needs the web bundle built first):

```bash
npm ci
npx playwright test
```

## CI

`.github/workflows/build.yml` mirrors the PeopleModeler pipeline:
secret scan (trufflehog), Rust core tests + WASM check, Dioxus WASM
bundle build (dx 0.7.9), Playwright e2e (sharded), Android APK/AAB
build, GitHub Pages deploy (main only), and a `build-<run>` GitHub
Release. Requires the Pages source to be set to **GitHub Actions**.

> **Note on verification:** the project compiles and links cleanly for
> `wasm32-unknown-unknown` and `aarch64-linux-android` (verified with
> `cargo check` and release `cargo build`, rustc 1.97). The math is a
> close, function-by-function port of the original JS (verified against
> `tpCalc()`, `proCalc()`, and the `STRINGS` tables), locked in by the
> core crate's unit tests. The Dioxus `rsx!` macro doesn't support
> `let` statements inside `for` loops, so ranking rows are precomputed
> into `TpRank`/`ProRank` structs before rendering.
