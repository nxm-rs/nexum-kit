use leptos::prelude::*;
use crate::state::connection::{use_connection_state, ConnectionStatus, WalletProvider};
use alloy::primitives::Address;

/// Wallet information including address, connection status, and Alloy provider
///
/// The provider combines:
/// - HTTP transport for blockchain RPC operations (consumer-provided)
/// - EIP-1193 signer for wallet signing operations
///
/// This matches the wagmi v2 architecture.
pub struct WalletInfo {
    pub address: Signal<Option<Address>>,
    pub chain_id: Signal<Option<u64>>,
    pub is_connected: Signal<bool>,
    pub is_connecting: Signal<bool>,
    pub connector_id: Signal<Option<String>>,
    /// Alloy provider combining HTTP transport + EIP-1193 signer
    ///
    /// This provider implements the full Alloy `Provider` trait, so you can:
    /// - Read state: `provider.get_balance(addr).await`
    /// - Send transactions: `provider.send_transaction(tx).await`
    /// - Interact with contracts: Use with Alloy contract bindings
    /// - Everything else Alloy supports
    ///
    /// Note: Chain changes are automatically synced via EIP-1193 events
    pub provider: Signal<Option<WalletProvider>>,
}

/// Hook to access wallet connection information and Alloy provider
///
/// # Example
/// ```no_run
/// use leptos::prelude::*;
/// use leptos_rainbowkit::prelude::*;
/// use alloy::rpc::types::TransactionRequest;
///
/// #[component]
/// fn MyComponent() -> impl IntoView {
///     let wallet = use_wallet();
///
///     // Get the Alloy provider and use it for blockchain operations
///     let send_transaction = move || {
///         spawn_local(async move {
///             if let Some(provider) = wallet.provider.get() {
///                 // Now you can use any Alloy provider methods!
///                 let balance = provider.get_balance(address).await.unwrap();
///
///                 let tx = TransactionRequest::default()
///                     .to(recipient)
///                     .value(amount);
///                 let pending = provider.send_transaction(tx).await.unwrap();
///             }
///         });
///     };
///
///     view! {
///         <Show when=move || wallet.is_connected.get()>
///             <button on:click=move |_| send_transaction()>
///                 "Send Transaction"
///             </button>
///         </Show>
///     }
/// }
/// ```
pub fn use_wallet() -> WalletInfo {
    let state = use_connection_state();

    let address = Signal::derive(move || state.address.get());
    let chain_id = Signal::derive(move || state.chain_id.get());
    let is_connected = Signal::derive(move || state.status.get() == ConnectionStatus::Connected);
    let is_connecting = Signal::derive(move || state.status.get() == ConnectionStatus::Connecting);
    let connector_id = Signal::derive(move || state.connector_id.get());
    let provider = Signal::derive(move || state.provider.get());

    WalletInfo {
        address,
        chain_id,
        is_connected,
        is_connecting,
        connector_id,
        provider,
    }
}
