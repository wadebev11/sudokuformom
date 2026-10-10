use leptos::prelude::*;
use sudoku_core::Difficulty;

#[component]
pub fn NewGameMenu(on_choose: Callback<Difficulty>, on_cancel: Callback<()>) -> impl IntoView {
    view! {
        <div class="overlay">
            <div class="dialog" role="dialog" aria-label="New game">
                <h2>"New game"</h2>
                {Difficulty::ALL
                    .map(|difficulty| {
                        view! {
                            <button class="primary" on:click=move |_| on_choose.run(difficulty)>
                                {difficulty.name()}
                            </button>
                        }
                    })
                    .collect_view()}
                <button on:click=move |_| on_cancel.run(())>"Cancel"</button>
            </div>
        </div>
    }
}
