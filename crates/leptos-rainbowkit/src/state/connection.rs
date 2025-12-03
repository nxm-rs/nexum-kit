use leptos::prelude::*;
use alloy::primitives::Address;
use alloy::providers::ProviderBuilder;
use crate::wallets::wallet::WalletConnector;
use crate::provider::{Eip1193Signer, Eip1193Transport};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::js_sys;
use std::collections::HashMap;
use std::sync::Arc;

/// Connection status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
}

/// Combined provider type: We use Arc<dyn Provider> to make it cloneable for Leptos signals
/// Combines HTTP transport for RPC with EIP-1193 signer for wallet operations
pub type WalletProvider = Arc<dyn alloy::providers::Provider>;

/// Connection state - manages wallet connection lifecycle
///
/// This uses Leptos signals for reactive state management, similar to how
/// the original RainbowKit uses React state + wagmi hooks.
///
/// Provides an Alloy provider that combines:
/// - HTTP transport for blockchain RPC (consumer-provided)
/// - EIP-1193 signer for wallet signing operations
///
/// This matches the wagmi v2 architecture where connectors provide signing
/// and consumers provide RPC infrastructure.
#[derive(Clone)]
pub struct ConnectionState {
    pub status: RwSignal<ConnectionStatus>,
    pub address: RwSignal<Option<Address>>,
    pub chain_id: RwSignal<Option<u64>>,
    pub connector_id: RwSignal<Option<String>>,
    pub provider: RwSignal<Option<WalletProvider>>,
    /// Consumer-provided RPC URLs for each chain
    pub(crate) transports: HashMap<u64, String>,
}

impl ConnectionState {
    pub fn new(transports: HashMap<u64, String>) -> Self {
        Self {
            status: RwSignal::new(ConnectionStatus::Disconnected),
            address: RwSignal::new(None),
            chain_id: RwSignal::new(None),
            connector_id: RwSignal::new(None),
            provider: RwSignal::new(None),
            transports,
        }
    }

    /// Get the Alloy provider if connected
    ///
    /// Returns a fully-featured Alloy provider that combines:
    /// - HTTP transport for blockchain RPC operations (reading state, sending txs)
    /// - EIP-1193 signer for wallet signing operations
    ///
    /// This provider can be used for any Alloy operations including:
    /// - Reading blockchain state (get_balance, get_block, etc.)
    /// - Sending transactions (send_transaction)
    /// - Contract interactions
    /// - Everything else Alloy supports
    pub fn get_provider(&self) -> Option<WalletProvider> {
        self.provider.get()
    }

