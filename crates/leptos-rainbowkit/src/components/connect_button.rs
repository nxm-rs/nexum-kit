use leptos::prelude::*;
use crate::state::modal::use_modal_state;

#[component]
pub fn ConnectButton() -> impl IntoView {
    let modal_state = use_modal_state();

    let handle_click = move |_| {
        modal_state.open_connect();
    };

    view! {
        <button
            class="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white font-medium rounded-lg transition-colors"
            on:click=handle_click
            data-rk=""
        >
            "Connect Wallet"
        </button>
    }
}
