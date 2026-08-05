# Smart Shopping Calculator — Rust + Dioxus port

A rewrite of the original Android/WebView app (HTML/CSS/JS in
`app/src/main/assets/index.html`) as a Rust web app using
[Dioxus](https://dioxuslabs.com/) 0.6.

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
- The Android `WebView` wrapper itself (splash screen, native manifest,
  etc.) — this port is a standalone web app.

## Project layout

```
src/
  main.rs     entry point (dioxus::launch)
  app.rs      root component, pages, cards, results/ranking rendering
  models.rs   TpRoll / Powder / Lang / Page / TpMethod + session structs
  calc.rs     all the unit-cost math (ported from tp.js / protein.js)
  i18n.rs     English + French UI strings
  storage.rs  localStorage helpers (theme, language, currency, sessions)
  css.rs      the app's stylesheet (same look as the original, adapted
              to CSS variables toggled by a `.dark` class)
```

## Running it

This targets the browser (WebAssembly), via Dioxus's `web` renderer.
You'll need a reasonably recent Rust toolchain — newer than what's in
Ubuntu's default `apt` packages (this pulls in `wasm-bindgen`/`web-sys`
versions that need rustc 1.77+). Easiest path:

```bash
# 1. Install/refresh Rust (https://rustup.rs)
rustup update stable
rustup target add wasm32-unknown-unknown

# 2. Install the Dioxus CLI
cargo install dioxus-cli

# 3. From this directory:
dx serve
```

Then open the URL it prints (http://localhost:8080/SmartShoppingCalculator/ — the base path mirrors the GitHub Pages deployment, set in `Dioxus.toml` `[web.app] base_path`).

For a static production build:

```bash
dx build --release
```

The bundled output lands in `target/dx/smart-shopping-calculator/release/web/public/` — deploy that folder anywhere that serves static files.

> **Note on verification:** the project compiles and links cleanly for
> `wasm32-unknown-unknown` (verified with `cargo check` and both dev and
> release `cargo build`, rustc 1.97). The math is a close, function-by-
> function port of the original JS (verified against `tpCalc()`,
> `proCalc()`, and the `STRINGS` tables). Note that the Dioxus `rsx!`
> macro doesn't support `let` statements inside `for` loops, so ranking
> rows are precomputed into `TpRank`/`ProRank` structs before rendering.
> Requires the `wasm32-unknown-unknown` target installed for the
> toolchain you build with.
