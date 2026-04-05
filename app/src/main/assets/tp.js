/* ══════════════════════
   TOILET PAPER
══════════════════════ */
let tpMethod = 'weight';
let tpRolls = [];
let tpId = 0;

const TP_DEFAULTS = [
  { name: 'Roll A', price: '1.50', packs: '1', totalW: '120', tubeW: '15', sheets: '200', sheetLen: '113', sheetWid: '100', outer: '110', inner: '40', width: '100', hOuter: '6', hTube: '1', hWidth: '1' },
  { name: 'Roll B', price: '2.20', packs: '1', totalW: '185', tubeW: '15', sheets: '280', sheetLen: '100', sheetWid: '100', outer: '130', inner: '40', width: '100', hOuter: '7', hTube: '1', hWidth: '1' },
];

function setTpMethod(m, btn) {
  tpMethod = m;
  document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
  btn.classList.add('active');
  const cal = document.getElementById('hand-calibration');
  if (cal) cal.style.display = m === 'hand' ? 'block' : 'none';
  // Re-render cards so only the active method's inputs exist in the DOM
  tpRender();
}

function tpAdd() {
  if (tpRolls.length >= 4) return;
  const id = tpId++;
  const idx = tpRolls.length;
  const def = TP_DEFAULTS[idx] || {
    name: 'Roll ' + String.fromCharCode(65 + idx),
    price: '', packs: '1', totalW: '', tubeW: '',
    sheets: '', sheetLen: '', sheetWid: '',
    outer: '', inner: '', width: '',
    hOuter: '', hTube: '', hWidth: ''
  };
  tpRolls.push({ id, idx, ...def });
  tpRender();
}

function tpRemove(id) {
  tpRolls = tpRolls.filter(r => r.id !== id);
  tpRolls.forEach((r, i) => r.idx = i);
  tpRender();
}

function tpUpdate(id, field, value) {
  const r = tpRolls.find(r => r.id === id);
  if (r) { r[field] = value; tpCalc(); }
}

function tpfmt(n) { return n < 0.01 ? n.toFixed(5) : n < 0.1 ? n.toFixed(4) : n.toFixed(3); }
function rf(r, f) { return parseFloat(r[f]) || 0; }

function tpCalc() {
  tpRolls.forEach(r => {
    const rawPrice = rf(r, 'price');
    const price = rawPrice > 0 ? rawPrice / Math.max(1, rf(r, 'packs')) : null;
    if (tpMethod === 'weight') {
      const g = rf(r, 'totalW') - rf(r, 'tubeW');
      r.unit = (price !== null && g > 0) ? price / g : null;
      r.detail = g > 0 ? g.toFixed(0) + 'g paper' : null;
      r.unitLabel = '€/g paper';
    } else if (tpMethod === 'sheets') {
      const area = rf(r, 'sheets') * rf(r, 'sheetLen') * rf(r, 'sheetWid') / 1000;
      r.unit = (price !== null && area > 0) ? (price / area) * 100 : null;
      r.detail = area > 0 ? area.toFixed(0) + ' cm²' : null;
      r.unitLabel = '€/100cm²';
    } else if (tpMethod === 'diameter') {
      const vol = Math.PI * ((rf(r, 'outer') / 2) ** 2 - (rf(r, 'inner') / 2) ** 2) * rf(r, 'width') / 1000;
      r.unit = (price !== null && vol > 0) ? price / vol : null;
      r.detail = vol > 0 ? vol.toFixed(0) + ' cm³' : null;
      r.unitLabel = '€/cm³';
    } else {
      // hand estimation method
      const fingerW = parseFloat(document.getElementById('hand-finger')?.value) || 18;
      const palmW   = parseFloat(document.getElementById('hand-palm')?.value)   || 85;
      const thumbL  = parseFloat(document.getElementById('hand-thumb')?.value)  || 60;
      const outerEst = rf(r, 'hOuter') * fingerW;
      const tubeEst  = rf(r, 'hTube') === 2 ? fingerW * 2 : 40;
      const widthCode = rf(r, 'hWidth');
      const widthEst  = widthCode === 1 ? palmW : widthCode === 2 ? thumbL : widthCode * fingerW;
      const vol = Math.PI * ((outerEst / 2) ** 2 - (tubeEst / 2) ** 2) * widthEst / 1000;
      r.unit = (price !== null && vol > 0) ? price / vol : null;
      r.detail = vol > 0 ? '~' + vol.toFixed(0) + ' cm³ est.' : null;
      r.unitLabel = '€/cm³ est.';
    }
    r.pricePerRoll = price ?? 0;
    r.valid = r.unit !== null && !isNaN(r.unit);
  });
  tpUpdateResults();
}

