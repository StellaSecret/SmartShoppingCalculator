/* ══════════════════════
   PROTEIN POWDER
══════════════════════ */
let proPowders = [];
let proId = 0;

const PRO_DEFAULTS = [
  { name: 'Brand A', price: '29.99', weight: '1000', servings: '33', protein: '25' },
  { name: 'Brand B', price: '44.99', weight: '2000', servings: '67', protein: '24' },
];

function proAdd() {
  if (proPowders.length >= 4) return;
  const id = proId++;
  const idx = proPowders.length;
  const def = PRO_DEFAULTS[idx] || {
    name: 'Brand ' + String.fromCharCode(65 + idx),
    price: '', weight: '', servings: '', protein: ''
  };
  proPowders.push({ id, idx, ...def });
  proRender();
}

function proRemove(id) {
  proPowders = proPowders.filter(p => p.id !== id);
  proPowders.forEach((p, i) => p.idx = i);
  proRender();
}

function proUpdate(id, field, value) {
  const p = proPowders.find(p => p.id === id);
  if (p) { p[field] = value; proCalc(); }
}

function proCalc() {
  proPowders.forEach(p => {
    const price = parseFloat(p.price);
    const servings = parseFloat(p.servings);
    const protein = parseFloat(p.protein);
    if (price > 0 && servings > 0 && protein > 0) {
      p.totalProtein = servings * protein;
      p.cpg = price / p.totalProtein;
      p.valid = true;
    } else {
      p.valid = false; p.cpg = null;
    }
  });
  proUpdateResults();
}

function proUpdateResults() {
  const valid = proPowders.filter(p => p.valid).sort((a, b) => a.cpg - b.cpg);
  const best = valid[0];

  proPowders.forEach(p => {
    const card = document.getElementById('procard-' + p.id);
    if (!card) return;
    card.classList.toggle('winner', p.valid && best && p.id === best.id && valid.length > 1);
    const res = document.getElementById('prores-' + p.id);
    if (!res) return;
    if (p.valid) {
      const isBest = best && p.id === best.id && valid.length > 1;
      res.innerHTML = `
        <div class="cpg" style="color:${isBest ? 'var(--win)' : 'var(--text)'}">€${p.cpg.toFixed(4)}</div>
        <div class="cpg-label">per gram of protein</div>
        <div class="extra">${p.totalProtein.toFixed(0)}g total · €${(parseFloat(p.price) / parseFloat(p.servings)).toFixed(2)}/serving</div>
        ${isBest ? '<div class="winner-badge">best value</div>' : ''}
      `;
    } else {
      res.innerHTML = '<div class="cpg-label" style="margin-top:4px">fill in all fields</div>';
    }
  });

  const summary = document.getElementById('pro-summary');
  if (valid.length < 2) {
    summary.innerHTML = '<div class="empty">add at least two valid powders to compare</div>';
    return;
  }

  const bestCpg = valid[0].cpg;
  const worstCpg = valid[valid.length - 1].cpg;
  const totalSavings = ((worstCpg - bestCpg) / worstCpg * 100).toFixed(1);

  const rows = proPowders.slice().sort((a, b) => {
    if (a.valid && b.valid) return a.cpg - b.cpg;
    if (a.valid) return -1;
    if (b.valid) return 1;
    return 0;
  }).map((p, i) => {
    if (!p.valid) {
      return `<li class="rank-item">
        <span class="rank-num">${i + 1}.</span>
        <span style="width:9px;height:9px;border-radius:50%;background:${COLORS[p.idx].dot};flex-shrink:0;display:inline-block;"></span>
        <span class="rank-name">${p.name || 'Powder ' + (p.idx + 1)}</span>
        <span class="rank-val">—</span>
        <span class="tag-more" style="opacity:0.4">incomplete</span>
      </li>`;
    }
    const more = i === 0 ? null : ((p.cpg - bestCpg) / bestCpg * 100).toFixed(1);
    return `<li class="rank-item">
      <span class="rank-num">${i + 1}.</span>
      <span style="width:9px;height:9px;border-radius:50%;background:${COLORS[p.idx].dot};flex-shrink:0;display:inline-block;"></span>
      <span class="rank-name">${p.name || 'Powder ' + (p.idx + 1)}</span>
      <span class="rank-val">€${p.cpg.toFixed(4)}/g</span>
      ${i === 0 ? '<span class="tag-best">best value</span>' : `<span class="tag-more">+${more}% more</span>`}
    </li>`;
  }).join('');

  summary.innerHTML = `
    <div class="summary-title">ranking — cost per gram of protein</div>
    <ul class="rank-list">${rows}</ul>
    <div style="margin-top:12px;font-size:0.78rem;color:var(--muted);font-family:var(--mono);">
      choosing <strong style="color:var(--text)">${valid[0].name || 'the best option'}</strong> over the most expensive saves you <strong style="color:var(--win)">${totalSavings}%</strong> per gram of protein.
    </div>
  `;
}

function proRender() {
  if (typeof _proInitOnly !== 'undefined' && _proInitOnly) return;
  const grid = document.getElementById('pro-grid');
  grid.innerHTML = '';
  const btn = document.getElementById('pro-add-btn');
  btn.style.opacity = proPowders.length >= 4 ? '0.4' : '1';
  btn.style.pointerEvents = proPowders.length >= 4 ? 'none' : 'auto';
  document.getElementById('pro-count-label').textContent =
    proPowders.length > 0 ? proPowders.length + ' / 4 powders' : '';

  proPowders.forEach(p => {
    const col = COLORS[p.idx];
    const card = document.createElement('div');
    card.className = 'item-card';
    card.id = 'procard-' + p.id;
    card.innerHTML = `
      <div class="card-header">
        <div class="card-title">
          <span class="dot" style="background:${col.dot}"></span>
          <input type="text" value="${p.name}" placeholder="Name"
            style="border:none;background:transparent;font-family:var(--mono);font-size:0.82rem;font-weight:500;color:var(--text);width:110px;outline:none;padding:0;"
            oninput="proUpdate(${p.id},'name',this.value)">
        </div>
        ${proPowders.length > 1 ? `<button class="remove-btn" onclick="proRemove(${p.id})">×</button>` : ''}
      </div>
      <div class="field"><label>Price (€)</label>
        <input type="number" value="${p.price}" min="0" step="0.01" placeholder="e.g. 29.99"
          oninput="proUpdate(${p.id},'price',this.value)"></div>
      <div class="field"><label>Bag weight (g)</label>
        <input type="number" value="${p.weight}" min="0" step="1" placeholder="e.g. 1000"
          oninput="proUpdate(${p.id},'weight',this.value)"></div>
      <div class="field"><label>Servings per bag</label>
        <input type="number" value="${p.servings}" min="1" step="1" placeholder="e.g. 33"
          oninput="proUpdate(${p.id},'servings',this.value)"></div>
      <div class="field"><label>Protein per serving (g)</label>
        <input type="number" value="${p.protein}" min="0" step="0.1" placeholder="e.g. 25"
          oninput="proUpdate(${p.id},'protein',this.value)"></div>
      <div class="card-result" id="prores-${p.id}">
        <div class="cpg-label" style="margin-top:4px">fill in all fields</div>
      </div>
    `;
    grid.appendChild(card);
  });

  proCalc();
}

