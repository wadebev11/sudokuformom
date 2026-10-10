use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub show_mistakes: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            show_mistakes: true,
        }
    }
}

#[component]
pub fn SettingsMenu(settings: RwSignal<Settings>, on_close: Callback<()>) -> impl IntoView {
    view! {
        <div class="overlay">
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
                <button class="primary" on:click=move |_| on_close.run(())>"Done"</button>
            </div>
        </div>
    }
}