function tpUpdateResults() {
  const valid = tpRolls.filter(r => r.valid).sort((a, b) => a.unit - b.unit);
  const best = valid[0];

  tpRolls.forEach(r => {
    const card = document.getElementById('tpcard-' + r.id);
    if (!card) return;
    card.classList.toggle('winner', r.valid && best && r.id === best.id && valid.length > 1);
    const res = document.getElementById('tpres-' + r.id);
    if (!res) return;
    if (r.valid) {
      const isBest = best && r.id === best.id && valid.length > 1;
      res.innerHTML = `
        <div class="cpg" style="color:${isBest ? 'var(--win)' : 'var(--text)'}">${tpfmt(r.unit)}</div>
        <div class="cpg-label">${r.unitLabel}</div>
        <div class="extra">${r.detail} · €${r.pricePerRoll.toFixed(2)}/roll</div>
        ${isBest ? '<div class="winner-badge">best value</div>' : ''}
      `;
    } else {
      res.innerHTML = '<div class="cpg-label" style="margin-top:4px">fill in all fields</div>';
    }
  });

  const el = document.getElementById('tp-results');
  if (valid.length < 2) {
    el.innerHTML = '<div class="empty">add at least two valid rolls to compare</div>';
    return;
  }

  const bestVal = valid[0].unit;
  const worstVal = valid[valid.length - 1].unit;
  const totalSavings = ((worstVal - bestVal) / worstVal * 100).toFixed(1);
  const methodLabel = tpMethod === 'weight' ? 'cost per gram of paper' : tpMethod === 'sheets' ? 'cost per 100cm²' : tpMethod === 'diameter' ? 'cost per cm³' : 'estimated cost per cm³ (hand method)';

  const rows = valid.map((r, i) => {
    const more = i === 0 ? null : ((r.unit - bestVal) / bestVal * 100).toFixed(1);
    return `<li class="rank-item">
      <span class="rank-num">${i + 1}.</span>
      <span style="width:9px;height:9px;border-radius:50%;background:${COLORS[r.idx].dot};flex-shrink:0;display:inline-block;"></span>
      <span class="rank-name">${r.name || 'Roll ' + (r.idx + 1)}</span>
      <span class="rank-val">${tpfmt(r.unit)} ${r.unitLabel}</span>
      ${i === 0 ? '<span class="tag-best">best value</span>' : `<span class="tag-more">+${more}% more</span>`}
    </li>`;
  }).join('');

  el.innerHTML = `
    <div class="summary-title">ranking — ${methodLabel}</div>
    <ul class="rank-list">${rows}</ul>
    <div style="margin-top:12px;font-size:0.78rem;color:var(--muted);font-family:var(--mono);">
      choosing <strong style="color:var(--text)">${valid[0].name || 'the best option'}</strong> over the most expensive saves you <strong style="color:var(--win)">${totalSavings}%</strong>.
    </div>
  `;
}

