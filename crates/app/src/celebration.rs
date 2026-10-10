use leptos::prelude::*;
use sudoku_core::Game;
use sudoku_core::stats::Stats;

use crate::format::clock;
use crate::modal::Modal;

const CONFETTI_COLORS: [&str; 6] = [
    "#e8505b", "#f9a826", "#14b1ab", "#2457c5", "#9b5de5", "#f15bb5",
];

#[component]
pub fn Celebration(
    game: RwSignal<Game>,
    elapsed_seconds: RwSignal<u64>,
    stats: RwSignal<Stats>,
    on_new_game: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    let best = move || {
        let difficulty = game.with(Game::difficulty);
        stats.with(|stats| stats.for_difficulty(difficulty).best_seconds)
    };

    view! {
        <Modal class="celebration" on_close>
            <div class="confetti" aria-hidden="true">
                {(0..24)
                    .map(|index| {
                        let style = format!(
                            "left: {}%; animation-delay: {}ms; background: {}",
                            (index * 37) % 100,
                            (index * 173) % 1400,
                            CONFETTI_COLORS[index % CONFETTI_COLORS.len()],
                        );
                        view! { <span style=style></span> }
                    })
                    .collect_view()}
            </div>
            <div class="dialog" role="dialog" aria-label="Puzzle solved">
                <h2 role="status">"Solved!"</h2>
                <p class="celebration-time">
                    {move || format!("{} puzzle in {}", game.with(Game::difficulty).name(), clock(elapsed_seconds.get()))}
                </p>
                <p class="celebration-best">{move || best().map(|seconds| format!("Best time: {}", clock(seconds)))}</p>
                <button class="primary" on:click=move |_| on_new_game.run(())>"New game"</button>
                <button on:click=move |_| on_close.run(())>"Close"</button>
            </div>
        </Modal>
    }
}
