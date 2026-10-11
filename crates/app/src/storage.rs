//! Saves state in the browser's local storage. Storage can be unavailable
//! (private browsing, blocked site data), so failures are ignored and the
//! app falls back to defaults.
//!
//! Every app on a GitHub Pages account shares one origin and so one local
//! storage, so keys are prefixed to keep this app's state apart from theirs.

use serde::Serialize;
use serde::de::DeserializeOwned;

pub const GAME_KEY: &str = "game";
pub const ELAPSED_KEY: &str = "elapsed";
pub const SETTINGS_KEY: &str = "settings";
pub const PLAYED_KEY: &str = "played";
pub const STATS_KEY: &str = "stats";

/// index.html reads the saved theme before the app loads, so it repeats this prefix.
const KEY_PREFIX: &str = "sudoku-for-mom:";

fn prefixed(key: &str) -> String {
    format!("{KEY_PREFIX}{key}")
}

fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

pub fn load<T: DeserializeOwned>(key: &str) -> Option<T> {
    let json = local_storage()?.get_item(&prefixed(key)).ok()??;
    serde_json::from_str(&json).ok()
}

pub fn save<T: Serialize>(key: &str, value: &T) {
    if let Some(storage) = local_storage()
        && let Ok(json) = serde_json::to_string(value)
    {
        let _ = storage.set_item(&prefixed(key), &json);
    }
}

/// Removes every key with this app's prefix, including ones older versions saved.
pub fn clear_all() {
    if let Some(storage) = local_storage()
        && let Ok(length) = storage.length()
    {
        // Collected first because removing a key shifts the indexes of the rest.
        let own_keys: Vec<String> = (0..length)
            .filter_map(|index| storage.key(index).ok().flatten())
            .filter(|key| key.starts_with(KEY_PREFIX))
            .collect();
        for key in own_keys {
            let _ = storage.remove_item(&key);
        }
    }
}