    /// Setup EIP-1193 event listeners for automatic state synchronization
    ///
    /// This sets up listeners for all required EIP-1193 events:
    /// - accountsChanged: Updates address when user switches account in wallet
    /// - chainChanged: Updates chain_id when user switches network in wallet
    /// - disconnect: Clears state when wallet disconnects
    /// - connect: Handles wallet reconnection
    ///
    /// Note: The closures are leaked (using `forget`) because:
    /// 1. They need to live for the lifetime of the browser session
    /// 2. Storing them would require Send + Sync, which Closure doesn't implement
    /// 3. In WASM, memory leaks are less critical as the page will eventually reload
    fn setup_event_listeners(&self, ethereum: &JsValue) {

        // accountsChanged listener
        {
            let address_signal = self.address;
            let status_signal = self.status;

            let closure = Closure::wrap(Box::new(move |accounts: JsValue| {
                // Guard: Only process if currently connected
                // This prevents the leaked closure from affecting disconnected state
                let current_status = status_signal.get_untracked();
                if current_status == ConnectionStatus::Disconnected {
                    log::debug!("accountsChanged event ignored - wallet is disconnected");
                    return;
                }

                log::debug!("accountsChanged event fired");

                if let Ok(accounts_array) = accounts.dyn_into::<js_sys::Array>() {
                    if accounts_array.length() > 0 {
                        if let Some(account_str) = accounts_array.get(0).as_string() {
                            match account_str.parse::<Address>() {
                                Ok(address) => {
                                    log::info!("Account changed to: {:?}", address);
                                    address_signal.set(Some(address));
                                }
                                Err(e) => log::error!("Failed to parse address: {}", e),
                            }
                        }
                    } else {
                        // Empty array = disconnected
                        log::info!("Accounts array empty, wallet disconnected");
                        address_signal.set(None);
                        status_signal.set(ConnectionStatus::Disconnected);
                    }
                }
            }) as Box<dyn FnMut(JsValue)>);

            if let Ok(on_fn) = js_sys::Reflect::get(ethereum, &"on".into()) {
                if let Ok(on_fn) = on_fn.dyn_into::<js_sys::Function>() {
                    let _ = on_fn.call2(
                        ethereum,
                        &"accountsChanged".into(),
                        closure.as_ref().unchecked_ref(),
                    );
                }
            }

            // Leak the closure so it lives for the lifetime of the page
            closure.forget();
        }

        // chainChanged listener
        {
            let chain_id_signal = self.chain_id;
            let status_signal = self.status;

            let closure = Closure::wrap(Box::new(move |chain_id_hex: JsValue| {
                // Guard: Only process if currently connected
                // This prevents the leaked closure from affecting disconnected state
                let current_status = status_signal.get_untracked();
                if current_status == ConnectionStatus::Disconnected {
                    log::debug!("chainChanged event ignored - wallet is disconnected");
                    return;
                }

                log::debug!("chainChanged event fired");

                if let Some(chain_id_str) = chain_id_hex.as_string() {
                    // Parse hex chain ID (e.g., "0x1" -> 1)
                    let chain_id_str = chain_id_str.trim_start_matches("0x");
                    match u64::from_str_radix(chain_id_str, 16) {
                        Ok(chain_id) => {
                            log::info!("Chain changed to: {}", chain_id);
                            chain_id_signal.set(Some(chain_id));
                        }
                        Err(e) => log::error!("Failed to parse chain ID: {}", e),
                    }
                }
            }) as Box<dyn FnMut(JsValue)>);

            if let Ok(on_fn) = js_sys::Reflect::get(ethereum, &"on".into()) {
                if let Ok(on_fn) = on_fn.dyn_into::<js_sys::Function>() {
                    let _ = on_fn.call2(
                        ethereum,
                        &"chainChanged".into(),
                        closure.as_ref().unchecked_ref(),
                    );
                }
            }

            // Leak the closure so it lives for the lifetime of the page
            closure.forget();
        }

        // disconnect listener
        {
            let status_signal = self.status;
            let address_signal = self.address;
            let chain_id_signal = self.chain_id;
            let provider_signal = self.provider;
            let connector_id_signal = self.connector_id;

            let closure = Closure::wrap(Box::new(move |_error: JsValue| {
                // Guard: Only process if not already disconnected
                // This prevents duplicate disconnection handling
                let current_status = status_signal.get_untracked();
                if current_status == ConnectionStatus::Disconnected {
                    log::debug!("disconnect event ignored - already disconnected");
                    return;
                }

                log::info!("Wallet disconnect event - clearing connection state");
                status_signal.set(ConnectionStatus::Disconnected);
                address_signal.set(None);
                chain_id_signal.set(None);
                provider_signal.set(None);
                connector_id_signal.set(None);
            }) as Box<dyn FnMut(JsValue)>);

            if let Ok(on_fn) = js_sys::Reflect::get(ethereum, &"on".into()) {
                if let Ok(on_fn) = on_fn.dyn_into::<js_sys::Function>() {
                    let _ = on_fn.call2(
                        ethereum,
                        &"disconnect".into(),
                        closure.as_ref().unchecked_ref(),
                    );
                }
            }

            // Leak the closure so it lives for the lifetime of the page
            closure.forget();
        }

        // connect listener
        {
            let status_signal = self.status;
            let chain_id_signal = self.chain_id;

            let closure = Closure::wrap(Box::new(move |connect_info: JsValue| {
                // Guard: Only process if we're in a connecting or disconnected state
                // This prevents spurious connect events when already connected
                let current_status = status_signal.get_untracked();
                if current_status == ConnectionStatus::Connected {
                    log::debug!("connect event ignored - already connected");
                    return;
                }

                log::info!("Wallet connect event received");
                if let Ok(chain_id_hex) = js_sys::Reflect::get(&connect_info, &"chainId".into()) {
                    if let Some(chain_id_str) = chain_id_hex.as_string() {
                        let chain_id_str = chain_id_str.trim_start_matches("0x");
                        if let Ok(chain_id) = u64::from_str_radix(chain_id_str, 16) {
                            log::debug!("Setting chain ID from connect event: {}", chain_id);
                            chain_id_signal.set(Some(chain_id));
                        }
                    }
                }
                // Note: We don't set status to Connected here, as the full connection
                // flow handles this. This event is primarily for chain ID updates.
            }) as Box<dyn FnMut(JsValue)>);

            if let Ok(on_fn) = js_sys::Reflect::get(ethereum, &"on".into()) {
                if let Ok(on_fn) = on_fn.dyn_into::<js_sys::Function>() {
                    let _ = on_fn.call2(
                        ethereum,
                        &"connect".into(),
                        closure.as_ref().unchecked_ref(),
                    );
                }
            }

            // Leak the closure so it lives for the lifetime of the page
            closure.forget();
        }

        log::info!("EIP-1193 event listeners setup complete");
    }

