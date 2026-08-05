use dioxus::prelude::*;

use smart_shopping_calculator_core::calc::{pf, pro_calc_powder, tp_calc_roll, tp_fmt, HandCal, ProCalc, TpCalc};
use smart_shopping_calculator_core::models::{Lang, Powder, ProSession, TpMethod, TpRoll, TpSession, COLORS};
use crate::css::CSS;
use crate::i18n::{card_price_label, choosing_saves, count_powders, count_rolls, lifetime_line, s};
use crate::storage;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Page {
    Tp,
    Pro,
}

#[derive(Clone, Copy)]
pub struct AppState {
    pub lang: Signal<Lang>,
    pub dark: Signal<bool>,
    pub currency: Signal<String>,
    pub tp_method: Signal<TpMethod>,
    pub tp_rolls: Signal<Vec<TpRoll>>,
    pub tp_next_id: Signal<u32>,
    pub tp_usage_week: Signal<String>,
    pub hand_finger: Signal<String>,
    pub hand_palm: Signal<String>,
    pub hand_thumb: Signal<String>,
    pub tp_has_save: Signal<bool>,
    pub tp_toast: Signal<String>,
    pub pro_powders: Signal<Vec<Powder>>,
    pub pro_next_id: Signal<u32>,
    pub pro_usage_week: Signal<String>,
    pub pro_has_save: Signal<bool>,
    pub pro_toast: Signal<String>,
}

/// Precomputed row for the toilet-paper ranking list (rsx `for` loops
/// can't declare `let` bindings, so all per-item values are computed here).
struct TpRank {
    id: u32,
    idx: usize,
    rank: usize,
    name: String,
    unit_str: String,
    label: String,
    bar_pct: String,
    more: String,
    bar_color: String,
    opacity: &'static str,
    is_best: bool,
}

/// Precomputed row for the protein ranking list.
struct ProRank {
    id: u32,
    idx: usize,
    rank: usize,
    name: String,
    valid: bool,
    cpg_str: String,
    bar_pct: String,
    more: String,
    bar_color: String,
    opacity: &'static str,
    is_best: bool,
}

#[component]
pub fn App() -> Element {
    let mut page = use_signal(|| Page::Tp);
    let lang = use_signal(storage::load_lang_pref);
    let dark = use_signal(storage::load_dark_pref);
    let currency = use_signal(storage::load_currency_pref);
    let tp_method = use_signal(|| TpMethod::Weight);
    let tp_rolls = use_signal(|| vec![TpRoll::new_default(0, 0), TpRoll::new_default(1, 1)]);
    let tp_next_id = use_signal(|| 2u32);
    let tp_usage_week = use_signal(|| "1".to_string());
    let hand_finger = use_signal(|| "18".to_string());
    let hand_palm = use_signal(|| "85".to_string());
    let hand_thumb = use_signal(|| "60".to_string());
    let tp_has_save = use_signal(storage::has_tp_session);
    let tp_toast = use_signal(String::new);
    let pro_powders = use_signal(|| vec![Powder::new_default(0, 0), Powder::new_default(1, 1)]);
    let pro_next_id = use_signal(|| 2u32);
    let pro_usage_week = use_signal(|| "5".to_string());
    let pro_has_save = use_signal(storage::has_pro_session);
    let pro_toast = use_signal(String::new);

    let state = AppState {
        lang,
        dark,
        currency,
        tp_method,
        tp_rolls,
        tp_next_id,
        tp_usage_week,
        hand_finger,
        hand_palm,
        hand_thumb,
        tp_has_save,
        tp_toast,
        pro_powders,
        pro_next_id,
        pro_usage_week,
        pro_has_save,
        pro_toast,
    };
    use_context_provider(|| state);

    let lg = *lang.read();
    let str_ = s(lg);
    let root_class = if *dark.read() { "app dark" } else { "app" };
    let page_val = *page.read();

    rsx! {
        style { "{CSS}" }
        link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=DM+Mono:wght@400;500&family=DM+Sans:wght@300;400;500&display=swap",
        }
        div { class: "{root_class}",
            div { class: "container",
                Header {}
                nav { class: "nav-tabs",
                    button {
                        class: if page_val == Page::Tp { "nav-tab active" } else { "nav-tab" },
                        onclick: move |_| page.set(Page::Tp),
                        "{str_.tab_tp}"
                    }
                    button {
                        class: if page_val == Page::Pro { "nav-tab active" } else { "nav-tab" },
                        onclick: move |_| page.set(Page::Pro),
                        "{str_.tab_pro}"
                    }
                }
                if page_val == Page::Tp {
                    TpPage {}
                } else {
                    ProPage {}
                }
                footer { "{str_.footer}" }
            }
        }
    }
}

