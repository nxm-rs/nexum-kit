use leptos::prelude::*;
use leptos::callback::UnsyncCallback;
use crate::components::primitives::Dialog;
use crate::state::modal::{use_modal_state, ModalType};

#[component]
pub fn ConnectModal() -> impl IntoView {
    let modal_state = use_modal_state();
    let is_open = modal_state.is_open(ModalType::Connect);
    let on_close = UnsyncCallback::new(move |_| modal_state.close());

    view! {
        <Dialog open=is_open on_close=on_close>
            <h2 class="text-2xl font-bold mb-4 text-gray-900 dark:text-white">
                "Connect Wallet"
            </h2>
            <p class="text-gray-600 dark:text-gray-400">
                "Wallet selection will be implemented in Phase 3"
            </p>
        </Dialog>
    }
}
