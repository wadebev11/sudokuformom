use leptos::prelude::*;

#[component]
pub fn Modal(
    on_close: Callback<()>,
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    let close_when_backdrop_clicked = move |event: leptos::ev::MouseEvent| {
        if event.target() == event.current_target() {
            on_close.run(());
        }
    };

    view! {
        <div class=format!("overlay {class}") on:click=close_when_backdrop_clicked>
            {children()}
        </div>
    }
}