#[component]
fn Header() -> Element {
    let mut state = use_context::<AppState>();
    let lang_val = *state.lang.read();
    let dark_val = *state.dark.read();
    let cur_val = state.currency.read().clone();
    let str_ = s(lang_val);

    rsx! {
        header {
            div { class: "header-row",
                div {
                    h1 { "🛒 smart shopping calculator" }
                    p { "{str_.subtitle}" }
                }
                div { class: "header-controls",
                    button {
                        class: "pill-btn",
                        onclick: move |_| {
                            let new_lang = state.lang.read().toggled();
                            state.lang.set(new_lang);
                            storage::save_lang_pref(new_lang);
                        },
                        if lang_val == Lang::En { "FR" } else { "EN" }
                    }
                    select {
                        class: "currency-select",
                        value: "{cur_val}",
                        onchange: move |evt| {
                            let v = evt.value();
                            storage::save_currency_pref(&v);
                            state.currency.set(v);
                        },
                        option { value: "€", "€ EUR" }
                        option { value: "$", "$ USD" }
                        option { value: "£", "£ GBP" }
                        option { value: "CHF", "CHF" }
                        option { value: "¥", "¥ JPY" }
                        option { value: "C$", "C$ CAD" }
                        option { value: "A$", "A$ AUD" }
                    }
                    button {
                        class: "pill-btn",
                        onclick: move |_| {
                            let new_dark = !*state.dark.read();
                            state.dark.set(new_dark);
                            storage::save_dark_pref(new_dark);
                        },
                        if dark_val { "{str_.light_btn}" } else { "{str_.dark_btn}" }
                    }
                }
            }
        }
    }
}

