use leptos::prelude::*;
use sudoku_core::{Cell, Game};

#[component]
pub fn NumberPad(game: RwSignal<Game>, selected: RwSignal<Option<Cell>>) -> impl IntoView {
    let pencil_mode = RwSignal::new(false);
    let enter = move |digit| {
        if let Some(cell) = selected.get() {
            if pencil_mode.get() {
                game.update(|game| game.toggle_pencil_mark(cell, digit));
            } else {
                game.update(|game| game.enter(cell, digit));
            }
        }
    };
    let erase = move |_| {
        if let Some(cell) = selected.get() {
            game.update(|game| game.erase(cell));
        }
    };

    view! {
        <div class="toolbar">
            <button on:click=move |_| game.update(Game::undo) disabled=move || !game.with(Game::can_undo)>
                "Undo"
            </button>
            <button on:click=move |_| game.update(Game::redo) disabled=move || !game.with(Game::can_redo)>
                "Redo"
            </button>
            <button
                class:active=move || pencil_mode.get()
                aria-pressed=move || pencil_mode.get().to_string()
                on:click=move |_| pencil_mode.update(|on| *on = !*on)
            >
                "Pencil"
            </button>
            <button on:click=erase>"Erase"</button>
        </div>
        <div class="number-pad" class:pencil=move || pencil_mode.get()>
            {(1..=9).map(|digit| view! { <button on:click=move |_| enter(digit)>{digit}</button> }).collect_view()}
        </div>
    }
}
