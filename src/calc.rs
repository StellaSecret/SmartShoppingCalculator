use crate::models::{Powder, TpMethod, TpRoll};

/// `parseFloat(x) || 0` — never NaN, defaults to 0.
pub fn pf(s: &str) -> f64 {
    s.trim().parse::<f64>().unwrap_or(0.0)
}

/// `parseFloat(x)` kept as NaN-able via Option — used where the JS
/// checks e.g. `price > 0` on a possibly-NaN value (NaN > 0 is false,
/// which `Option::filter` reproduces naturally).
pub fn pf_opt(s: &str) -> Option<f64> {
    s.trim().parse::<f64>().ok()
}

/// Mirrors `tpfmt()`: adaptive precision so tiny per-unit prices still
/// show meaningful digits.
pub fn tp_fmt(n: f64) -> String {
    if n < 0.01 {
        format!("{:.5}", n)
    } else if n < 0.1 {
        format!("{:.4}", n)
    } else {
        format!("{:.3}", n)
    }
}

#[derive(Clone, Debug, Default)]
pub struct TpCalc {
    pub unit: Option<f64>,
    pub detail: Option<String>,
    pub unit_label: String,
    pub price_per_roll: f64,
    pub valid: bool,
}

/// Hand-calibration measurements (finger width, palm width, thumb length), all in mm.
#[derive(Clone, Copy)]
pub struct HandCal {
    pub finger: f64,
    pub palm: f64,
    pub thumb: f64,
}

impl Default for HandCal {
    fn default() -> Self {
        HandCal {
            finger: 18.0,
            palm: 85.0,
            thumb: 60.0,
        }
    }
}

/// Direct port of `tpCalc()`'s per-roll logic.
pub fn tp_calc_roll(r: &TpRoll, method: TpMethod, hand: HandCal, cur: &str, fr: bool) -> TpCalc {
    let raw_price = pf(&r.price);
    let price = if raw_price > 0.0 {
        Some(raw_price / pf(&r.packs).max(1.0))
    } else {
        None
    };

    let (unit, detail, unit_label) = match method {
        TpMethod::Weight => {
            let g = pf(&r.total_w) - pf(&r.tube_w);
            let unit = match price {
                Some(p) if g > 0.0 => Some(p / g),
                _ => None,
            };
            let detail = (g > 0.0).then(|| format!("{:.0}g {}", g, if fr { "papier" } else { "paper" }));
            (unit, detail, format!("{}/g {}", cur, if fr { "papier" } else { "paper" }))
        }
        TpMethod::Sheets => {
            let area = pf(&r.sheets) * pf(&r.sheet_len) * pf(&r.sheet_wid) / 1000.0;
            let unit = match price {
                Some(p) if area > 0.0 => Some((p / area) * 100.0),
                _ => None,
            };
            let detail = (area > 0.0).then(|| format!("{:.0} cm²", area));
            (unit, detail, format!("{}/100cm²", cur))
        }
        TpMethod::Diameter => {
            let vol = std::f64::consts::PI
                * ((pf(&r.outer) / 2.0).powi(2) - (pf(&r.inner) / 2.0).powi(2))
                * pf(&r.width)
                / 1000.0;
            let unit = match price {
                Some(p) if vol > 0.0 => Some(p / vol),
                _ => None,
            };
            let detail = (vol > 0.0).then(|| format!("{:.0} cm³", vol));
            (unit, detail, format!("{}/cm³", cur))
        }
        TpMethod::Hand => {
            let outer_est = pf(&r.h_outer) * hand.finger;
            let tube_code = pf(&r.h_tube);
            let tube_est = if tube_code == 2.0 { hand.finger * 2.0 } else { 40.0 };
            let width_code = pf(&r.h_width);
            let width_est = if width_code == 1.0 {
                hand.palm
            } else if width_code == 2.0 {
                hand.thumb
            } else {
                width_code * hand.finger
            };
            let vol = std::f64::consts::PI
                * ((outer_est / 2.0).powi(2) - (tube_est / 2.0).powi(2))
                * width_est
                / 1000.0;
            let unit = match price {
                Some(p) if vol > 0.0 => Some(p / vol),
                _ => None,
            };
            let detail = (vol > 0.0).then(|| format!("~{:.0} cm³ est.", vol));
            (unit, detail, format!("{}/cm³ est.", cur))
        }
    };

    let valid = matches!(unit, Some(v) if v.is_finite());
    TpCalc {
        unit,
        detail,
        unit_label,
        price_per_roll: price.unwrap_or(0.0),
        valid,
    }
}

#[derive(Clone, Debug, Default)]
pub struct ProCalc {
    pub total_protein: f64,
    pub cpg: f64,
    pub valid: bool,
}

/// Direct port of `proCalc()`'s per-powder logic.
pub fn pro_calc_powder(p: &Powder) -> ProCalc {
    let price = pf_opt(&p.price);
    let servings = pf_opt(&p.servings);
    let protein = pf_opt(&p.protein);
    match (price, servings, protein) {
        (Some(price), Some(servings), Some(protein)) if price > 0.0 && servings > 0.0 && protein > 0.0 => {
            let total_protein = servings * protein;
            ProCalc {
                total_protein,
                cpg: price / total_protein,
                valid: true,
            }
        }
        _ => ProCalc::default(),
    }
}
