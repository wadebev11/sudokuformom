use std::sync::LazyLock;

use leptos::prelude::*;
use sudoku_core::{Cell, Difficulty, Game, PuzzleBank};

use crate::board::Board;
use crate::number_pad::NumberPad;

static PUZZLE_BANK: LazyLock<PuzzleBank> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../puzzles.json"))
        .expect("bundled puzzle bank is valid JSON")
});

fn new_game(difficulty: Difficulty) -> Game {
    let puzzles = PUZZLE_BANK.puzzles(difficulty);
    let index = (js_sys::Math::random() * puzzles.len() as f64) as usize;
    Game::new(puzzles[index], difficulty)
}

#[component]
pub fn App() -> impl IntoView {
    let game = RwSignal::new(new_game(Difficulty::Easy));
    let selected = RwSignal::new(None::<Cell>);
    let start_new_game = move |_| {
        game.set(new_game(game.with(Game::difficulty)));
        selected.set(None);
    };

    view! {
        <main>
            <header>
                <h1>"Sudoku for Mom"</h1>
                <button on:click=start_new_game>"New game"</button>
            </header>
            <Board game selected />
            <Show when=move || game.with(Game::is_solved)>
                <p class="solved" role="status">"Solved!"</p>
            </Show>
            <NumberPad game selected />
        </main>
    }
}
