use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use sudoku_core::Game;
use sudoku_core::hint::Hint;

use crate::modal::Modal;
use crate::problem_report::SendProblemReportButton;
use crate::storage;

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}

impl Theme {
    pub const ALL: [Theme; 3] = [Theme::System, Theme::Light, Theme::Dark];

    /// Matches the serialized form so index.html can apply the saved theme before the app loads.
    pub fn attribute_value(self) -> &'static str {
        match self {
            Theme::System => "system",
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Theme::System => "Match device",
            Theme::Light => "Light",
            Theme::Dark => "Dark",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub show_mistakes: bool,
    pub auto_solve_naked_singles: bool,
    pub theme: Theme,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            show_mistakes: true,
            auto_solve_naked_singles: false,
            theme: Theme::default(),
        }
    }
}

#[component]
pub fn SettingsMenu(
    settings: RwSignal<Settings>,
    game: RwSignal<Game>,
    hint: RwSignal<Option<Hint>>,
    on_close: Callback<()>,
) -> impl IntoView {
    let confirming_reset = RwSignal::new(false);

    let erase_everything = move |_| {
        storage::clear_all();
        let _ = window().location().reload();
    };

    let close_from_backdrop = Callback::new(move |_| {
        if confirming_reset.get_untracked() {
            confirming_reset.set(false);
        } else {
            on_close.run(());
        }
    });

    view! {
        <Modal on_close=close_from_backdrop>
            <Show
                when=move || confirming_reset.get()
                fallback=move || view! { <SettingsOptions settings game hint confirming_reset on_close /> }
            >
                <div class="dialog" role="dialog" aria-label="Hard reset">
                    <h2>"Hard reset"</h2>
                    <p class="warning">
                        "This erases your current puzzle, stats, and settings. It can't be undone."
                    </p>
                    <button class="danger" on:click=erase_everything>"Erase everything"</button>
                    <button on:click=move |_| confirming_reset.set(false)>"Cancel"</button>
                </div>
            </Show>
        </Modal>
    }
}

#[component]
fn SettingsOptions(
    settings: RwSignal<Settings>,
    game: RwSignal<Game>,
    hint: RwSignal<Option<Hint>>,
    confirming_reset: RwSignal<bool>,
    on_close: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="dialog" role="dialog" aria-label="Settings">
            <h2>"Settings"</h2>
            <label class="setting">
                <input
                    type="checkbox"
                    prop:checked=move || settings.with(|settings| settings.show_mistakes)
                    on:change=move |event| {
                        settings.update(|settings| settings.show_mistakes = event_target_checked(&event))
                    }
                />
                "Show mistakes in red"
            </label>
            <label class="setting">
                <input
                    type="checkbox"
                    prop:checked=move || settings.with(|settings| settings.auto_solve_naked_singles)
                    on:change=move |event| {
                        settings.update(|settings| {
                            settings.auto_solve_naked_singles = event_target_checked(&event)
                        })
                    }
                />
                "Auto-solve naked singles (Not applied on Easy difficulty)"
            </label>
            <fieldset class="setting-group">
                <legend>"Appearance"</legend>
                {Theme::ALL
                    .map(|theme| {
                        view! {
                            <label class="setting">
                                <input
                                    type="radio"
                                    name="theme"
                                    prop:checked=move || settings.with(|settings| settings.theme == theme)
                                    on:change=move |_| settings.update(|settings| settings.theme = theme)
                                />
                                {theme.label()}
                            </label>
                        }
                    })
                    .collect_view()}
            </fieldset>
            <SendProblemReportButton game settings hint />
            <button class="danger" on:click=move |_| confirming_reset.set(true)>"Hard reset"</button>
            <button class="primary" on:click=move |_| on_close.run(())>"Done"</button>
        </div>
    }
}
