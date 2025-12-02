use leptos::prelude::*;
use crate::state::modal::use_modal_state;
use crate::hooks::use_wallet;
use crate::utils::format::format_address;

#[component]
pub fn ConnectButton() -> impl IntoView {
    let modal_state = use_modal_state();
    let wallet = use_wallet();

    let handle_click = move |_| {
        if wallet.is_connected.get() {
            log::debug!("Opening account modal");
            modal_state.open_account();
        } else {
            log::debug!("Opening connect modal");
            modal_state.open_connect();
        }
    };

    // Dynamic button text based on connection state
    let button_text = move || {
        if wallet.is_connecting.get() {
            "Connecting...".to_string()
        } else if let Some(addr) = wallet.address.get() {
            format_address(&addr)
        } else {
            "Connect Wallet".to_string()
        }
    };

    view! {
        <button
            class="rk-button rk-button-primary"
            on:click=handle_click
            data-rk=""
            disabled=move || wallet.is_connecting.get()
            style=move || {
                if wallet.is_connecting.get() {
                    "opacity: 0.6; cursor: wait;"
                } else {
                    ""
                }
            }
        >
            {button_text}
        </button>
    }
}
