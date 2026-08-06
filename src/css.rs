pub const CSS: &str = r#"
*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

:root {
  --bg: #f0f0ed;
  --surface: #fafaf7;
  --border: #d4d0c8;
  --text: #18181a;
  --muted: #76746e;
  --win: #2d5a1a;
  --win-bg: #eaf2e6;
  --danger: #b3261e;
  --tag-more-bg: #eeebe5;
  --mono: 'DM Mono', monospace;
  --sans: 'DM Sans', sans-serif;
}

.app.dark {
  --bg: #18181a;
  --surface: #222224;
  --border: #333336;
  --text: #f0f0ed;
  --muted: #888884;
  --win: #5aaa3a;
  --win-bg: #1a2e12;
  --danger: #e5a3a0;
  --tag-more-bg: #2a2a2c;
}

html, body { background: var(--bg); }

.app {
  background: var(--bg);
  color: var(--text);
  font-family: var(--sans);
  min-height: 100vh;
  padding: 2rem 1rem 3rem;
  transition: background 0.2s, color 0.2s;
}

.container { max-width: 800px; margin: 0 auto; }

header { margin-bottom: 2rem; padding-bottom: 1.25rem; border-bottom: 1px solid var(--border); }
header h1 { font-family: var(--mono); font-size: 1.4rem; font-weight: 500; letter-spacing: -0.02em; }
header p { font-size: 0.875rem; color: var(--muted); margin-top: 0.35rem; }

.header-row { display:flex; align-items:flex-start; justify-content:space-between; gap:12px; }
.header-controls { display:flex; gap:6px; flex-shrink:0; margin-top:2px; }
.pill-btn {
  padding:6px 12px; font-family:var(--mono); font-size:0.78rem;
  border:1px solid var(--border); background:var(--surface); color:var(--muted);
  border-radius:4px; cursor:pointer; transition:all 0.15s; white-space:nowrap;
}
.pill-btn:hover { color: var(--text); border-color: var(--text); }
.currency-select {
  padding:6px 8px; font-family:var(--mono); font-size:0.78rem; border:1px solid var(--border);
  background:var(--surface); color:var(--muted); border-radius:4px; cursor:pointer; outline:none;
}

.nav-tabs { display: flex; gap: 6px; margin-bottom: 2rem; border-bottom: 1px solid var(--border); }
.nav-tab {
  padding: 8px 20px; font-family: var(--mono); font-size: 0.85rem; border: 1px solid transparent;
  border-bottom: none; background: transparent; color: var(--muted); border-radius: 6px 6px 0 0;
  cursor: pointer; transition: all 0.15s; position: relative; bottom: -1px;
}
.nav-tab:hover { color: var(--text); }
.nav-tab.active {
  background: var(--surface); color: var(--text); border-color: var(--border);
  border-bottom-color: var(--surface); font-weight: 500;
}

.page { display: none; }
.page.visible { display: block; }

.field { margin-bottom: 8px; }
.field label { display: block; font-size: 0.72rem; color: var(--muted); font-family: var(--mono); margin-bottom: 3px; }
.field input, .field select {
  width: 100%; padding: 7px 10px; font-family: var(--mono); font-size: 0.83rem;
  border: 1px solid var(--border); border-radius: 4px; background: var(--bg); color: var(--text);
  outline: none; transition: border-color 0.15s;
}
.field input:focus, .field select:focus { border-color: var(--text); }
.divider { height: 1px; background: var(--border); margin: 10px 0; }

.results-summary { background: var(--surface); border: 1px solid var(--border); border-radius: 8px; padding: 1.25rem; margin-top: 0.5rem; }
.summary-title { font-family: var(--mono); font-size: 0.8rem; color: var(--muted); margin-bottom: 1rem; }
.empty { text-align: center; font-family: var(--mono); font-size: 0.8rem; color: var(--muted); padding: 1.5rem; }

.top-controls { display: flex; align-items: center; gap: 10px; margin-bottom: 1.25rem; flex-wrap: wrap; }
.top-controls label { font-size: 0.8rem; color: var(--muted); font-family: var(--mono); }

.add-btn {
  padding: 7px 16px; font-family: var(--mono); font-size: 0.8rem; border: 1px solid var(--border);
  background: var(--surface); color: var(--text); border-radius: 4px; cursor: pointer; transition: all 0.15s;
}
.add-btn:hover { background: var(--text); color: var(--bg); border-color: var(--text); }
.add-btn.disabled { opacity: 0.4; pointer-events: none; }

.session-bar { display: flex; align-items: center; gap: 8px; margin-bottom: 1.25rem; flex-wrap: wrap; }
.session-btn {
  padding: 5px 12px; font-family: var(--mono); font-size: 0.75rem; border: 1px solid var(--border);
  background: var(--surface); color: var(--muted); border-radius: 4px; cursor: pointer; transition: all 0.15s;
}
.session-btn:hover { color: var(--text); border-color: var(--text); }
.session-btn.has-save { color: var(--win); border-color: var(--win); }
.session-toast { font-family: var(--mono); font-size: 0.72rem; color: var(--win); opacity: 0; transition: opacity 0.3s; }
.session-toast.show { opacity: 1; }

