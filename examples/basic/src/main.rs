use leptos::prelude::*;
use leptos::callback::{Callback, UnsyncCallback};
use leptos::task::spawn_local;
use nexum_kit::prelude::*;
use nexum_kit::components::modals::{ConnectModal, AccountModal};
use nexum_kit::theme::{LightTheme, DarkTheme, MidnightTheme, ThemeOptions, BorderRadius, FontStack, OverlayBlur};
use std::collections::HashMap;
use alloy::signers::Signer;
use alloy::dyn_abi::eip712::TypedData;
use alloy::primitives::Address;
use alloy_eip1193::prelude::*;

fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <App />
        }
    })
}

#[derive(Clone, Copy, PartialEq)]
enum ThemeMode {
    Light,
    Dark,
    Midnight,
}

impl ThemeMode {
    fn next(&self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Midnight,
            Self::Midnight => Self::Light,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Dark => "Dark",
            Self::Midnight => "Midnight",
        }
    }
}

#[component]
fn App() -> impl IntoView {
    let (theme_mode, set_theme_mode) = signal(ThemeMode::Light);

    let bg_color = Callback::new(move |_: ()| match theme_mode.get() {
        ThemeMode::Light => "#f5f5f5",
        ThemeMode::Dark => "#1A1B1F",
        ThemeMode::Midnight => "#000",
    });

    let text_color = Callback::new(move |_: ()| match theme_mode.get() {
        ThemeMode::Light => "#25292E",
        ThemeMode::Dark => "#FFF",
        ThemeMode::Midnight => "#FFF",
    });

    let card_bg = Callback::new(move |_: ()| match theme_mode.get() {
        ThemeMode::Light => "#FFF",
        ThemeMode::Dark => "#1A1B1F",
        ThemeMode::Midnight => "#000",
    });

    let handle_theme_toggle = UnsyncCallback::new(move |_: leptos::ev::MouseEvent| {
        set_theme_mode.set(theme_mode.get().next());
    });

    // Configure RPC transports for each chain
    let mut transports = HashMap::new();
    // Ethereum Mainnet
    transports.insert(1, "https://eth.llamarpc.com".to_string());
    // Sepolia testnet
    transports.insert(11155111, "https://ethereum-sepolia-rpc.publicnode.com".to_string());
    // Polygon
    transports.insert(137, "https://polygon.llamarpc.com".to_string());
    // Arbitrum
    transports.insert(42161, "https://arbitrum.llamarpc.com".to_string());
    // Optimism
    transports.insert(10, "https://optimism.llamarpc.com".to_string());
    // Gnosis Chain
    transports.insert(100, "https://rpc.gnosischain.com".to_string());

    // Create theme options with blur enabled
    let theme_options = ThemeOptions {
        accent_color: None,
        accent_color_foreground: None,
        border_radius: BorderRadius::Large,
        font_stack: FontStack::Rounded,
        overlay_blur: OverlayBlur::Small, // Enable blur on modal backdrop
    };

    view! {
        {move || match theme_mode.get() {
            ThemeMode::Light => view! {
                <NexumKitProvider transports=transports.clone() theme=LightTheme theme_options=theme_options.clone()>
                    <AppContent
                        theme_mode=theme_mode
                        bg_color=bg_color
                        text_color=text_color
                        card_bg=card_bg
                        on_toggle=handle_theme_toggle
                    />
                </NexumKitProvider>
            }.into_any(),
            ThemeMode::Dark => view! {
                <NexumKitProvider transports=transports.clone() theme=DarkTheme theme_options=theme_options.clone()>
                    <AppContent
                        theme_mode=theme_mode
                        bg_color=bg_color
                        text_color=text_color
                        card_bg=card_bg
                        on_toggle=handle_theme_toggle
                    />
                </NexumKitProvider>
            }.into_any(),
            ThemeMode::Midnight => view! {
                <NexumKitProvider transports=transports.clone() theme=MidnightTheme theme_options=theme_options.clone()>
                    <AppContent
                        theme_mode=theme_mode
                        bg_color=bg_color
                        text_color=text_color
                        card_bg=card_bg
                        on_toggle=handle_theme_toggle
                    />
                </NexumKitProvider>
            }.into_any(),
        }}
    }
}

