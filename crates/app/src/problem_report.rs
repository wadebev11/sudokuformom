//! Sends a Problem Report through the phone's share sheet, or copies it on a
//! computer. Pasting it into the app's browser console loads her game.

use js_sys::{JsString, Reflect};
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::Serialize;
use sudoku_core::Game;
use sudoku_core::grid::cell_name;
use sudoku_core::hint::Hint;
use wasm_bindgen_futures::JsFuture;
use web_sys::ShareData;

use crate::settings::Settings;
use crate::storage::{self, GAME_KEY, SETTINGS_KEY};

/// A script for the browser console that saves the game and settings, then
/// reloads so the app opens just as she had it.
fn problem_report(game: &Game, settings: &Settings, hint: Option<&Hint>) -> String {
    let hint_comment = match hint {
        Some(hint) => {
            let cells: Vec<String> = hint.cells().into_iter().map(cell_name).collect();
            format!(
                "// Hint showing: {}: {}\n// Cells: {}",
                hint.title(),
                hint.explanation(),
                cells.join("; ")
            )
        }
        None => "// No Hint showing".to_string(),
    };
    format!(
        "{hint_comment}\n{}\n{}\nlocation.reload();",
        save_statement(GAME_KEY, game),
        save_statement(SETTINGS_KEY, settings)
    )
}

/// The JSON is quoted a second time so it becomes a JavaScript string.
fn save_statement<T: Serialize>(key: &str, value: &T) -> String {
    let json = serde_json::to_string(value).unwrap_or_default();
    let json_string = serde_json::to_string(&json).unwrap_or_default();
    format!(
        "localStorage.setItem(\"{}\", {json_string});",
        storage::prefixed(key)
    )
}

fn navigator_has(property: &str) -> bool {
    Reflect::has(&window().navigator(), &JsString::from(property)).unwrap_or(false)
}

/// Phones and tablets get the share sheet. On a computer it offers odd choices
/// and no way to copy, so the report is copied for pasting instead.
fn is_touch_device() -> bool {
    window()
        .match_media("(pointer: coarse)")
        .ok()
        .flatten()
        .is_some_and(|query| query.matches())
}

#[component]
pub fn SendProblemReportButton(
    game: RwSignal<Game>,
    settings: RwSignal<Settings>,
    hint: RwSignal<Option<Hint>>,
) -> impl IntoView {
    let shares = is_touch_device() && navigator_has("share");
    let copies = !shares && navigator_has("clipboard");
    let copied = RwSignal::new(false);

    let send = move |_| {
        let report = game.with_untracked(|game| {
            settings.with_untracked(|settings| {
                hint.with_untracked(|hint| problem_report(game, settings, hint.as_ref()))
            })
        });
        if shares {
            let share_data = ShareData::new();
            share_data.set_title("Sudoku problem report");
            share_data.set_text(&report);
            // Cancelling the share sheet rejects the promise, which needs no response.
            let _ = window().navigator().share_with_data(&share_data);
        } else {
            let copying = window().navigator().clipboard().write_text(&report);
            spawn_local(async move {
                if JsFuture::from(copying).await.is_ok() {
                    copied.set(true);
                }
            });
        }
    };
    let label = move || {
        if copied.get() {
            "Copied! Paste it in a message to Wade"
        } else {
            "Send this game to Wade"
        }
    };

    (shares || copies).then(|| view! { <button on:click=send>{label}</button> })
}
