use leptos::html::Section;
use leptos::prelude::*;
use sudoku_core::{Difficulty, Technique};

use crate::modal::Modal;

#[component]
pub fn TechniqueGuide(current: Technique, on_close: Callback<()>) -> impl IntoView {
    view! {
        <Modal on_close>
            <div class="dialog technique-guide" role="dialog" aria-label="Techniques">
                <h2>"Techniques"</h2>
                <div class="technique-list">
                    {Difficulty::ALL
                        .map(|difficulty| {
                            view! {
                                <h3>{difficulty.name()}</h3>
                                {Technique::ALL
                                    .into_iter()
                                    .filter(|technique| technique.difficulty() == difficulty)
                                    .map(|technique| {
                                        let is_current = technique == current;
                                        let section = NodeRef::<Section>::new();
                                        if is_current {
                                            Effect::new(move || {
                                                if let Some(section) = section.get() {
                                                    section.scroll_into_view();
                                                }
                                            });
                                        }
                                        view! {
                                            <section
                                                class="technique"
                                                class:current=is_current
                                                node_ref=section
                                            >
                                                <h4>{technique.name()}</h4>
                                                <p>{technique.description()}</p>
                                            </section>
                                        }
                                    })
                                    .collect_view()}
                            }
                        })
                        .collect_view()}
                </div>
                <button class="primary" on:click=move |_| on_close.run(())>"Done"</button>
            </div>
        </Modal>
    }
}