#[component]
fn AppContent(
    theme_mode: ReadSignal<ThemeMode>,
    bg_color: Callback<(), &'static str>,
    text_color: Callback<(), &'static str>,
    card_bg: Callback<(), &'static str>,
    on_toggle: UnsyncCallback<leptos::ev::MouseEvent>,
) -> impl IntoView {
    view! {
        <div
            class="min-h-screen flex flex-col items-center justify-center gap-6 p-4"
            style=move || format!("background: {}; transition: background 0.3s ease;", bg_color.run(()))
        >
            <div class="text-center space-y-2">
                <h1
                    class="text-5xl font-bold"
                    style=move || format!(
                        "color: {}; transition: color 0.3s ease;",
                        text_color.run(())
                    )
                >
                    "Nexum-Kit"
                </h1>
                <p
                    class="text-lg"
                    style=move || format!("color: {}; opacity: 0.8;", text_color.run(()))
                >
                    "Phase 2: Core Components Complete ✨"
                </p>
            </div>

            <div
                class="rounded-2xl p-8 shadow-lg max-w-md w-full space-y-4"
                style=move || format!(
                    "background: {}; transition: background 0.3s ease;",
                    card_bg.run(())
                )
            >
                <div class="flex justify-between items-center mb-4">
                    <h2
                        class="text-2xl font-bold"
                        style=move || format!("color: {};", text_color.run(()))
                    >
                        "Features Implemented"
                    </h2>
                    <button
                        class="px-3 py-1 rounded-lg text-sm font-medium"
                        style="background: var(--nk-colors-accentColor); color: var(--nk-colors-accentColorForeground);"
                        on:click=move |ev| on_toggle.run(ev)
                    >
                        {move || format!("Theme: {}", theme_mode.get().name())}
                    </button>
                </div>
                <ul
                    class="space-y-2"
                    style=move || format!("color: {}; opacity: 0.8;", text_color.run(()))
                >
                    <li>"✅ Theme system with CSS variables"</li>
                    <li>"✅ Modal state management"</li>
                    <li>"✅ Animated dialog component"</li>
                    <li>"✅ Connect button with theming"</li>
                    <li>"✅ Light, Dark & Midnight themes"</li>
                    <li>"✅ Box & Text primitive components"</li>
                </ul>

                <div class="pt-4">
                    <ConnectButton />
                </div>

                <ConnectModal />
                <AccountModal />

                <p
                    class="text-sm pt-2"
                    style=move || format!("color: {}; opacity: 0.6;", text_color.run(()))
                >
                    "Click the button to see the modal! 🎉"
                </p>
            </div>

            // Demo section - only shows when connected
            <DemoSection
                card_bg=card_bg
                text_color=text_color
            />
        </div>
    }
}

