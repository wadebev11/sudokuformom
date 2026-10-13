use leptos::prelude::*;
use sudoku_core::Difficulty;

use crate::modal::Modal;

#[component]
pub fn NewGameMenu(
    progress_will_be_lost: Signal<bool>,
    can_restart: Signal<bool>,
    on_choose: Callback<Difficulty>,
    on_restart: Callback<()>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    view! {
        <Modal on_close=on_cancel>
            <div class="dialog" role="dialog" aria-label="New game">
                <h2>"New game"</h2>
                <Show when=move || progress_will_be_lost.get()>
                    <p class="warning">"Your current puzzle will be lost."</p>
                </Show>
                {Difficulty::ALL
                    .map(|difficulty| {
                        view! {
                            <button class="primary" on:click=move |_| on_choose.run(difficulty)>
                                {difficulty.name()}
                            </button>
                        }
                    })
                    .collect_view()}
                <Show when=move || can_restart.get()>
                    <button on:click=move |_| on_restart.run(())>"Restart this puzzle"</button>
                </Show>
                <button on:click=move |_| on_cancel.run(())>"Cancel"</button>
            </div>
        </Modal>
    }
}
