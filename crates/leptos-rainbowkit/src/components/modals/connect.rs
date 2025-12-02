use leptos::prelude::*;
use leptos::callback::UnsyncCallback;
use crate::components::primitives::{Dialog, Text, BoxFontWeight};
use crate::state::modal::{use_modal_state, ModalType};

#[component]
pub fn ConnectModal() -> impl IntoView {
    let modal_state = use_modal_state();
    let is_open = modal_state.is_open(ModalType::Connect);
    let on_close = UnsyncCallback::new(move |_| modal_state.close());

    Effect::new(move |_| {
        log::debug!("ConnectModal is_open signal changed: {}", is_open.get());
    });

    view! {
        <Dialog open=is_open on_close=on_close>
            <Text
                as_element="h2"
                size="24px"
                font_weight=BoxFontWeight::Bold
                color="modalText"
                additional_style="margin-bottom: 16px;"
            >
                "Connect Wallet"
            </Text>
            <Text
                as_element="p"
                size="16px"
                color="modalTextSecondary"
                additional_style="margin-bottom: 16px;"
            >
                "Wallet selection will be implemented in Phase 3"
            </Text>
            <button
                class="w-full py-3 px-4 font-medium transition-colors"
                style="background: var(--rk-colors-accentColor); color: var(--rk-colors-accentColorForeground); border-radius: var(--rk-radii-actionButton); border: none; cursor: pointer;"
                on:click=move |_| modal_state.close()
            >
                "Close"
            </button>
        </Dialog>
    }
}