#[component]
fn DemoSection(
    card_bg: Callback<(), &'static str>,
    text_color: Callback<(), &'static str>,
) -> impl IntoView {
    let wallet = use_wallet();

    // Debug: Log connection status
    Effect::new({
        let wallet = wallet.clone();
        move |_| {
            log::info!("DemoSection - is_connected: {}, address: {:?}",
                wallet.is_connected.get(),
                wallet.address.get()
            );
        }
    });

    // State for results
    let (signature_result, set_signature_result) = signal(None::<String>);
    let (typed_signature_result, set_typed_signature_result) = signal(None::<String>);
    let (balance_result, set_balance_result) = signal(None::<String>);
    let (vitalik_balance_result, set_vitalik_balance_result) = signal(None::<String>);
    let (block_number_result, set_block_number_result) = signal(None::<String>);
    let (send_tx_result, set_send_tx_result) = signal(None::<String>);

    // Handler: Personal Sign
    let handle_personal_sign = move |_| {
        // Read values before entering async context
        let addr = wallet.address_untracked();

        if addr.is_none() {
            set_signature_result.set(Some("No address connected".to_string()));
            return;
        }

        set_signature_result.set(Some("Signing...".to_string()));
        spawn_local(async move {
            let addr = addr.unwrap();
            match Eip1193Transport::get_ethereum() {
                Ok(ethereum) => {
                    let signer = Eip1193Signer::new(ethereum, addr);
                    let message = b"Hello from Nexum-Kit! This is a test message.";

                    match signer.sign_message(message).await {
                        Ok(signature) => {
                            let sig_str = format!("0x{}", hex::encode(signature.as_bytes()));
                            log::info!("Personal Sign signature: {}", sig_str);
                            set_signature_result.set(Some(sig_str));
                        }
                        Err(e) => {
                            let err_msg = format!("Sign failed: {:?}", e);
                            log::error!("{}", err_msg);
                            set_signature_result.set(Some(err_msg));
                        }
                    }
                }
                Err(e) => {
                    let err_msg = format!("Failed to get ethereum: {:?}", e);
                    log::error!("{}", err_msg);
                    set_signature_result.set(Some(err_msg));
                }
            }
        });
    };

    // Handler: EIP-712 Typed Data Sign
    let handle_typed_sign = move |_| {
        // Read values before entering async context
        let addr = wallet.address_untracked();
        let chain_id = wallet.chain_id_untracked().unwrap_or(1);

        if addr.is_none() {
            set_typed_signature_result.set(Some("No address connected".to_string()));
            return;
        }

        set_typed_signature_result.set(Some("Signing...".to_string()));
        spawn_local(async move {
            let addr = addr.unwrap();
            match Eip1193Transport::get_ethereum() {
                Ok(ethereum) => {
                    let signer = Eip1193Signer::new(ethereum, addr);

                    // Create EIP-712 typed data
                    let typed_data_json = serde_json::json!({
                        "types": {
                            "EIP712Domain": [
                                {"name": "name", "type": "string"},
                                {"name": "version", "type": "string"},
                                {"name": "chainId", "type": "uint256"}
                            ],
                            "Person": [
                                {"name": "name", "type": "string"},
                                {"name": "wallet", "type": "address"}
                            ]
                        },
                        "domain": {
                            "name": "Nexum-Kit",
                            "version": "1",
                            "chainId": chain_id
                        },
                        "primaryType": "Person",
                        "message": {
                            "name": "Alice",
                            "wallet": format!("{:?}", addr)
                        }
                    });

                    log::info!("EIP-712 typed data: {}", serde_json::to_string_pretty(&typed_data_json).unwrap());

                    let typed_data: TypedData = serde_json::from_value(typed_data_json).unwrap();

                    match signer.sign_dynamic_typed_data(&typed_data).await {
                        Ok(signature) => {
                            let sig_str = format!("0x{}", hex::encode(signature.as_bytes()));
                            log::info!("EIP-712 signature: {}", sig_str);
                            set_typed_signature_result.set(Some(sig_str));
                        }
                        Err(e) => {
                            let err_msg = format!("Sign failed: {:?}", e);
                            log::error!("{}", err_msg);
                            set_typed_signature_result.set(Some(err_msg));
                        }
                    }
                }
                Err(e) => {
                    let err_msg = format!("Failed to get ethereum: {:?}", e);
                    log::error!("{}", err_msg);
                    set_typed_signature_result.set(Some(err_msg));
                }
            }
        });
    };

    // Handler: Fetch My Balance
    let handle_fetch_my_balance = move |_| {
        // Read values before entering async context
        let provider = wallet.provider_untracked();
        let addr = wallet.address_untracked();

        if provider.is_none() || addr.is_none() {
            set_balance_result.set(Some("Not connected".to_string()));
            return;
        }

        set_balance_result.set(Some("Fetching...".to_string()));
        spawn_local(async move {
            let provider = provider.unwrap();
            let addr = addr.unwrap();
            match provider.get_balance(addr).await {
                Ok(balance) => {
                    let eth_balance = balance.to::<u128>() as f64 / 1e18;
                    let result = format!("{:.6} ETH", eth_balance);
                    log::info!("My balance: {}", result);
                    set_balance_result.set(Some(result));
                }
                Err(e) => {
                    let err_msg = format!("Failed: {:?}", e);
                    log::error!("{}", err_msg);
                    set_balance_result.set(Some(err_msg));
                }
            }
        });
    };

    // Handler: Fetch Vitalik's Balance
    let handle_fetch_vitalik_balance = move |_| {
        // Read values before entering async context
        let provider = wallet.provider_untracked();

        if provider.is_none() {
            set_vitalik_balance_result.set(Some("Not connected".to_string()));
            return;
        }

        set_vitalik_balance_result.set(Some("Fetching...".to_string()));
        spawn_local(async move {
            let provider = provider.unwrap();
            // Vitalik's address
            let vitalik: Address = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"
                .parse()
                .unwrap();

            match provider.get_balance(vitalik).await {
                Ok(balance) => {
                    let eth_balance = balance.to::<u128>() as f64 / 1e18;
                    let result = format!("{:.6} ETH", eth_balance);
                    log::info!("vitalik.eth balance: {}", result);
                    set_vitalik_balance_result.set(Some(result));
                }
                Err(e) => {
                    let err_msg = format!("Failed: {:?}", e);
                    log::error!("{}", err_msg);
                    set_vitalik_balance_result.set(Some(err_msg));
                }
            }
        });
    };

    // Handler: Fetch Block Number
    let handle_fetch_block_number = move |_| {
        // Read values before entering async context
        let provider = wallet.provider_untracked();

        if provider.is_none() {
            set_block_number_result.set(Some("Not connected".to_string()));
            return;
        }

        set_block_number_result.set(Some("Fetching...".to_string()));
        spawn_local(async move {
            let provider = provider.unwrap();
            match provider.get_block_number().await {
                Ok(block_num) => {
                    let result = format!("Block #{}", block_num);
                    log::info!("Current {}", result);
                    set_block_number_result.set(Some(result));
                }
                Err(e) => {
                    let err_msg = format!("Failed: {:?}", e);
                    log::error!("{}", err_msg);
                    set_block_number_result.set(Some(err_msg));
                }
            }
        });
    };

    // Handler: Send Transaction
    let handle_send_transaction = move |_| {
        // Read values before entering async context
        let provider = wallet.provider_untracked();
        let addr = wallet.address_untracked();

        if provider.is_none() || addr.is_none() {
            set_send_tx_result.set(Some("Not connected".to_string()));
            return;
        }

        set_send_tx_result.set(Some("Switching to Gnosis Chain...".to_string()));
        spawn_local(async move {
            use alloy::rpc::types::TransactionRequest;

            let provider = provider.unwrap();
            let from_addr = addr.unwrap();

            // First, switch to Gnosis Chain (chain ID 100) using direct RPC call
            log::info!("Switching to Gnosis Chain...");

            // Get ethereum provider to switch chains
            match Eip1193Transport::get_ethereum() {
                Ok(ethereum) => {
                    // Call wallet_switchEthereumChain directly using the transport
                    let transport = Eip1193Transport::new(ethereum);

                    // Define the chain switch params structure
                    // serde_wasm_bindgen requires actual Rust structs, not serde_json::Value
                    #[derive(serde::Serialize)]
                    struct ChainIdParam {
                        #[serde(rename = "chainId")]
                        chain_id: String,
                    }

                    let switch_params = vec![ChainIdParam {
                        chain_id: "0x64".to_string(), // 100 in hex = Gnosis Chain
                    }];

                    match transport.request::<_, ()>("wallet_switchEthereumChain", switch_params).await {
                        Ok(_) => {
                            log::info!("Switched to Gnosis Chain");
                            set_send_tx_result.set(Some("Creating transaction on Gnosis...".to_string()));
                        }
                        Err(e) => {
                            let user_msg = format!("❌ {}", e.user_message());
                            log::error!("Chain switch error: {} (code: {})", e, e.code());
                            set_send_tx_result.set(Some(user_msg));
                            return;
                        }
                    }
                }
                Err(e) => {
                    let err_msg = format!("Failed to get ethereum: {:?}", e);
                    log::error!("{}", err_msg);
                    set_send_tx_result.set(Some(err_msg));
                    return;
                }
            }

            // Small delay to let the chain switch complete
            gloo_timers::future::sleep(std::time::Duration::from_millis(500)).await;

            // Create a transaction sending 0.00001 xDAI to self (10^13 wei)
            let tx = TransactionRequest::default()
                .from(from_addr)
                .to(from_addr)
                .value(alloy::primitives::U256::from(10_000_000_000_000u128)); // 0.00001 xDAI

            log::info!("Sending transaction on Gnosis Chain: {:?}", tx);
            set_send_tx_result.set(Some("Waiting for approval...".to_string()));

            match provider.send_transaction(tx).await {
                Ok(pending_tx) => {
                    let tx_hash = *pending_tx.tx_hash();
                    let result = format!("✅ Tx sent on Gnosis!\nHash: {:?}\nView: https://gnosisscan.io/tx/{:?}", tx_hash, tx_hash);
                    log::info!("Transaction sent: {:?}", tx_hash);
                    set_send_tx_result.set(Some(result));
                }
                Err(e) => {
                    // Use the helper to extract EIP-1193 error and format user message
                    let user_msg = format_transport_error(&e);
                    log::error!("Transaction error: {}", e);
                    set_send_tx_result.set(Some(user_msg));
                }
            }
        });
    };

    view! {
        <Show when=move || wallet.is_connected.get()>
            <div
                class="rounded-2xl p-8 shadow-lg max-w-md w-full space-y-6 mt-6"
                style=move || format!(
                    "background: {}; transition: background 0.3s ease;",
                    card_bg.run(())
                )
            >
                // Wallet Signing Section
                <div class="space-y-4">
                    <h2
                        class="text-2xl font-bold"
                        style=move || format!("color: {};", text_color.run(()))
                    >
                        "🔐 Wallet Signing"
                    </h2>

                    <div class="flex flex-col gap-2">
                        <button
                            class="px-4 py-2 rounded-lg font-medium text-sm"
                            style="background: var(--nk-colors-accentColor); color: var(--nk-colors-accentColorForeground);"
                            on:click=handle_personal_sign
                            disabled=move || wallet.is_connecting.get()
                        >
                            "Sign Message (personal_sign)"
                        </button>

                        {move || signature_result.get().map(|sig| view! {
                            <div
                                class="p-3 rounded-lg text-xs break-all"
                                style="background: rgba(0,0,0,0.1);"
                            >
                                <code style=move || format!("color: {};", text_color.run(()))>
                                    {sig}
                                </code>
                            </div>
                        })}
                    </div>

                    <div class="flex flex-col gap-2">
                        <button
                            class="px-4 py-2 rounded-lg font-medium text-sm"
                            style="background: var(--nk-colors-accentColor); color: var(--nk-colors-accentColorForeground);"
                            on:click=handle_typed_sign
                            disabled=move || wallet.is_connecting.get()
                        >
                            "Sign Typed Data (EIP-712)"
                        </button>

                        {move || typed_signature_result.get().map(|sig| view! {
                            <div
                                class="p-3 rounded-lg text-xs break-all"
                                style="background: rgba(0,0,0,0.1);"
                            >
                                <code style=move || format!("color: {};", text_color.run(()))>
                                    {sig}
                                </code>
                            </div>
                        })}
                    </div>
                </div>

                // RPC Read Section
                <div class="space-y-4 pt-4 border-t" style="border-color: rgba(128,128,128,0.2);">
                    <h2
                        class="text-2xl font-bold"
                        style=move || format!("color: {};", text_color.run(()))
                    >
                        "📡 RPC Read Operations"
                    </h2>

                    <div class="flex flex-col gap-2">
                        <button
                            class="px-4 py-2 rounded-lg font-medium text-sm"
                            style="background: var(--nk-colors-accentColor); color: var(--nk-colors-accentColorForeground);"
                            on:click=handle_fetch_my_balance
                            disabled=move || wallet.is_connecting.get()
                        >
                            "Get My Balance"
                        </button>

                        {move || balance_result.get().map(|result| view! {
                            <div
                                class="p-3 rounded-lg text-sm"
                                style="background: rgba(0,0,0,0.1);"
                            >
                                <span style=move || format!("color: {};", text_color.run(()))>
                                    {result}
                                </span>
                            </div>
                        })}
                    </div>

                    <div class="flex flex-col gap-2">
                        <button
                            class="px-4 py-2 rounded-lg font-medium text-sm"
                            style="background: var(--nk-colors-accentColor); color: var(--nk-colors-accentColorForeground);"
                            on:click=handle_fetch_vitalik_balance
                            disabled=move || wallet.is_connecting.get()
                        >
                            "Get vitalik.eth Balance"
                        </button>

                        {move || vitalik_balance_result.get().map(|result| view! {
                            <div
                                class="p-3 rounded-lg text-sm"
                                style="background: rgba(0,0,0,0.1);"
                            >
                                <span style=move || format!("color: {};", text_color.run(()))>
                                    {result}
                                </span>
                            </div>
                        })}
                    </div>

                    <div class="flex flex-col gap-2">
                        <button
                            class="px-4 py-2 rounded-lg font-medium text-sm"
                            style="background: var(--nk-colors-accentColor); color: var(--nk-colors-accentColorForeground);"
                            on:click=handle_fetch_block_number
                            disabled=move || wallet.is_connecting.get()
                        >
                            "Get Current Block Number"
                        </button>

                        {move || block_number_result.get().map(|result| view! {
                            <div
                                class="p-3 rounded-lg text-sm"
                                style="background: rgba(0,0,0,0.1);"
                            >
                                <span style=move || format!("color: {};", text_color.run(()))>
                                    {result}
                                </span>
                            </div>
                        })}
                    </div>

                    <div class="flex flex-col gap-2">
                        <button
                            class="px-4 py-2 rounded-lg font-medium text-sm"
                            style="background: var(--nk-colors-accentColor); color: var(--nk-colors-accentColorForeground);"
                            on:click=handle_send_transaction
                            disabled=move || wallet.is_connecting.get()
                        >
                            "Send 0.00001 xDAI on Gnosis Chain"
                        </button>

                        {move || send_tx_result.get().map(|result| view! {
                            <div
                                class="p-3 rounded-lg text-sm break-all"
                                style="background: rgba(0,0,0,0.1);"
                            >
                                <span style=move || format!("color: {};", text_color.run(()))>
                                    {result}
                                </span>
                            </div>
                        })}
                    </div>
                </div>
            </div>
        </Show>
    }
}
