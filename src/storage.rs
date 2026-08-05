use smart_shopping_calculator_core::models::{Lang, ProSession, TpSession};

const KEY_THEME: &str = "theme";
const KEY_LANG: &str = "lang";
const KEY_CURRENCY: &str = "currency";
const KEY_TP_SESSION: &str = "ssc_session_tp";
const KEY_PRO_SESSION: &str = "ssc_session_pro";

#[cfg(target_arch = "wasm32")]
mod kv {
    use gloo_storage::{LocalStorage, Storage};

    pub fn get(key: &str) -> Option<String> {
        LocalStorage::get::<String>(key).ok()
    }

    pub fn set(key: &str, value: &str) {
        let _ = LocalStorage::set(key, value);
    }

    pub fn delete(key: &str) {
        LocalStorage::delete(key);
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod kv {
    use std::collections::HashMap;
    use std::sync::Mutex;

    static MEM: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);

    pub fn get(key: &str) -> Option<String> {
        MEM.lock().ok()?.as_ref()?.get(key).cloned()
    }

    pub fn set(key: &str, value: &str) {
        if let Ok(mut m) = MEM.lock() {
            m.get_or_insert_with(HashMap::new)
                .insert(key.to_string(), value.to_string());
        }
    }

    pub fn delete(key: &str) {
        if let Ok(mut m) = MEM.lock() {
            if let Some(h) = m.as_mut() {
                h.remove(key);
            }
        }
    }
}

pub fn load_dark_pref() -> bool {
    match kv::get(KEY_THEME) {
        Some(v) => v == "dark",
        None => prefers_dark_media(),
    }
}

pub fn save_dark_pref(dark: bool) {
    kv::set(KEY_THEME, if dark { "dark" } else { "light" });
}

fn prefers_dark_media() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        (|| -> Option<bool> {
            let window = web_sys::window()?;
            let mql = window
                .match_media("(prefers-color-scheme: dark)")
                .ok()??;
            Some(mql.matches())
        })()
        .unwrap_or(false)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}

pub fn load_lang_pref() -> Lang {
    kv::get(KEY_LANG)
        .and_then(|v| Lang::from_code(&v))
        .unwrap_or(Lang::En)
}

pub fn save_lang_pref(lang: Lang) {
    kv::set(KEY_LANG, lang.code());
}

pub fn load_currency_pref() -> String {
    kv::get(KEY_CURRENCY).unwrap_or_else(|| "€".to_string())
}

pub fn save_currency_pref(cur: &str) {
    kv::set(KEY_CURRENCY, cur);
}

pub fn save_tp_session(session: &TpSession) {
    if let Ok(v) = serde_json::to_string(session) {
        kv::set(KEY_TP_SESSION, &v);
    }
}

pub fn load_tp_session() -> Option<TpSession> {
    kv::get(KEY_TP_SESSION).and_then(|v| serde_json::from_str(&v).ok())
}

pub fn clear_tp_session() {
    kv::delete(KEY_TP_SESSION);
}

pub fn has_tp_session() -> bool {
    kv::get(KEY_TP_SESSION).is_some()
}

pub fn save_pro_session(session: &ProSession) {
    if let Ok(v) = serde_json::to_string(session) {
        kv::set(KEY_PRO_SESSION, &v);
    }
}

pub fn load_pro_session() -> Option<ProSession> {
    kv::get(KEY_PRO_SESSION).and_then(|v| serde_json::from_str(&v).ok())
}

pub fn clear_pro_session() {
    kv::delete(KEY_PRO_SESSION);
}

pub fn has_pro_session() -> bool {
    kv::get(KEY_PRO_SESSION).is_some()
}
