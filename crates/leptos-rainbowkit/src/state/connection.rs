use leptos::prelude::*;
use alloy::primitives::Address;
use crate::wallets::wallet::WalletConnector;
use wasm_bindgen::JsValue;

/// Connection status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
}

/// Connection state - manages wallet connection lifecycle
///
/// This uses Leptos signals for reactive state management, similar to how
/// the original RainbowKit uses React state + wagmi hooks.
#[derive(Debug, Clone, Copy)]
pub struct ConnectionState {
    pub status: RwSignal<ConnectionStatus>,
    pub address: RwSignal<Option<Address>>,
    pub connector_id: RwSignal<Option<String>>,
}

impl ConnectionState {
    pub fn new() -> Self {
        Self {
            status: RwSignal::new(ConnectionStatus::Disconnected),
            address: RwSignal::new(None),
            connector_id: RwSignal::new(None),
        }
    }

    /// Connect to a wallet
    ///
    /// This is the main connection method. It:
    /// 1. Checks if already connecting to prevent duplicate requests
    /// 2. Sets status to Connecting
    /// 3. Calls the connector's connect() method
    /// 4. Updates address and connector_id on success
    /// 5. Sets status to Connected or reverts to Disconnected on error
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
                self.address.set(Some(address));
                self.connector_id.set(Some(connector.metadata().id.clone()));
                self.status.set(ConnectionStatus::Connected);
                Ok(())
            }
            Err(e) => {
                log::error!("Failed to connect: {:?}", e);
                self.status.set(ConnectionStatus::Disconnected);
                Err(e)
            }
        }
    }

    /// Disconnect from the wallet
    pub async fn disconnect(&self) -> Result<(), JsValue> {
        log::info!("Disconnecting wallet");
        self.address.set(None);
        self.connector_id.set(None);
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

impl Default for ConnectionState {
    fn default() -> Self {
        Self::new()
    }
}

/// Provide connection state to the component tree
///
/// Call this in your RainbowKitProvider to make connection state
/// available to all child components.
pub fn provide_connection_state() -> ConnectionState {
    let state = ConnectionState::new();
    provide_context(state);
    state
}

/// Access connection state from any component
///
/// This will panic if called outside of a RainbowKitProvider.
/// Use in components that need to access wallet connection state.
pub fn use_connection_state() -> ConnectionState {
    expect_context::<ConnectionState>()
}
