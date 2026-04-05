/* INIT */
// Render toilet paper (active page on load)
tpAdd(); tpAdd();
// Initialize protein powder state without rendering the grid,
// so inactive-page .item-card elements don't pollute global selectors.
// The grid renders when the user switches to the protein tab.
let _proInitOnly = true;
proAdd(); proAdd();
_proInitOnly = false;