function tpRender() {
  const grid = document.getElementById('tp-grid');
  grid.innerHTML = '';
  const btn = document.getElementById('tp-add-btn');
  btn.style.opacity = tpRolls.length >= 4 ? '0.4' : '1';
  btn.style.pointerEvents = tpRolls.length >= 4 ? 'none' : 'auto';
  document.getElementById('tp-count-label').textContent = tpRolls.length > 0 ? tpRolls.length + ' / 4 rolls' : '';

  tpRolls.forEach(r => {
    const col = COLORS[r.idx];
    const card = document.createElement('div');
    card.className = 'item-card';
    card.id = 'tpcard-' + r.id;
    card.innerHTML = `
      <div class="card-header">
        <div class="card-title">
          <span class="dot" style="background:${col.dot}"></span>
          <input type="text" value="${r.name}" placeholder="Name"
            style="border:none;background:transparent;font-family:var(--mono);font-size:0.82rem;font-weight:500;color:var(--text);width:100px;outline:none;padding:0;"
            oninput="tpUpdate(${r.id},'name',this.value)">
        </div>
        ${tpRolls.length > 1 ? `<button class="remove-btn" onclick="tpRemove(${r.id})">×</button>` : ''}
      </div>
      <div class="field"><label>Price (€)</label>
        <input type="number" value="${r.price}" min="0" step="0.01" placeholder="e.g. 1.99"
          oninput="tpUpdate(${r.id},'price',this.value)"></div>
      <div class="field"><label>Rolls in pack</label>
        <input type="number" value="${r.packs}" min="1" step="1" placeholder="e.g. 4"
          oninput="tpUpdate(${r.id},'packs',this.value)"></div>
      <div class="divider"></div>

      <div class="tp-section ${tpMethod === 'weight' ? 'visible' : ''}" id="tpsec-weight-${r.id}">
        ${tpMethod === 'weight' ? `
        <div class="field"><label>Total roll weight (g)</label>
          <input type="number" value="${r.totalW}" min="0" placeholder="e.g. 120"
            oninput="tpUpdate(${r.id},'totalW',this.value)"></div>
        <div class="field"><label>Tube weight (g)</label>
          <input type="number" value="${r.tubeW}" min="0" placeholder="e.g. 15"
            oninput="tpUpdate(${r.id},'tubeW',this.value)"></div>
        ` : ''}
      </div>

      <div class="tp-section ${tpMethod === 'sheets' ? 'visible' : ''}" id="tpsec-sheets-${r.id}">
        ${tpMethod === 'sheets' ? `
        <div class="field"><label>Sheet count</label>
          <input type="number" value="${r.sheets}" min="1" placeholder="e.g. 200"
            oninput="tpUpdate(${r.id},'sheets',this.value)"></div>
        <div class="field"><label>Sheet length (mm)</label>
          <input type="number" value="${r.sheetLen}" min="1" placeholder="e.g. 113"
            oninput="tpUpdate(${r.id},'sheetLen',this.value)"></div>
        <div class="field"><label>Sheet width (mm)</label>
          <input type="number" value="${r.sheetWid}" min="1" placeholder="e.g. 100"
            oninput="tpUpdate(${r.id},'sheetWid',this.value)"></div>
        ` : ''}
      </div>

      <div class="tp-section ${tpMethod === 'diameter' ? 'visible' : ''}" id="tpsec-diameter-${r.id}">
        ${tpMethod === 'diameter' ? `
        <div class="field"><label>Outer diameter (mm)</label>
          <input type="number" value="${r.outer}" min="1" placeholder="e.g. 110"
            oninput="tpUpdate(${r.id},'outer',this.value)"></div>
        <div class="field"><label>Tube diameter (mm)</label>
          <input type="number" value="${r.inner}" min="1" placeholder="e.g. 40"
            oninput="tpUpdate(${r.id},'inner',this.value)"></div>
        <div class="field"><label>Roll width (mm)</label>
          <input type="number" value="${r.width}" min="1" placeholder="e.g. 100"
            oninput="tpUpdate(${r.id},'width',this.value)"></div>
        ` : ''}
      </div>

      <div class="tp-section ${tpMethod === 'hand' ? 'visible' : ''}" id="tpsec-hand-${r.id}">
        ${tpMethod === 'hand' ? `
        <div class="field">
          <label>roll diameter — finger-widths across</label>
          <input type="number" value="${r.hOuter||''}" min="1" max="15" step="0.5" placeholder="e.g. 6"
            oninput="tpUpdate(${r.id},'hOuter',this.value)">
        </div>
        <div class="field">
          <label>tube size</label>
          <select oninput="tpUpdate(${r.id},'hTube',this.value)"
            style="width:100%;padding:7px 10px;font-family:var(--mono);font-size:0.83rem;border:1px solid var(--border);border-radius:4px;background:var(--bg);color:var(--text);outline:none;">
            <option value="1" ${rf(r,'hTube')===1||!r.hTube?'selected':''}>fits 1 finger (~40mm, standard)</option>
            <option value="2" ${rf(r,'hTube')===2?'selected':''}>fits 2 fingers (~36mm, compact)</option>
          </select>
        </div>
        <div class="field">
          <label>roll width</label>
          <select oninput="tpUpdate(${r.id},'hWidth',this.value)"
            style="width:100%;padding:7px 10px;font-family:var(--mono);font-size:0.83rem;border:1px solid var(--border);border-radius:4px;background:var(--bg);color:var(--text);outline:none;">
            <option value="1" ${rf(r,'hWidth')===1||!r.hWidth?'selected':''}>about 1 palm wide (~85mm)</option>
            <option value="2" ${rf(r,'hWidth')===2?'selected':''}>about 1 thumb length (~60mm)</option>
            <option value="5" ${rf(r,'hWidth')===5?'selected':''}>about 5 finger-widths</option>
            <option value="6" ${rf(r,'hWidth')===6?'selected':''}>about 6 finger-widths</option>
          </select>
        </div>
        <div style="font-size:0.68rem;color:var(--muted);font-family:var(--mono);margin-top:4px;line-height:1.4;">
          estimate how many fingers wide the roll is when looking at it straight on
        </div>
        ` : ''}
      </div>

      <div class="card-result" id="tpres-${r.id}">
        <div class="cpg-label" style="margin-top:4px">fill in all fields</div>
      </div>
    `;
    grid.appendChild(card);
  });

  tpCalc();
}

