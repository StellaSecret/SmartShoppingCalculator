/* Shared theme/lang toggles for static pages.
   Persists to the same localStorage keys the Dioxus SPA uses
   (src/storage.rs): "theme" = "dark"|"light", "lang" = "en"|"fr". */
(function () {
  "use strict";

  var THEME_KEY = "theme";
  var LANG_KEY = "lang";

  function stored(key) {
    try { return localStorage.getItem(key); } catch (e) { return null; }
  }

  /* The SPA (gloo-storage) JSON-encodes strings, so values are stored
     quoted ("dark", "fr"). Read both quoted and raw. */
  function readVal(key) {
    var v = stored(key);
    if (v === null) return null;
    try { return JSON.parse(v); } catch (e) { return v; }
  }

  /* Write JSON-encoded so the SPA's LocalStorage::get::<String> can parse it. */
  function save(key, value) {
    try { localStorage.setItem(key, JSON.stringify(value)); } catch (e) {}
  }

  function getLang() {
    return readVal(LANG_KEY) === "fr" ? "fr" : "en";
  }

  function getTheme() {
    var t = readVal(THEME_KEY);
    if (t === "dark" || t === "light") return t;
    return window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
  }

  function setTheme(dark) {
    document.documentElement.classList.toggle("dark", dark);
    save(THEME_KEY, dark ? "dark" : "light");
  }

  function applyLang(lang) {
    var dict = window.SSC_DICT || {};
    document.documentElement.setAttribute("lang", lang);
    document.querySelectorAll("[data-i18n]").forEach(function (el) {
      var entry = dict[el.getAttribute("data-i18n")];
      if (entry && entry[lang]) el.innerHTML = entry[lang];
    });
    save(LANG_KEY, lang);
  }

  function init(dict) {
    window.SSC_DICT = dict;

    var lang = getLang();
    var theme = getTheme();
    var dark = theme === "dark";

    applyLang(lang);
    setTheme(dark);

    var langBtn = document.getElementById("btn-lang");
    var themeBtn = document.getElementById("btn-theme");

    if (langBtn) {
      langBtn.textContent = lang === "fr" ? "EN" : "FR";
      langBtn.addEventListener("click", function () {
        var next = getLang() === "fr" ? "en" : "fr";
        applyLang(next);
        langBtn.textContent = next === "fr" ? "EN" : "FR";
      });
    }

    if (themeBtn) {
      themeBtn.textContent = dark ? "☀️ light" : "🌙 dark";
      themeBtn.addEventListener("click", function () {
        var next = getTheme() === "dark" ? "light" : "dark";
        setTheme(next === "dark");
        themeBtn.textContent = next === "dark" ? "☀️ light" : "🌙 dark";
      });
    }
  }

  window.SSC = { init: init };
})();