.method-tabs { display: flex; gap: 6px; margin-bottom: 1.5rem; flex-wrap: wrap; }
.tab-btn {
  padding: 7px 16px; font-family: var(--mono); font-size: 0.8rem; border: 1px solid var(--border);
  background: var(--surface); color: var(--muted); border-radius: 4px; cursor: pointer; transition: all 0.15s;
}
.tab-btn:hover { color: var(--text); border-color: var(--text); }
.tab-btn.active { background: var(--text); color: var(--bg); border-color: var(--text); }

.cards-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; margin-bottom: 1.5rem; }

.item-card { background: var(--surface); border: 1px solid var(--border); border-radius: 8px; padding: 1rem; transition: border-color 0.2s; }
.item-card.winner { border-color: var(--win); border-width: 2px; }

.card-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 0.875rem; }
.card-title { display: flex; align-items: center; gap: 7px; font-family: var(--mono); font-size: 0.82rem; font-weight: 500; }
.name-input {
  border:none; background:transparent; font-family:var(--mono); font-size:0.82rem; font-weight:500;
  color:var(--text); width:100px; outline:none; padding:0;
}
.dot { width: 9px; height: 9px; border-radius: 50%; flex-shrink: 0; display: inline-block; }
.remove-btn { background: none; border: none; cursor: pointer; color: var(--muted); font-size: 1rem; line-height: 1; padding: 0 2px; transition: color 0.15s; }
.remove-btn:hover { color: var(--text); }
.card-actions { display: flex; align-items: center; gap: 8px; flex-shrink: 0; }
.scan-btn {
  padding: 3px 8px; font-family: var(--mono); font-size: 0.66rem; line-height: 1;
  border: 1px solid var(--border); border-radius: 4px; background: var(--surface);
  color: var(--muted); cursor: pointer; white-space: nowrap; transition: all 0.15s;
}
.scan-btn:hover { color: var(--text); border-color: var(--text); }
.scan-status { font-family: var(--mono); font-size: 0.68rem; color: var(--muted); margin: -4px 0 10px; }
.scan-status.error { color: var(--danger); }

.card-result { margin-top: 10px; padding-top: 10px; border-top: 1px solid var(--border); font-family: var(--mono); }
.cpg { font-size: 1.2rem; font-weight: 500; }
.cpg-label { font-size: 0.68rem; color: var(--muted); margin-top: 2px; }
.extra { font-size: 0.72rem; color: var(--muted); margin-top: 4px; }
.winner-badge { display: inline-block; font-size: 0.68rem; padding: 2px 8px; background: var(--win-bg); color: var(--win); border-radius: 3px; margin-top: 5px; }

.rank-list { list-style: none; }
.rank-item { padding: 9px 0 7px; border-bottom: 1px solid var(--border); font-family: var(--mono); font-size: 0.82rem; }
.rank-item:last-child { border-bottom: none; }
.rank-row { display: flex; align-items: center; gap: 10px; }
.rank-num { font-size: 0.72rem; color: var(--muted); min-width: 18px; }
.rank-name { flex: 1; }
.rank-val { font-weight: 500; }
.tag-best { font-size: 0.72rem; padding: 2px 7px; border-radius: 3px; background: var(--win-bg); color: var(--win); }
.tag-more { font-size: 0.72rem; padding: 2px 7px; border-radius: 3px; background: var(--tag-more-bg); color: var(--muted); }

.cost-bar-track { height: 4px; background: var(--border); border-radius: 2px; margin-top: 5px; overflow: hidden; }
.cost-bar-fill { height: 100%; border-radius: 2px; transition: width 0.4s ease; }

.lifetime-block { margin-top: 14px; padding-top: 12px; border-top: 1px solid var(--border); }
.lifetime-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 8px; }
.lifetime-title { font-family: var(--mono); font-size: 0.75rem; color: var(--muted); }
.lifetime-inputs { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; }
.lifetime-inputs label { font-family: var(--mono); font-size: 0.72rem; color: var(--muted); }
.lifetime-inputs input {
  width: 52px; padding: 4px 7px; font-family: var(--mono); font-size: 0.78rem;
  border: 1px solid var(--border); border-radius: 4px; background: var(--bg); color: var(--text); outline: none;
}
.lifetime-inputs input:focus { border-color: var(--text); }
.lifetime-result { margin-top: 8px; font-family: var(--mono); font-size: 0.8rem; color: var(--muted); line-height: 1.5; }
.lifetime-result strong { color: var(--win); }

.tp-section { display: none; }
.tp-section.visible { display: block; }

footer { margin-top: 2rem; padding-top: 1rem; border-top: 1px solid var(--border); font-size: 0.72rem; color: var(--muted); font-family: var(--mono); }

@media (max-width: 520px) { .cards-grid { grid-template-columns: 1fr 1fr; } }
@media (max-width: 360px) { .cards-grid { grid-template-columns: 1fr; } }
"#;
