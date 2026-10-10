use leptos::ev;
use leptos::prelude::*;
use sudoku_core::hint::Hint;
use sudoku_core::{Cell, Digit, Game};

#[component]
pub fn NumberPad(
    game: RwSignal<Game>,
    selected: RwSignal<Option<Cell>>,
    hint: RwSignal<Option<Hint>>,
    solved: Memo<bool>,
    accepts_keys: Signal<bool>,
) -> impl IntoView {
    let pencil_mode = RwSignal::new(false);
    let change_game = move |change: &dyn Fn(&mut Game)| {
        game.update(|game| change(game));
        hint.set(None);
    };
    let enter = move |digit| {
        if let Some(cell) = selected.get() {
            if pencil_mode.get() {
                change_game(&|game| game.toggle_pencil_mark(cell, digit));
            } else {
                change_game(&|game| game.enter(cell, digit));
            }
        }
    };
    let erase = move || {
        if let Some(cell) = selected.get() {
            change_game(&|game| game.erase(cell));
        }
    };
    let key_listener = window_event_listener(ev::keydown, move |event| {
        let has_modifier = event.ctrl_key() || event.meta_key() || event.alt_key();
        if accepts_keys.get_untracked() && !has_modifier {
            let key = event.key();
            if let Some(digit) = key
                .parse::<Digit>()
                .ok()
                .filter(|digit| (1..=9).contains(digit))
            {
                event.prevent_default();
                enter(digit);
            } else if key == "Backspace" || key == "Delete" {
                event.prevent_default();
                erase();
            }
        }
    });
    on_cleanup(move || key_listener.remove());
    let show_or_apply_hint = move |_| match hint.get() {
        Some(shown) => change_game(&|game| game.apply_hint(&shown)),
        None => hint.set(game.with(Game::hint)),
    };

    view! {
        <div class="toolbar">
            <button on:click=move |_| change_game(&Game::undo) disabled=move || !game.with(Game::can_undo)>
                "Undo"
            </button>
            <button on:click=move |_| change_game(&Game::redo) disabled=move || !game.with(Game::can_redo)>
                "Redo"
            </button>
            <button
                class:active=move || pencil_mode.get()
                aria-pressed=move || pencil_mode.get().to_string()
                on:click=move |_| pencil_mode.update(|on| *on = !*on)
            >
                "Pencil"
            </button>
            <button on:click=move |_| erase()>"Erase"</button>
            <button
                class:active=move || hint.with(Option::is_some)
                on:click=show_or_apply_hint
                disabled=move || solved.get()
            >
                "Hint"
            </button>
        </div>
        <div class="number-pad" class:pencil=move || pencil_mode.get()>
            {(1..=9).map(|digit| view! { <button on:click=move |_| enter(digit)>{digit}</button> }).collect_view()}
        </div>
    }
}
