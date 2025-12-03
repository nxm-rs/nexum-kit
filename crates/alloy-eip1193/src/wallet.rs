//! Wallet-specific EIP-1193 operations
//!
//! This module provides high-level methods for interacting with browser wallets,
//! including switching chains and adding new chains to the wallet.

use alloy::primitives::Address;
use wasm_bindgen::prelude::*;
use crate::transport::Eip1193Transport;
use crate::chain::ChainConfig;

/// Wallet operations helper
///
/// This wraps an EIP-1193 provider to provide convenient methods for
/// wallet-specific operations like switching chains or adding new chains.
#[derive(Clone, Debug)]
pub struct WalletOperations {
    transport: Eip1193Transport,
}

impl WalletOperations {
    /// Create a new WalletOperations instance
    pub fn new(ethereum: JsValue) -> Self {
        Self {
            transport: Eip1193Transport::new(ethereum),
        }
    }

    /// Request accounts from the wallet (prompts user if needed)
    ///
    /// This is essentially an alias for `wallet_requestPermissions` with eth_accounts.
    /// It prompts the user to connect their wallet and returns the list of accounts.
    ///
    /// # Example
    /// ```rust,ignore
    /// let wallet = WalletOperations::new(ethereum);
    /// let accounts = wallet.request_accounts().await?;
    /// ```
    pub async fn request_accounts(&self) -> Result<Vec<Address>, JsValue> {
        let empty_params: Vec<String> = Vec::new();
        let accounts: Vec<String> = self.transport
            .request("eth_requestAccounts", empty_params)
            .await?;

        accounts
            .into_iter()
            .map(|addr_str| {
                addr_str
                    .parse::<Address>()
                    .map_err(|e| JsValue::from_str(&format!("Invalid address format: {:?}", e)))
            })
            .collect()
    }

    /// Switch to a different chain
    ///
    /// # Arguments
    /// * `chain_id` - The chain ID to switch to
    ///
    /// # Example
    /// ```rust,ignore
    /// let wallet = WalletOperations::new(ethereum);
    /// wallet.switch_chain(137).await?; // Switch to Polygon
    /// ```
    pub async fn switch_chain(&self, chain_id: u64) -> Result<(), JsValue> {
        // EIP-1193 requires: params: [{ chainId: "0x..." }]
        let params = vec![serde_json::json!({
            "chainId": format!("0x{:x}", chain_id)
        })];

        self.transport
            .request::<_, serde_json::Value>("wallet_switchEthereumChain", params)
            .await?;

        Ok(())
    }

    /// Add a new chain to the wallet
    ///
    /// # Arguments
    /// * `config` - The chain configuration
    ///
    /// # Example
    /// ```rust,ignore
    /// use alloy_eip1193::{WalletOperations, ChainConfig};
    ///
    /// let wallet = WalletOperations::new(ethereum);
    ///
    /// // Add a known chain (currency info auto-derived from alloy-chains)
    /// let config = ChainConfig::new(
    ///     137, // Polygon
    ///     vec!["https://polygon-rpc.com".to_string()],
    ///     vec!["https://polygonscan.com".to_string()],
    /// );
    /// wallet.add_chain(config).await?;
    ///
    /// // Add a custom chain with explicit currency info
    /// let config = ChainConfig::new_custom(
    ///     12345,
    ///     "My Custom Chain".to_string(),
    ///     vec!["https://rpc.example.com".to_string()],
    ///     vec!["https://explorer.example.com".to_string()],
    ///     "Custom Token".to_string(),
    ///     "CTK".to_string(),
    ///     18,
    /// );
    /// wallet.add_chain(config).await?;
    /// ```
    pub async fn add_chain(&self, config: ChainConfig) -> Result<(), JsValue> {
        // EIP-1193 requires: params: [{ chainId, chainName, rpcUrls, nativeCurrency?, blockExplorerUrls }]
        let mut params_obj = serde_json::json!({
            "chainId": format!("0x{:x}", config.chain_id()),
            "chainName": config.chain_name(),
            "rpcUrls": config.rpc_urls,
            "blockExplorerUrls": config.block_explorer_urls,
        });

        // Build native currency object
        let symbol = config.native_currency_symbol().unwrap_or_else(|| "ETH".to_string());
        let name = config.native_currency_name.unwrap_or_else(|| symbol.clone());
        let decimals = config.native_currency_decimals.unwrap_or(18);

        params_obj["nativeCurrency"] = serde_json::json!({
            "name": name,
            "symbol": symbol,
            "decimals": decimals,
        });

        let params = vec![params_obj];

        self.transport
            .request::<_, serde_json::Value>("wallet_addEthereumChain", params)
            .await?;

        Ok(())
    }
}

// WASM is single-threaded, so Send/Sync are safe
unsafe impl Send for WalletOperations {}
unsafe impl Sync for WalletOperations {}
