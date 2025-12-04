use leptos::prelude::*;
use leptos::callback::{UnsyncCallback, Callback};
use crate::components::primitives::{Dialog, Text, BoxFontWeight};
use crate::state::modal::{use_modal_state, ModalType};
use crate::state::connection::use_connection_state;
use crate::hooks::use_wallet;
use crate::utils::format::format_address;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn AccountModal() -> impl IntoView {
    let modal_state = use_modal_state();
    let connection_state = use_connection_state();
    let wallet = use_wallet();

    let is_open = modal_state.is_open(ModalType::Account);
    let on_close = UnsyncCallback::new(move |_| modal_state.close());

    let handle_disconnect = {
        let connection_state = connection_state.clone();
        let modal_state = modal_state.clone();
        Callback::new(move |_| {
            let connection_state = connection_state.clone();
            let modal_state = modal_state.clone();
            spawn_local(async move {
                log::info!("Disconnecting wallet...");
                let _ = connection_state.disconnect().await;
                modal_state.close();
            });
        })
    };

    view! {
        <Dialog open=is_open on_close=on_close>
            <Show when=move || wallet.is_connected.get()>
                <Text
                    as_element="h2"
                    size="24px"
                    font_weight=BoxFontWeight::Bold
                    color="modalText"
                    additional_style="margin-bottom: 16px;"
                >
                    "Account"
                </Text>

                // Address display
                <div style="
                    padding: 16px;
                    background: var(--nk-colors-modalBackgroundSecondary);
                    border-radius: var(--nk-radii-modal);
                    margin-bottom: 16px;
                ">
                    <Text
                        as_element="p"
                        size="12px"
                        color="modalTextSecondary"
                        additional_style="margin-bottom: 4px;"
                    >
                        "Connected Address"
                    </Text>
                    <Text
                        as_element="p"
                        size="16px"
                        font_weight=BoxFontWeight::Semibold
                        color="modalText"
                        additional_style="font-family: monospace;"
                    >
                        {move || wallet.address.get().map(|a| format_address(&a)).unwrap_or_default()}
                    </Text>
                </div>

                // Disconnect button
                <button
                    style="
                        width: 100%;
                        padding: 12px 16px;
                        background: var(--nk-colors-modalBackground);
                        border: 1px solid var(--nk-colors-actionButtonBorder);
                        border-radius: var(--nk-radii-actionButton);
                        color: var(--nk-colors-error);
                        font-family: var(--nk-fonts-body);
                        font-size: 16px;
                        font-weight: 600;
                        cursor: pointer;
                        transition: all 0.125s ease;
                    "
                    on:click=move |ev| handle_disconnect.run(ev)
                >
                    "Disconnect"
                </button>
            </Show>
        </Dialog>
    }
}
