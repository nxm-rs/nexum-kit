use leptos::prelude::*;
use crate::state::modal::use_modal_state;

#[component]
pub fn ConnectButton() -> impl IntoView {
    let modal_state = use_modal_state();

    let handle_click = move |_| {
        log::debug!("Connect button clicked!");
        modal_state.open_connect();
        log::debug!("Modal state updated to Connect");
    };

    view! {
        <button
            class="rk-button rk-button-primary"
            on:click=handle_click
            data-rk=""
        >
            "Connect Wallet"
        </button>
    }
}
