use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Page {
    Tp,
    Pro,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum TpMethod {
    Weight,
    Sheets,
    Diameter,
    Hand,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Lang {
    En,
    Fr,
}

impl Lang {
    pub fn toggled(self) -> Lang {
        match self {
            Lang::En => Lang::Fr,
            Lang::Fr => Lang::En,
        }
    }
    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Fr => "fr",
        }
    }
    pub fn from_code(s: &str) -> Option<Lang> {
        match s {
            "en" => Some(Lang::En),
            "fr" => Some(Lang::Fr),
            _ => None,
        }
    }
}

/// A single toilet-paper roll being compared.
/// All numeric fields are kept as raw `String`s (mirrors the original
/// JS app, which stores whatever the user typed and re-parses it on
/// every calculation) so partially-typed input never gets clobbered.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct TpRoll {
    pub id: u32,
    pub idx: usize,
    pub name: String,
    pub price: String,
    pub packs: String,
    pub total_w: String,
    pub tube_w: String,
    pub sheets: String,
    pub sheet_len: String,
    pub sheet_wid: String,
    pub outer: String,
    pub inner: String,
    pub width: String,
    pub h_outer: String,
    pub h_tube: String,
    pub h_width: String,
}

impl TpRoll {
    pub fn new_default(id: u32, idx: usize) -> Self {
        // Mirrors TP_DEFAULTS[0] / TP_DEFAULTS[1] in the original app,
        // falling back to an empty "Roll C" / "Roll D" style card.
        let defaults = [
            (
                "Roll A", "1.50", "1", "120", "15", "200", "113", "100", "110", "40", "100", "6",
                "1", "1",
            ),
            (
                "Roll B", "2.20", "1", "185", "15", "280", "100", "100", "130", "40", "100", "7",
                "1", "1",
            ),
        ];
        if let Some(d) = defaults.get(idx) {
            TpRoll {
                id,
                idx,
                name: d.0.to_string(),
                price: d.1.to_string(),
                packs: d.2.to_string(),
                total_w: d.3.to_string(),
                tube_w: d.4.to_string(),
                sheets: d.5.to_string(),
                sheet_len: d.6.to_string(),
                sheet_wid: d.7.to_string(),
                outer: d.8.to_string(),
                inner: d.9.to_string(),
                width: d.10.to_string(),
                h_outer: d.11.to_string(),
                h_tube: d.12.to_string(),
                h_width: d.13.to_string(),
            }
        } else {
            TpRoll {
                id,
                idx,
                name: format!("Roll {}", (b'A' + idx as u8) as char),
                price: String::new(),
                packs: "1".to_string(),
                total_w: String::new(),
                tube_w: String::new(),
                sheets: String::new(),
                sheet_len: String::new(),
                sheet_wid: String::new(),
                outer: String::new(),
                inner: String::new(),
                width: String::new(),
                h_outer: String::new(),
                h_tube: String::new(),
                h_width: String::new(),
            }
        }
    }
}

/// A single protein powder being compared.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Powder {
    pub id: u32,
    pub idx: usize,
    pub name: String,
    pub price: String,
    pub weight: String,
    pub servings: String,
    pub protein: String,
}

impl Powder {
    pub fn new_default(id: u32, idx: usize) -> Self {
        let defaults = [
            ("Brand A", "29.99", "1000", "33", "25"),
            ("Brand B", "44.99", "2000", "67", "24"),
        ];
        if let Some(d) = defaults.get(idx) {
            Powder {
                id,
                idx,
                name: d.0.to_string(),
                price: d.1.to_string(),
                weight: d.2.to_string(),
                servings: d.3.to_string(),
                protein: d.4.to_string(),
            }
        } else {
            Powder {
                id,
                idx,
                name: format!("Brand {}", (b'A' + idx as u8) as char),
                price: String::new(),
                weight: String::new(),
                servings: String::new(),
                protein: String::new(),
            }
        }
    }
}

/// The dot colours used to tell cards apart, in card order.
pub const COLORS: [&str; 4] = ["#1a5c3a", "#1a3a7a", "#7a3a1a", "#5a1a7a"];

#[derive(Clone, Serialize, Deserialize)]
pub struct TpSession {
    pub rolls: Vec<TpRoll>,
    pub method: TpMethod,
    pub next_id: u32,
    pub hand_finger: String,
    pub hand_palm: String,
    pub hand_thumb: String,
    pub usage_week: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProSession {
    pub powders: Vec<Powder>,
    pub next_id: u32,
    pub usage_week: String,
}
