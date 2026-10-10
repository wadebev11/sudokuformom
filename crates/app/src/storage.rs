//! Saves state in the browser's local storage. Storage can be unavailable
//! (private browsing, blocked site data), so failures are ignored and the
//! app falls back to defaults.

use serde::Serialize;
use serde::de::DeserializeOwned;

pub const GAME_KEY: &str = "game";
pub const ELAPSED_KEY: &str = "elapsed";
pub const SETTINGS_KEY: &str = "settings";
pub const PLAYED_KEY: &str = "played";
pub const STATS_KEY: &str = "stats";

fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

pub fn load<T: DeserializeOwned>(key: &str) -> Option<T> {
    let json = local_storage()?.get_item(key).ok()??;
    serde_json::from_str(&json).ok()
}

pub fn save<T: Serialize>(key: &str, value: &T) {
    if let Some(storage) = local_storage()
        && let Ok(json) = serde_json::to_string(value)
    {
        let _ = storage.set_item(key, &json);
    }
}

pub fn clear_all() {
    if let Some(storage) = local_storage() {
        let _ = storage.clear();
    }
}
