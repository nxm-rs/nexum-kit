use leptos::prelude::*;
use leptos::task::spawn_local;
use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder};
use wasm_bindgen::JsValue;

#[derive(Clone)]
pub struct BalanceInfo {
    pub value: Signal<Option<u128>>,
    pub formatted: Signal<String>,
    pub is_loading: Signal<bool>,
}

/// Hook to fetch and track the balance of an Ethereum address
///
/// # Arguments
/// * `address` - Signal containing the address to fetch balance for
/// * `rpc_url` - RPC URL to use for fetching the balance
///
/// # Returns
/// `BalanceInfo` containing the raw balance, formatted balance, and loading state
pub fn use_balance(
    address: Signal<Option<Address>>,
    rpc_url: &'static str,
) -> BalanceInfo {
    let (balance, set_balance) = signal(None::<u128>);
    let (is_loading, set_is_loading) = signal(false);

    Effect::new(move || {
        if let Some(addr) = address.get() {
            set_is_loading.set(true);

            spawn_local(async move {
                match fetch_balance(addr, rpc_url).await {
                    Ok(bal) => {
                        set_balance.set(Some(bal));
                    }
                    Err(e) => {
                        log::error!("Failed to fetch balance: {:?}", e);
                        set_balance.set(None);
                    }
                }
                set_is_loading.set(false);
            });
        } else {
            set_balance.set(None);
            set_is_loading.set(false);
        }
    });

    let formatted = Signal::derive(move || {
        balance.get()
            .map(|b| crate::utils::format::format_balance(b, 18))
            .unwrap_or_default()
    });

    BalanceInfo {
        value: balance.into(),
        formatted,
        is_loading: is_loading.into(),
    }
}

async fn fetch_balance(address: Address, rpc_url: &str) -> Result<u128, JsValue> {
    let provider = ProviderBuilder::new()
        .connect(rpc_url)
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to connect to provider: {:?}", e)))?;

    let balance = provider
        .get_balance(address)
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to get balance: {:?}", e)))?;

    // Convert U256 balance to u128
    let balance_u128 = balance.to::<u128>();
    Ok(balance_u128)
}
