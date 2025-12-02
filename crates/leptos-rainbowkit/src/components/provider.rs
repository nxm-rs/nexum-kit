use leptos::prelude::*;
use crate::state::modal::provide_modal_state;

#[component]
pub fn RainbowKitProvider(
    children: Children,
) -> impl IntoView {
    // Provide modal state
    provide_modal_state();

    view! {
        <div data-rk="">
            {children()}
        </div>
    }
}