    /// Connect to a wallet
    ///
    /// This is the main connection method. It:
    /// 1. Checks if already connecting to prevent duplicate requests
    /// 2. Sets status to Connecting
    /// 3. Calls the connector's connect() method
    /// 4. Creates HTTP provider with consumer's RPC URL
    /// 5. Creates EIP-1193 signer from wallet
    /// 6. Combines them into a provider
    /// 7. Sets up EIP-1193 event listeners for auto-sync
    /// 8. Updates all state signals on success
    pub async fn connect<C: WalletConnector>(&self, connector: &C) -> Result<(), JsValue> {
        // Prevent duplicate connection attempts
        if self.status.get_untracked() == ConnectionStatus::Connecting {
            log::warn!("Connection already in progress, ignoring duplicate request");
            return Err(JsValue::from_str("Connection already in progress"));
        }

        // Already connected to this wallet
        if self.status.get_untracked() == ConnectionStatus::Connected
            && self.connector_id.get_untracked().as_ref() == Some(&connector.metadata().id) {
            log::info!("Already connected to {}", connector.metadata().name);
            return Ok(());
        }

        log::info!("Connecting to wallet: {}", connector.metadata().name);
        self.status.set(ConnectionStatus::Connecting);

        match connector.connect().await {
            Ok(address) => {
                log::info!("Successfully connected: {:?}", address);

                // Get the ethereum provider from the connector
                let ethereum_js = connector.get_provider()
                    .ok_or_else(|| JsValue::from_str("Connector did not provide ethereum provider"))?;

                // Create EIP-1193 signer from wallet
                let signer = Eip1193Signer::new(ethereum_js.clone(), address);

                // Get current chain ID from wallet
                let transport = Eip1193Transport::new(ethereum_js.clone());
                let chain_id = self.get_current_chain_id(&transport).await?;

                // Get consumer's RPC URL for this chain
                let rpc_url = self.transports.get(&chain_id)
                    .ok_or_else(|| JsValue::from_str(&format!("No RPC URL configured for chain {}", chain_id)))?
                    .clone();

                log::info!("Using RPC URL: {} for chain {}", rpc_url, chain_id);

                // Create HTTP provider with consumer's RPC + wallet signer
                let url: reqwest::Url = rpc_url.parse().map_err(|e| JsValue::from_str(&format!("Invalid RPC URL: {}", e)))?;
                let provider = ProviderBuilder::new()
                    .wallet(signer)
                    .connect_http(url);

                // Wrap in Arc to make it cloneable for Leptos signals
                let provider: WalletProvider = Arc::new(provider);

                // Setup EIP-1193 event listeners for automatic state sync
                self.setup_event_listeners(&ethereum_js);

                // Update all state
                self.address.set(Some(address));
                self.chain_id.set(Some(chain_id));
                self.connector_id.set(Some(connector.metadata().id.clone()));
                self.provider.set(Some(provider));
                self.status.set(ConnectionStatus::Connected);

                log::info!("Connection successful, provider created with HTTP transport + EIP-1193 signer");
                Ok(())
            }
            Err(e) => {
                log::error!("Failed to connect: {:?}", e);
                self.status.set(ConnectionStatus::Disconnected);
                self.provider.set(None);
                Err(e)
            }
        }
    }

