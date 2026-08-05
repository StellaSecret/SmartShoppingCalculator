use crate::models::{Lang, ProSession, TpSession};
use gloo_storage::{LocalStorage, Storage};

const KEY_THEME: &str = "theme";
const KEY_LANG: &str = "lang";
const KEY_CURRENCY: &str = "currency";
const KEY_TP_SESSION: &str = "ssc_session_tp";
const KEY_PRO_SESSION: &str = "ssc_session_pro";

pub fn load_dark_pref() -> bool {
    match LocalStorage::get::<String>(KEY_THEME) {
        Ok(v) => v == "dark",
        Err(_) => prefers_dark_media(),
    }
}

pub fn save_dark_pref(dark: bool) {
    let _ = LocalStorage::set(KEY_THEME, if dark { "dark" } else { "light" });
}

fn prefers_dark_media() -> bool {
    (|| -> Option<bool> {
        let window = web_sys::window()?;
        let mql = window
            .match_media("(prefers-color-scheme: dark)")
            .ok()??;
        Some(mql.matches())
    })()
    .unwrap_or(false)
}

pub fn load_lang_pref() -> Lang {
    LocalStorage::get::<String>(KEY_LANG)
        .ok()
        .and_then(|v| Lang::from_code(&v))
        .unwrap_or(Lang::En)
}

pub fn save_lang_pref(lang: Lang) {
    let _ = LocalStorage::set(KEY_LANG, lang.code());
}

pub fn load_currency_pref() -> String {
    LocalStorage::get::<String>(KEY_CURRENCY).unwrap_or_else(|_| "€".to_string())
}

pub fn save_currency_pref(cur: &str) {
    let _ = LocalStorage::set(KEY_CURRENCY, cur);
}

pub fn save_tp_session(session: &TpSession) {
    let _ = LocalStorage::set(KEY_TP_SESSION, session);
}

pub fn load_tp_session() -> Option<TpSession> {
    LocalStorage::get::<TpSession>(KEY_TP_SESSION).ok()
}

pub fn clear_tp_session() {
    LocalStorage::delete(KEY_TP_SESSION);
}

pub fn has_tp_session() -> bool {
    LocalStorage::get::<TpSession>(KEY_TP_SESSION).is_ok()
}

pub fn save_pro_session(session: &ProSession) {
    let _ = LocalStorage::set(KEY_PRO_SESSION, session);
}

pub fn load_pro_session() -> Option<ProSession> {
    LocalStorage::get::<ProSession>(KEY_PRO_SESSION).ok()
}

pub fn clear_pro_session() {
    LocalStorage::delete(KEY_PRO_SESSION);
}

pub fn has_pro_session() -> bool {
    LocalStorage::get::<ProSession>(KEY_PRO_SESSION).is_ok()
}
