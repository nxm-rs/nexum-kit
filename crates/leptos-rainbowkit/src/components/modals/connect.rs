use leptos::prelude::*;
use leptos::callback::{UnsyncCallback, Callback};
use crate::components::primitives::{Dialog, Text, BoxFontWeight};
use crate::state::modal::{use_modal_state, ModalType};
use crate::state::connection::use_connection_state;
use crate::wallets::connectors::MetaMaskConnector;
use crate::wallets::eip6963::{setup_eip6963_discovery, EIP6963ProviderInfo};
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn ConnectModal() -> impl IntoView {
    let modal_state = use_modal_state();
    let connection_state = use_connection_state();

    let is_open = modal_state.is_open(ModalType::Connect);
    let on_close = UnsyncCallback::new(move |_| modal_state.close());

    // Store just the provider info (not the JsValue provider)
    let discovered_wallets = RwSignal::new(Vec::<EIP6963ProviderInfo>::new());

    // Setup EIP-6963 discovery when component mounts
    Effect::new(move |_| {
        log::info!("Setting up EIP-6963 wallet discovery");

        setup_eip6963_discovery(move |provider| {
            log::info!("EIP-6963: Discovered wallet: {}", provider.info.name);
            discovered_wallets.update(|wallets| {
                // Avoid duplicates based on uuid
                if !wallets.iter().any(|w| w.uuid == provider.info.uuid) {
                    wallets.push(provider.info);
                }
            });
        });
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
                "Connect a Wallet"
            </Text>
            <Text
                as_element="p"
                size="14px"
                color="modalTextSecondary"
                additional_style="margin-bottom: 24px;"
            >
                "Choose how you want to connect. There are several wallet providers to choose from."
            </Text>

            // Wallet list
            <div style="display: flex; flex-direction: column; gap: 12px;">
                // Show discovered EIP-6963 wallets
                <For
                    each=move || discovered_wallets.get()
                    key=|wallet| wallet.uuid.clone()
                    children={
                        let connection_state = connection_state.clone();
                        let modal_state = modal_state.clone();
                        move |wallet_info: EIP6963ProviderInfo| {
                            let wallet_name = wallet_info.name.clone();
                            let wallet_icon = wallet_info.icon.clone();
                            let _wallet_rdns = wallet_info.rdns.clone(); // Will be used to select connector

                            let handle_click = {
                                let wallet_name = wallet_name.clone();
                                let connection_state = connection_state.clone();
                                let modal_state = modal_state.clone();
                            Callback::new(move |_| {
                                let wallet_name = wallet_name.clone();
                                let connection_state = connection_state.clone();
                                let modal_state = modal_state.clone();
                                // Use the RDNS to identify which connector to use
                                // For now, we only support MetaMask
                                let connector = MetaMaskConnector::new();
                                spawn_local(async move {
                                    log::info!("Attempting to connect to {} via EIP-6963...", wallet_name);
                                    match connection_state.connect(&connector).await {
                                        Ok(_) => {
                                            log::info!("Successfully connected to {}!", wallet_name);
                                            modal_state.close();
                                        }
                                        Err(e) => {
                                            log::error!("Failed to connect: {:?}", e);
                                        }
                                    }
                                });
                            })
                        };

                        let connection_state_for_style = connection_state.clone();
                            let connection_state_for_disabled = connection_state.clone();

                            view! {
                            <button
                                class="wallet-option"
                                style=move || {
                                    let base_style = "
                                        display: flex;
                                        align-items: center;
                                        gap: 12px;
                                        width: 100%;
                                        padding: 16px;
                                        background: var(--rk-colors-modalBackground);
                                        border: 1px solid var(--rk-colors-actionButtonBorder);
                                        border-radius: var(--rk-radii-actionButton);
                                        transition: all 0.125s ease;
                                        font-family: var(--rk-fonts-body);
                                        font-size: 16px;
                                        font-weight: 600;
                                        color: var(--rk-colors-modalText);
                                    ";

                                    if connection_state_for_style.is_connecting() {
                                        format!("{} opacity: 0.6; cursor: wait;", base_style)
                                    } else {
                                        format!("{} cursor: pointer;", base_style)
                                    }
                                }
                                disabled=move || connection_state_for_disabled.is_connecting()
                                on:click=move |ev| handle_click.run(ev)
                            >
                                // Wallet icon from EIP-6963 (actual icon from the wallet!)
                                <img
                                    src=wallet_icon.clone()
                                    alt=format!("{} icon", wallet_name.clone())
                                    style="width: 40px; height: 40px; border-radius: 8px; object-fit: contain;"
                                />

                                <span style="flex: 1; text-align: left;">{wallet_name.clone()}</span>

                                // Show "Installed" badge for EIP-6963 wallets
                                <span style="
                                    padding: 4px 8px;
                                    background: var(--rk-colors-accentColor);
                                    color: var(--rk-colors-accentColorForeground);
                                    border-radius: 6px;
                                    font-size: 12px;
                                    font-weight: 600;
                                ">
                                    "Installed"
                                </span>
                            </button>
                        }
                        }
                    }
                />

                // Fallback: Show message if no wallets discovered
                <Show when=move || discovered_wallets.get().is_empty()>
                    <Text
                        as_element="p"
                        size="14px"
                        color="modalTextSecondary"
                        additional_style="text-align: center; margin-top: 8px; padding: 32px;"
                    >
                        "No wallets detected. Please install MetaMask or another Ethereum wallet."
                    </Text>
                </Show>
            </div>
        </Dialog>
    }
}
