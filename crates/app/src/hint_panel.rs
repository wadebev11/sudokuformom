use leptos::prelude::*;
use sudoku_core::Technique;
use sudoku_core::hint::Hint;

#[component]
pub fn HintPanel(
    hint: RwSignal<Option<Hint>>,
    on_learn_more: Callback<Technique>,
) -> impl IntoView {
    let instruction = move || {
        hint.with(|hint| match hint {
            Some(Hint::Mistake(_)) => "Tap Hint again to clear it.",
            Some(hint) if hint.places_a_number() => "Tap Hint again to fill it in.",
            _ => "Tap Hint again to cross those numbers out of your notes.",
        })
    };

    view! {
        <Show when=move || hint.with(Option::is_some)>
            <div class="hint-panel" role="note" aria-label="Hint">
                <strong>{move || hint.with(|hint| hint.as_ref().map(Hint::title))}</strong>
                <p>{move || hint.with(|hint| hint.as_ref().map(Hint::explanation))}</p>
                <p class="hint-instruction">{instruction}</p>
                {move || {
                    hint.with(|hint| hint.as_ref().and_then(Hint::technique))
                        .map(|technique| {
                            view! {
                                <button class="link" on:click=move |_| on_learn_more.run(technique)>
                                    {format!("Learn more about {}", technique.name())}
                                </button>
                            }
                        })
                }}
            </div>
        </Show>
    }
}