#[component]
fn TpPage() -> Element {
    let mut state = use_context::<AppState>();
    let lg = *state.lang.read();
    let str_ = s(lg);
    let cur = state.currency.read().clone();
    let fr = lg == Lang::Fr;
    let method = *state.tp_method.read();
    let rolls_owned: Vec<TpRoll> = state.tp_rolls.read().clone();

    let hand_finger_val = state.hand_finger.read().clone();
    let hand_palm_val = state.hand_palm.read().clone();
    let hand_thumb_val = state.hand_thumb.read().clone();
    let hand = HandCal {
        finger: pf(&hand_finger_val),
        palm: pf(&hand_palm_val),
        thumb: pf(&hand_thumb_val),
    };

    let calced: Vec<(TpRoll, TpCalc)> = rolls_owned
        .iter()
        .cloned()
        .map(|r| {
            let c = tp_calc_roll(&r, method, hand, &cur, fr);
            (r, c)
        })
        .collect();

    let mut valid: Vec<(TpRoll, TpCalc)> = calced.iter().filter(|(_, c)| c.valid).cloned().collect();
    valid.sort_by(|a, b| a.1.unit.unwrap().partial_cmp(&b.1.unit.unwrap()).unwrap());
    let best_id = valid.first().map(|(r, _)| r.id);
    let worst = valid.last().cloned();
    let valid_len = valid.len();

    let can_add = rolls_owned.len() < 4;
    let has_save = *state.tp_has_save.read();
    let toast = state.tp_toast.read().clone();
    let roll_count = rolls_owned.len();

    rsx! {
        div { class: "page visible",
            div { class: "top-controls",
                button {
                    class: if can_add { "add-btn" } else { "add-btn disabled" },
                    onclick: move |_| {
                        if state.tp_rolls.read().len() < 4 {
                            let id = *state.tp_next_id.read();
                            let idx = state.tp_rolls.read().len();
                            state.tp_next_id.set(id + 1);
                            state.tp_rolls.write().push(TpRoll::new_default(id, idx));
                        }
                    },
                    "{str_.add_roll}"
                }
                label {
                    if roll_count > 0 {
                        "{count_rolls(lg, roll_count)}"
                    }
                }
            }

            div { class: "session-bar",
                button {
                    class: "session-btn",
                    title: "{str_.save_session_title}",
                    onclick: move |_| {
                        let session = TpSession {
                            rolls: state.tp_rolls.read().clone(),
                            method: *state.tp_method.read(),
                            next_id: *state.tp_next_id.read(),
                            hand_finger: state.hand_finger.read().clone(),
                            hand_palm: state.hand_palm.read().clone(),
                            hand_thumb: state.hand_thumb.read().clone(),
                            usage_week: state.tp_usage_week.read().clone(),
                        };
                        storage::save_tp_session(&session);
                        state.tp_has_save.set(true);
                        state.tp_toast.set(s(*state.lang.read()).toast_saved.to_string());
                    },
                    "{str_.save_session}"
                }
                button {
                    class: if has_save { "session-btn has-save" } else { "session-btn" },
                    title: "{str_.restore_title}",
                    onclick: move |_| {
                        if let Some(session) = storage::load_tp_session() {
                            state.tp_rolls.set(session.rolls);
                            state.tp_method.set(session.method);
                            state.tp_next_id.set(session.next_id);
                            state.hand_finger.set(session.hand_finger);
                            state.hand_palm.set(session.hand_palm);
                            state.hand_thumb.set(session.hand_thumb);
                            state.tp_usage_week.set(session.usage_week);
                            state.tp_toast.set(s(*state.lang.read()).toast_restored.to_string());
                        } else {
                            state.tp_toast.set(s(*state.lang.read()).toast_restore_fail.to_string());
                        }
                    },
                    "{str_.restore_session}"
                }
                button {
                    class: "session-btn",
                    title: "{str_.clear_title}",
                    onclick: move |_| {
                        storage::clear_tp_session();
                        state.tp_has_save.set(false);
                        state.tp_toast.set(s(*state.lang.read()).toast_cleared.to_string());
                    },
                    "{str_.clear_saved}"
                }
                span { class: if toast.is_empty() { "session-toast" } else { "session-toast show" }, "{toast}" }
            }

            div { class: "method-tabs",
                button {
                    class: if method == TpMethod::Weight { "tab-btn active" } else { "tab-btn" },
                    onclick: move |_| state.tp_method.set(TpMethod::Weight),
                    "{str_.by_weight}"
                }
                button {
                    class: if method == TpMethod::Sheets { "tab-btn active" } else { "tab-btn" },
                    onclick: move |_| state.tp_method.set(TpMethod::Sheets),
                    "{str_.by_sheets}"
                }
                button {
                    class: if method == TpMethod::Diameter { "tab-btn active" } else { "tab-btn" },
                    onclick: move |_| state.tp_method.set(TpMethod::Diameter),
                    "{str_.by_diameter}"
                }
                button {
                    class: if method == TpMethod::Hand { "tab-btn active" } else { "tab-btn" },
                    onclick: move |_| state.tp_method.set(TpMethod::Hand),
                    "{str_.by_hand}"
                }
            }

            if method == TpMethod::Hand {
                div { style: "background:var(--surface);border:1px solid var(--border);border-radius:8px;padding:1rem 1.25rem;margin-bottom:1.5rem;",
                    div { style: "font-family:var(--mono);font-size:0.78rem;color:var(--muted);margin-bottom:0.75rem;",
                        "{str_.hand_title} "
                        span { style: "font-size:0.68rem;", "{str_.hand_subtitle}" }
                    }
                    div { style: "display:grid;grid-template-columns:1fr 1fr 1fr;gap:10px;",
                        div { class: "field",
                            label { "{str_.hand_finger}" }
                            input {
                                r#type: "number",
                                value: "{hand_finger_val}",
                                min: "10",
                                max: "30",
                                step: "1",
                                oninput: move |evt| state.hand_finger.set(evt.value()),
                            }
                        }
                        div { class: "field",
                            label { "{str_.hand_palm}" }
                            input {
                                r#type: "number",
                                value: "{hand_palm_val}",
                                min: "50",
                                max: "130",
                                step: "1",
                                oninput: move |evt| state.hand_palm.set(evt.value()),
                            }
                        }
                        div { class: "field",
                            label { "{str_.hand_thumb}" }
                            input {
                                r#type: "number",
                                value: "{hand_thumb_val}",
                                min: "40",
                                max: "90",
                                step: "1",
                                oninput: move |evt| state.hand_thumb.set(evt.value()),
                            }
                        }
                    }
                    div { style: "font-size:0.7rem;color:var(--muted);font-family:var(--mono);margin-top:6px;", "{str_.hand_tip}" }
                }
            }

            div { class: "cards-grid",
                for r in rolls_owned.iter().cloned() {
                    { let id = r.id; tp_card(r, method, cur.clone(), lg, Some(id) == best_id && valid_len > 1, roll_count > 1, state.tp_rolls, state.hand_finger, state.hand_palm, state.hand_thumb) }
                }
            }

            {tp_results(valid, worst, method, cur.clone(), lg, state.tp_usage_week)}
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn tp_card(
    r: TpRoll,
    method: TpMethod,
    cur: String,
    lang: Lang,
    is_winner: bool,
    removable: bool,
    mut tp_rolls: Signal<Vec<TpRoll>>,
    hand_finger: Signal<String>,
    hand_palm: Signal<String>,
    hand_thumb: Signal<String>,
) -> Element {
    let id = r.id;
    let str_ = s(lang);
    let col = COLORS[r.idx % 4];
    let hand = HandCal {
        finger: pf(&hand_finger.read()),
        palm: pf(&hand_palm.read()),
        thumb: pf(&hand_thumb.read()),
    };
    let fr = lang == Lang::Fr;
    let calc = tp_calc_roll(&r, method, hand, &cur, fr);
    let price_fmt = format!("{:.2}", calc.price_per_roll);
    let detail = calc.detail.clone().unwrap_or_default();
    let unit_label = calc.unit_label.clone();
    let price_label = card_price_label(lang, &cur);

    let h_tube_val = if r.h_tube.is_empty() { "1".to_string() } else { r.h_tube.clone() };
    let h_width_val = if r.h_width.is_empty() { "1".to_string() } else { r.h_width.clone() };

    rsx! {
        div { class: if is_winner { "item-card winner" } else { "item-card" }, key: "{id}",
            div { class: "card-header",
                div { class: "card-title",
                    span { class: "dot", style: "background:{col}" }
                    input {
                        class: "name-input",
                        r#type: "text",
                        value: "{r.name}",
                        placeholder: "{str_.card_name}",
                        oninput: move |evt| {
                            let v = evt.value();
                            if let Some(x) = tp_rolls.write().iter_mut().find(|x| x.id == id) {
                                x.name = v;
                            }
                        },
                    }
                }
                if removable {
                    button {
                        class: "remove-btn",
                        onclick: move |_| {
                            let mut rolls = tp_rolls.write();
                            rolls.retain(|x| x.id != id);
                            for (i, x) in rolls.iter_mut().enumerate() {
                                x.idx = i;
                            }
                        },
                        "×"
                    }
                }
            }
            div { class: "field",
                label { "{price_label}" }
                input {
                    r#type: "number",
                    value: "{r.price}",
                    min: "0",
                    step: "0.01",
                    placeholder: "e.g. 1.99",
                    oninput: move |evt| {
                        let v = evt.value();
                        if let Some(x) = tp_rolls.write().iter_mut().find(|x| x.id == id) {
                            x.price = v;
                        }
                    },
                }
            }
            div { class: "field",
                label { "{str_.rolls_in_pack}" }
                input {
                    r#type: "number",
                    value: "{r.packs}",
                    min: "1",
                    step: "1",
                    placeholder: "e.g. 4",
                    oninput: move |evt| {
                        let v = evt.value();
                        if let Some(x) = tp_rolls.write().iter_mut().find(|x| x.id == id) {
                            x.packs = v;
                        }
                    },
                }
            }
            div { class: "divider" }

            if method == TpMethod::Weight {
                div { class: "field",
                    label { "{str_.total_weight}" }
                    input {
                        r#type: "number",
                        value: "{r.total_w}",
                        min: "0",
                        placeholder: "e.g. 120",
                        oninput: move |evt| {
                            let v = evt.value();
                            if let Some(x) = tp_rolls.write().iter_mut().find(|x| x.id == id) {
                                x.total_w = v;
                            }
                        },
                    }
                }
                div { class: "field",
                    label { "{str_.tube_weight}" }
                    input {
                        r#type: "number",
                        value: "{r.tube_w}",
                        min: "0",
                        placeholder: "e.g. 15",
                        oninput: move |evt| {
                            let v = evt.value();
                            if let Some(x) = tp_rolls.write().iter_mut().find(|x| x.id == id) {
                                x.tube_w = v;
                            }
                        },
                    }
                }
            } else if method == TpMethod::Sheets {
                div { class: "field",
                    label { "{str_.sheet_count}" }
                    input {
                        r#type: "number",
                        value: "{r.sheets}",
                        min: "1",
                        placeholder: "e.g. 200",
                        oninput: move |evt| {
                            let v = evt.value();
                            if let Some(x) = tp_rolls.write().iter_mut().find(|x| x.id == id) {
                                x.sheets = v;
                            }
                        },
                    }
                }
                div { class: "field",
                    label { "{str_.sheet_length}" }
                    input {
                        r#type: "number",
                        value: "{r.sheet_len}",
                        min: "1",
                        placeholder: "e.g. 113",
                        oninput: move |evt| {
                            let v = evt.value();
                            if let Some(x) = tp_rolls.write().iter_mut().find(|x| x.id == id) {
                                x.sheet_len = v;
                            }
                        },
                    }
                }
                div { class: "field",
                    label { "{str_.sheet_width}" }
                    input {
                        r#type: "number",
                        value: "{r.sheet_wid}",
                        min: "1",
                        placeholder: "e.g. 100",
                        oninput: move |evt| {
                            let v = evt.value();
                            if let Some(x) = tp_rolls.write().iter_mut().find(|x| x.id == id) {
                                x.sheet_wid = v;
                            }
                        },
                    }
                }
            } else if method == TpMethod::Diameter {
                div { class: "field",
                    label { "{str_.outer_diam}" }
                    input {
                        r#type: "number",
                        value: "{r.outer}",
                        min: "1",
                        placeholder: "e.g. 110",
                        oninput: move |evt| {
                            let v = evt.value();
                            if let Some(x) = tp_rolls.write().iter_mut().find(|x| x.id == id) {
                                x.outer = v;
                            }
                        },
                    }
                }
                div { class: "field",
                    label { "{str_.tube_diam}" }
                    input {
                        r#type: "number",
                        value: "{r.inner}",
                        min: "1",
                        placeholder: "e.g. 40",
                        oninput: move |evt| {
                            let v = evt.value();
                            if let Some(x) = tp_rolls.write().iter_mut().find(|x| x.id == id) {
                                x.inner = v;
                            }
                        },
                    }
                }
                div { class: "field",
                    label { "{str_.roll_width}" }
                    input {
                        r#type: "number",
                        value: "{r.width}",
                        min: "1",
                        placeholder: "e.g. 100",
                        oninput: move |evt| {
                            let v = evt.value();
                            if let Some(x) = tp_rolls.write().iter_mut().find(|x| x.id == id) {
                                x.width = v;
                            }
                        },
                    }
                }
            } else {
                div { class: "field",
                    label { "{str_.hand_roll_diam}" }
                    input {
                        r#type: "number",
                        value: "{r.h_outer}",
                        min: "1",
                        max: "15",
                        step: "0.5",
                        placeholder: "e.g. 6",
                        oninput: move |evt| {
                            let v = evt.value();
                            if let Some(x) = tp_rolls.write().iter_mut().find(|x| x.id == id) {
                                x.h_outer = v;
                            }
                        },
                    }
                }
                div { class: "field",
                    label { "{str_.hand_tube_size}" }
                    select {
                        value: "{h_tube_val}",
                        onchange: move |evt| {
                            let v = evt.value();
                            if let Some(x) = tp_rolls.write().iter_mut().find(|x| x.id == id) {
                                x.h_tube = v;
                            }
                        },
                        option { value: "1", "{str_.hand_tube_opt1}" }
                        option { value: "2", "{str_.hand_tube_opt2}" }
                    }
                }
                div { class: "field",
                    label { "{str_.hand_roll_width}" }
                    select {
                        value: "{h_width_val}",
                        onchange: move |evt| {
                            let v = evt.value();
                            if let Some(x) = tp_rolls.write().iter_mut().find(|x| x.id == id) {
                                x.h_width = v;
                            }
                        },
                        option { value: "1", "{str_.hand_width_opt1}" }
                        option { value: "2", "{str_.hand_width_opt2}" }
                        option { value: "5", "{str_.hand_width_opt3}" }
                        option { value: "6", "{str_.hand_width_opt4}" }
                    }
                }
                div { style: "font-size:0.68rem;color:var(--muted);font-family:var(--mono);margin-top:4px;line-height:1.4;",
                    "{str_.hand_est_tip}"
                }
            }

            div { class: "card-result",
                if calc.valid {
                    div {
                        class: "cpg",
                        style: if is_winner { "color:var(--win)" } else { "color:var(--text)" },
                        "{tp_fmt(calc.unit.unwrap())}"
                    }
                    div { class: "cpg-label", "{unit_label}" }
                    div { class: "extra", "{detail} · {cur}{price_fmt}{str_.per_roll}" }
                    if is_winner {
                        div { class: "winner-badge", "{str_.best_value}" }
                    }
                } else {
                    div { class: "cpg-label", style: "margin-top:4px", "{str_.fill_fields}" }
                }
            }
        }
    }
}

fn tp_results(
    valid: Vec<(TpRoll, TpCalc)>,
    worst: Option<(TpRoll, TpCalc)>,
    method: TpMethod,
    cur: String,
    lang: Lang,
    mut tp_usage_week: Signal<String>,
) -> Element {
    let str_ = s(lang);

    if valid.len() < 2 {
        return rsx! {
            div { class: "results-summary",
                div { class: "empty", "{str_.empty_tp_valid}" }
            }
        };
    }

    let best_val = valid[0].1.unit.unwrap();
    let worst_val = worst.as_ref().unwrap().1.unit.unwrap();
    let savings = format!("{:.1}", (worst_val - best_val) / worst_val * 100.0);
    let method_label = match method {
        TpMethod::Weight => str_.method_weight,
        TpMethod::Sheets => str_.method_sheets,
        TpMethod::Diameter => str_.method_diam,
        TpMethod::Hand => str_.method_hand,
    };

    let best_name = if valid[0].0.name.is_empty() {
        str_.best_value.to_string()
    } else {
        valid[0].0.name.clone()
    };
    let choosing_html = choosing_saves(lang, &best_name, &savings, false);

    let usage_week_val = tp_usage_week.read().clone();
    let usage_week_num = pf(&usage_week_val);
    let lifetime_html = if usage_week_num > 0.0 {
        let rolls_per_year = usage_week_num * 52.0;
        let cost_best = rolls_per_year * valid[0].1.price_per_roll;
        let cost_worst = rolls_per_year * worst.as_ref().unwrap().1.price_per_roll;
        let save_year = format!("{:.2}", cost_worst - cost_best);
        Some(lifetime_line(
            lang,
            usage_week_num,
            "roll",
            "rolls",
            "rouleau",
            "rouleaux",
            &format!("{:.0}", cost_best),
            &format!("{:.0}", cost_worst),
            &save_year,
            &cur,
        ))
    } else {
        None
    };

    let ranks: Vec<TpRank> = valid
        .iter()
        .enumerate()
        .map(|(i, (r, c))| {
            let unit = c.unit.unwrap();
            let is_best = i == 0;
            TpRank {
                id: r.id,
                idx: r.idx,
                rank: i + 1,
                name: if r.name.is_empty() {
                    format!("Roll {}", r.idx + 1)
                } else {
                    r.name.clone()
                },
                unit_str: tp_fmt(unit),
                label: c.unit_label.clone(),
                bar_pct: format!("{:.1}", (unit / worst_val) * 100.0),
                more: if is_best {
                    String::new()
                } else {
                    format!("{:.1}", (unit - best_val) / best_val * 100.0)
                },
                bar_color: if is_best {
                    "var(--win)".to_string()
                } else {
                    COLORS[r.idx % 4].to_string()
                },
                opacity: if is_best { "1" } else { "0.45" },
                is_best,
            }
        })
        .collect();

    rsx! {
        div { class: "results-summary",
            div { class: "summary-title", "{str_.ranking_prefix}{method_label}" }
            ul { class: "rank-list",
                for rank in ranks {
                    li { class: "rank-item", key: "{rank.id}",
                        div { class: "rank-row",
                            span { class: "rank-num", "{rank.rank}." }
                            span { class: "dot", style: "background:{COLORS[rank.idx % 4]}" }
                            span { class: "rank-name", "{rank.name}" }
                            span { class: "rank-val", "{rank.unit_str} {rank.label}" }
                            if rank.is_best {
                                span { class: "tag-best", "{str_.best_value}" }
                            } else {
                                span { class: "tag-more", "+{rank.more}%" }
                            }
                        }
                        div { class: "cost-bar-track",
                            div { class: "cost-bar-fill", style: "width:{rank.bar_pct}%;background:{rank.bar_color};opacity:{rank.opacity}" }
                        }
                    }
                }
            }
            div {
                style: "margin-top:12px;font-size:0.78rem;color:var(--muted);font-family:var(--mono);",
                dangerous_inner_html: "{choosing_html}",
            }
            div { class: "lifetime-block",
                div { class: "lifetime-header",
                    span { class: "lifetime-title", "{str_.lifetime_title}" }
                }
                div { class: "lifetime-inputs",
                    label { "{str_.rolls_per_week_lbl}" }
                    input {
                        r#type: "number",
                        min: "0.1",
                        max: "100",
                        step: "0.5",
                        value: "{usage_week_val}",
                        placeholder: "{str_.rolls_per_week_ph}",
                        oninput: move |evt| tp_usage_week.set(evt.value()),
                    }
                }
                if let Some(html) = lifetime_html {
                    div { class: "lifetime-result", dangerous_inner_html: "{html}" }
                }
            }
        }
    }
}

#[component]
fn ProPage() -> Element {
    let mut state = use_context::<AppState>();
    let lg = *state.lang.read();
    let str_ = s(lg);
    let cur = state.currency.read().clone();
    let powders_owned: Vec<Powder> = state.pro_powders.read().clone();

    let calced: Vec<(Powder, ProCalc)> = powders_owned
        .iter()
        .cloned()
        .map(|p| {
            let c = pro_calc_powder(&p);
            (p, c)
        })
        .collect();

    let mut valid_sorted: Vec<(Powder, ProCalc)> = calced.iter().filter(|(_, c)| c.valid).cloned().collect();
    valid_sorted.sort_by(|a, b| a.1.cpg.partial_cmp(&b.1.cpg).unwrap());
    let best_id = valid_sorted.first().map(|(p, _)| p.id);
    let valid_len = valid_sorted.len();

    let can_add = powders_owned.len() < 4;
    let has_save = *state.pro_has_save.read();
    let toast = state.pro_toast.read().clone();
    let powder_count = powders_owned.len();

    rsx! {
        div { class: "page visible",
            div { class: "top-controls",
                button {
                    class: if can_add { "add-btn" } else { "add-btn disabled" },
                    onclick: move |_| {
                        if state.pro_powders.read().len() < 4 {
                            let id = *state.pro_next_id.read();
                            let idx = state.pro_powders.read().len();
                            state.pro_next_id.set(id + 1);
                            state.pro_powders.write().push(Powder::new_default(id, idx));
                        }
                    },
                    "{str_.add_powder}"
                }
                label {
                    if powder_count > 0 {
                        "{count_powders(lg, powder_count)}"
                    }
                }
            }

            div { class: "session-bar",
                button {
                    class: "session-btn",
                    title: "{str_.save_session_title_pro}",
                    onclick: move |_| {
                        let session = ProSession {
                            powders: state.pro_powders.read().clone(),
                            next_id: *state.pro_next_id.read(),
                            usage_week: state.pro_usage_week.read().clone(),
                        };
                        storage::save_pro_session(&session);
                        state.pro_has_save.set(true);
                        state.pro_toast.set(s(*state.lang.read()).toast_saved.to_string());
                    },
                    "{str_.save_session}"
                }
                button {
                    class: if has_save { "session-btn has-save" } else { "session-btn" },
                    title: "{str_.restore_title}",
                    onclick: move |_| {
                        if let Some(session) = storage::load_pro_session() {
                            state.pro_powders.set(session.powders);
                            state.pro_next_id.set(session.next_id);
                            state.pro_usage_week.set(session.usage_week);
                            state.pro_toast.set(s(*state.lang.read()).toast_restored.to_string());
                        } else {
                            state.pro_toast.set(s(*state.lang.read()).toast_restore_fail.to_string());
                        }
                    },
                    "{str_.restore_session}"
                }
                button {
                    class: "session-btn",
                    title: "{str_.clear_title}",
                    onclick: move |_| {
                        storage::clear_pro_session();
                        state.pro_has_save.set(false);
                        state.pro_toast.set(s(*state.lang.read()).toast_cleared.to_string());
                    },
                    "{str_.clear_saved}"
                }
                span { class: if toast.is_empty() { "session-toast" } else { "session-toast show" }, "{toast}" }
            }

            div { class: "cards-grid",
                for p in powders_owned.iter().cloned() {
                    { let id = p.id; pro_card(p, cur.clone(), lg, Some(id) == best_id && valid_len > 1, powder_count > 1, state.pro_powders) }
                }
            }

            {pro_results(calced, cur.clone(), lg, state.pro_usage_week)}
        }
    }
}

fn pro_card(
    p: Powder,
    cur: String,
    lang: Lang,
    is_winner: bool,
    removable: bool,
    mut pro_powders: Signal<Vec<Powder>>,
) -> Element {
    let id = p.id;
    let str_ = s(lang);
    let col = COLORS[p.idx % 4];
    let calc = pro_calc_powder(&p);
    let per_serving_price = if calc.valid { pf(&p.price) / pf(&p.servings) } else { 0.0 };
    let price_label = card_price_label(lang, &cur);
    let cpg_str = format!("{:.4}", calc.cpg);
    let total_protein_str = format!("{:.0}", calc.total_protein);
    let per_serving_str = format!("{:.2}", per_serving_price);

    rsx! {
        div { class: if is_winner { "item-card winner" } else { "item-card" }, key: "{id}",
            div { class: "card-header",
                div { class: "card-title",
                    span { class: "dot", style: "background:{col}" }
                    input {
                        class: "name-input",
                        r#type: "text",
                        value: "{p.name}",
                        placeholder: "{str_.card_name}",
                        oninput: move |evt| {
                            let v = evt.value();
                            if let Some(x) = pro_powders.write().iter_mut().find(|x| x.id == id) {
                                x.name = v;
                            }
                        },
                    }
                }
                if removable {
                    button {
                        class: "remove-btn",
                        onclick: move |_| {
                            let mut powders = pro_powders.write();
                            powders.retain(|x| x.id != id);
                            for (i, x) in powders.iter_mut().enumerate() {
                                x.idx = i;
                            }
                        },
                        "×"
                    }
                }
            }
            div { class: "field",
                label { "{price_label}" }
                input {
                    r#type: "number",
                    value: "{p.price}",
                    min: "0",
                    step: "0.01",
                    placeholder: "e.g. 29.99",
                    oninput: move |evt| {
                        let v = evt.value();
                        if let Some(x) = pro_powders.write().iter_mut().find(|x| x.id == id) {
                            x.price = v;
                        }
                    },
                }
            }
            div { class: "field",
                label { "{str_.bag_weight}" }
                input {
                    r#type: "number",
                    value: "{p.weight}",
                    min: "0",
                    step: "1",
                    placeholder: "e.g. 1000",
                    oninput: move |evt| {
                        let v = evt.value();
                        if let Some(x) = pro_powders.write().iter_mut().find(|x| x.id == id) {
                            x.weight = v;
                        }
                    },
                }
            }
            div { class: "field",
                label { "{str_.servings_per_bag}" }
                input {
                    r#type: "number",
                    value: "{p.servings}",
                    min: "1",
                    step: "1",
                    placeholder: "e.g. 33",
                    oninput: move |evt| {
                        let v = evt.value();
                        if let Some(x) = pro_powders.write().iter_mut().find(|x| x.id == id) {
                            x.servings = v;
                        }
                    },
                }
            }
            div { class: "field",
                label { "{str_.protein_per_serving}" }
                input {
                    r#type: "number",
                    value: "{p.protein}",
                    min: "0",
                    step: "0.1",
                    placeholder: "e.g. 25",
                    oninput: move |evt| {
                        let v = evt.value();
                        if let Some(x) = pro_powders.write().iter_mut().find(|x| x.id == id) {
                            x.protein = v;
                        }
                    },
                }
            }
            div { class: "card-result",
                if calc.valid {
                    div {
                        class: "cpg",
                        style: if is_winner { "color:var(--win)" } else { "color:var(--text)" },
                        "{cur}{cpg_str}"
                    }
                    div { class: "cpg-label", "{str_.per_gram_protein}" }
                    div { class: "extra", "{total_protein_str}g total · {cur}{per_serving_str}{str_.per_serving}" }
                    if is_winner {
                        div { class: "winner-badge", "{str_.best_value}" }
                    }
                } else {
                    div { class: "cpg-label", style: "margin-top:4px", "{str_.fill_fields}" }
                }
            }
        }
    }
}

fn pro_results(all: Vec<(Powder, ProCalc)>, cur: String, lang: Lang, mut pro_usage_week: Signal<String>) -> Element {
    let str_ = s(lang);
    let mut sorted = all.clone();
    sorted.sort_by(|a, b| match (a.1.valid, b.1.valid) {
        (true, true) => a.1.cpg.partial_cmp(&b.1.cpg).unwrap(),
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        (false, false) => std::cmp::Ordering::Equal,
    });
    let valid: Vec<&(Powder, ProCalc)> = sorted.iter().filter(|(_, c)| c.valid).collect();

    if valid.len() < 2 {
        let msg = if all.is_empty() { str_.empty_pro } else { str_.empty_pro_valid };
        return rsx! {
            div { class: "results-summary",
                div { class: "empty", "{msg}" }
            }
        };
    }

    let best_cpg = valid[0].1.cpg;
    let worst_cpg = valid[valid.len() - 1].1.cpg;
    let savings = format!("{:.1}", (worst_cpg - best_cpg) / worst_cpg * 100.0);
    let best_name = if valid[0].0.name.is_empty() {
        str_.best_value.to_string()
    } else {
        valid[0].0.name.clone()
    };
    let choosing_html = choosing_saves(lang, &best_name, &savings, true);

    let usage_week_val = pro_usage_week.read().clone();
    let usage_week_num = pf(&usage_week_val);
    let lifetime_html = if usage_week_num > 0.0 {
        let best = &valid[0].0;
        let worst = &valid[valid.len() - 1].0;
        let weeks_per_bag_best = pf(&best.servings) / usage_week_num;
        let weeks_per_bag_worst = pf(&worst.servings) / usage_week_num;
        let cost_year_best = (52.0 / weeks_per_bag_best) * pf(&best.price);
        let cost_year_worst = (52.0 / weeks_per_bag_worst) * pf(&worst.price);
        let save_year = format!("{:.2}", cost_year_worst - cost_year_best);
        Some(lifetime_line(
            lang,
            usage_week_num,
            "serving",
            "servings",
            "portion",
            "portions",
            &format!("{:.0}", cost_year_best),
            &format!("{:.0}", cost_year_worst),
            &save_year,
            &cur,
        ))
    } else {
        None
    };

    let pro_ranks: Vec<ProRank> = sorted
        .iter()
        .enumerate()
        .map(|(i, (p, c))| {
            let is_best = i == 0;
            ProRank {
                id: p.id,
                idx: p.idx,
                rank: i + 1,
                name: if p.name.is_empty() {
                    format!("Powder {}", p.idx + 1)
                } else {
                    p.name.clone()
                },
                valid: c.valid,
                cpg_str: format!("{:.4}", c.cpg),
                bar_pct: format!("{:.1}", (c.cpg / worst_cpg) * 100.0),
                more: if is_best {
                    String::new()
                } else {
                    format!("{:.1}", (c.cpg - best_cpg) / best_cpg * 100.0)
                },
                bar_color: if is_best {
                    "var(--win)".to_string()
                } else {
                    COLORS[p.idx % 4].to_string()
                },
                opacity: if is_best { "1" } else { "0.45" },
                is_best,
            }
        })
        .collect();

    rsx! {
        div { class: "results-summary",
            div { class: "summary-title", "{str_.ranking_protein}" }
            ul { class: "rank-list",
                for rank in pro_ranks {
                    if rank.valid {
                        li { class: "rank-item", key: "{rank.id}",
                            div { class: "rank-row",
                                span { class: "rank-num", "{rank.rank}." }
                                span { class: "dot", style: "background:{COLORS[rank.idx % 4]}" }
                                span { class: "rank-name", "{rank.name}" }
                                span { class: "rank-val", "{cur}{rank.cpg_str}/g" }
                                if rank.is_best {
                                    span { class: "tag-best", "{str_.best_value}" }
                                } else {
                                    span { class: "tag-more", "+{rank.more}%" }
                                }
                            }
                            div { class: "cost-bar-track",
                                div { class: "cost-bar-fill", style: "width:{rank.bar_pct}%;background:{rank.bar_color};opacity:{rank.opacity}" }
                            }
                        }
                    } else {
                        li { class: "rank-item", key: "{rank.id}",
                            div { class: "rank-row",
                                span { class: "rank-num", "{rank.rank}." }
                                span { class: "dot", style: "background:{COLORS[rank.idx % 4]}" }
                                span { class: "rank-name", "{rank.name}" }
                                span { class: "rank-val", "—" }
                                span { class: "tag-more", style: "opacity:0.4", "{str_.incomplete_label}" }
                            }
                        }
                    }
                }
            }
            div {
                style: "margin-top:12px;font-size:0.78rem;color:var(--muted);font-family:var(--mono);",
                dangerous_inner_html: "{choosing_html}",
            }
            div { class: "lifetime-block",
                div { class: "lifetime-header",
                    span { class: "lifetime-title", "{str_.lifetime_title}" }
                }
                div { class: "lifetime-inputs",
                    label { "{str_.servings_per_week_lbl}" }
                    input {
                        r#type: "number",
                        min: "1",
                        max: "50",
                        step: "1",
                        value: "{usage_week_val}",
                        placeholder: "{str_.servings_per_week_ph}",
                        oninput: move |evt| pro_usage_week.set(evt.value()),
                    }
                }
                if let Some(html) = lifetime_html {
                    div { class: "lifetime-result", dangerous_inner_html: "{html}" }
                }
            }
        }
    }
}