    /// Get current chain ID from wallet
    async fn get_current_chain_id(&self, transport: &Eip1193Transport) -> Result<u64, JsValue> {
        let request_obj = js_sys::Object::new();
        js_sys::Reflect::set(&request_obj, &"method".into(), &"eth_chainId".into())
            .map_err(|e| JsValue::from_str(&format!("Failed to create request: {:?}", e)))?;
        js_sys::Reflect::set(&request_obj, &"params".into(), &js_sys::Array::new())
            .map_err(|e| JsValue::from_str(&format!("Failed to set params: {:?}", e)))?;

        let request_fn = js_sys::Reflect::get(transport.ethereum(), &"request".into())
            .map_err(|e| JsValue::from_str(&format!("Failed to get request fn: {:?}", e)))?;
        let request_fn = request_fn
            .dyn_into::<js_sys::Function>()
            .map_err(|e| JsValue::from_str(&format!("Request is not a function: {:?}", e)))?;

        let promise = request_fn
            .call1(transport.ethereum(), &request_obj)
            .map_err(|e| JsValue::from_str(&format!("Failed to call request: {:?}", e)))?;
        let promise = promise
            .dyn_into::<js_sys::Promise>()
            .map_err(|e| JsValue::from_str(&format!("Not a promise: {:?}", e)))?;

        let result = wasm_bindgen_futures::JsFuture::from(promise).await
            .map_err(|e| JsValue::from_str(&format!("Request failed: {:?}", e)))?;

        let chain_id_hex = result.as_string()
            .ok_or_else(|| JsValue::from_str("Chain ID is not a string"))?;

        let chain_id_hex = chain_id_hex.trim_start_matches("0x");
        u64::from_str_radix(chain_id_hex, 16)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse chain ID: {}", e)))
    }

    /// Disconnect from the wallet
    pub async fn disconnect(&self) -> Result<(), JsValue> {
        log::info!("Disconnecting wallet");

        // Note: Event listeners are leaked and will remain active.
        // This is intentional as they handle wallet-side disconnections.

        // Clear all state
        self.address.set(None);
        self.chain_id.set(None);
        self.connector_id.set(None);
        self.provider.set(None);
        self.status.set(ConnectionStatus::Disconnected);
        Ok(())
    }

    /// Check if currently connected
    pub fn is_connected(&self) -> bool {
        matches!(self.status.get(), ConnectionStatus::Connected)
    }

    /// Check if currently connecting
    pub fn is_connecting(&self) -> bool {
        matches!(self.status.get(), ConnectionStatus::Connecting)
    }
}

/// Provide connection state to the component tree
///
/// Call this in your RainbowKitProvider to make connection state
/// available to all child components.
///
/// # Arguments
/// * `transports` - Map of chain_id to RPC URL for blockchain communication
pub fn provide_connection_state(transports: HashMap<u64, String>) -> ConnectionState {
    let state = ConnectionState::new(transports);
    provide_context(state.clone());
    state
}

/// Access connection state from any component
///
/// This will panic if called outside of a RainbowKitProvider.
/// Use in components that need to access wallet connection state.
pub fn use_connection_state() -> ConnectionState {
    expect_context::<ConnectionState>()
}
