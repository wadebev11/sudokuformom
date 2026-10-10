use leptos::prelude::*;
use sudoku_core::{Cell, Game};

#[component]
pub fn NumberPad(game: RwSignal<Game>, selected: RwSignal<Option<Cell>>) -> impl IntoView {
    let enter = move |digit| {
        if let Some(cell) = selected.get() {
            game.update(|game| game.enter(cell, digit));
        }
    };
    let erase = move |_| {
        if let Some(cell) = selected.get() {
            game.update(|game| game.erase(cell));
        }
    };

    view! {
        <div class="number-pad">
            {(1..=9).map(|digit| view! { <button on:click=move |_| enter(digit)>{digit}</button> }).collect_view()}
            <button on:click=erase>"Erase"</button>
        </div>
    }
}
