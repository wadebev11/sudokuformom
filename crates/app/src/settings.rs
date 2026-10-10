use leptos::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub theme: Theme,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            show_mistakes: true,
            theme: Theme::default(),
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
                <button class="primary" on:click=move |_| on_close.run(())>"Done"</button>
            </div>
        </div>
    }
}
