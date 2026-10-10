use leptos::prelude::*;
use sudoku_core::Difficulty;
use sudoku_core::stats::Stats;

use crate::format::clock;

#[component]
pub fn StatsMenu(stats: RwSignal<Stats>, on_close: Callback<()>) -> impl IntoView {
    let time = |seconds: Option<u64>| seconds.map_or("–".to_string(), clock);

    view! {
        <div class="overlay">
            <div class="dialog" role="dialog" aria-label="Stats">
                <h2>"Stats"</h2>
                <table class="stats">
                    <thead>
                        <tr>
                            <th></th>
                            <th>"Solved"</th>
                            <th>"Best"</th>
                            <th>"Average"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {Difficulty::ALL
                            .map(|difficulty| {
                                let row = move || stats.with(|stats| stats.for_difficulty(difficulty));
                                view! {
                                    <tr>
                                        <th scope="row">{difficulty.name()}</th>
                                        <td>{move || row().solved}</td>
                                        <td>{move || time(row().best_seconds)}</td>
                                        <td>{move || time(row().average_seconds())}</td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
                <button class="primary" on:click=move |_| on_close.run(())>"Done"</button>
            </div>
        </div>
    }
}
