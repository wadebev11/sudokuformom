use leptos::prelude::*;
use sudoku_core::hint::Hint;

#[component]
pub fn HintPanel(hint: RwSignal<Option<Hint>>) -> impl IntoView {
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
            </div>
        </Show>
    }
}
