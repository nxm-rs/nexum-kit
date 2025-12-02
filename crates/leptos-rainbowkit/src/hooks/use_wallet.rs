use leptos::prelude::*;
use crate::state::connection::{use_connection_state, ConnectionStatus};
use alloy::primitives::Address;

/// Wallet information hook
///
/// This provides reactive access to wallet connection state.
/// Similar to wagmi's useAccount hook in the original RainbowKit.
pub struct WalletInfo {
    pub address: Signal<Option<Address>>,
    pub is_connected: Signal<bool>,
    pub is_connecting: Signal<bool>,
    pub connector_id: Signal<Option<String>>,
}

/// Access wallet connection information
///
/// Returns reactive signals for:
/// - `address`: Currently connected wallet address (if any)
/// - `is_connected`: Whether a wallet is currently connected
/// - `is_connecting`: Whether a connection is in progress
/// - `connector_id`: ID of the connected wallet (e.g., "metamask")
///
/// # Example
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_rainbowkit::hooks::use_wallet;
///
/// #[component]
/// pub fn MyComponent() -> impl IntoView {
///     let wallet = use_wallet();
///
///     view! {
///         <Show when=move || wallet.is_connected.get()>
///             <p>"Connected: " {move || format!("{:?}", wallet.address.get())}</p>
///         </Show>
///     }
/// }
/// ```
pub fn use_wallet() -> WalletInfo {
    let state = use_connection_state();

    let address = Signal::derive(move || state.address.get());
    let is_connected = Signal::derive(move || state.status.get() == ConnectionStatus::Connected);
    let is_connecting = Signal::derive(move || state.status.get() == ConnectionStatus::Connecting);
    let connector_id = Signal::derive(move || state.connector_id.get());

    WalletInfo {
        address,
        is_connected,
        is_connecting,
        connector_id,
    }
}
