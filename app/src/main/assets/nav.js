/* NAV */
function showPage(id, btn) {
  document.querySelectorAll('.page').forEach(p => p.classList.remove('visible'));
  document.querySelectorAll('.nav-tab').forEach(b => b.classList.remove('active'));
  document.getElementById('page-' + id).classList.add('visible');
  btn.classList.add('active');
  // Clear inactive grid so .item-card only matches active page's cards
  if (id === 'tp') {
    document.getElementById('pro-grid').innerHTML = '';
    tpRender();
  } else {
    document.getElementById('tp-grid').innerHTML = '';
    proRender();
  }
}

const COLORS = [
  { dot: '#1a5c3a' },
  { dot: '#1a3a7a' },
  { dot: '#7a3a1a' },
  { dot: '#5a1a7a' },
];

