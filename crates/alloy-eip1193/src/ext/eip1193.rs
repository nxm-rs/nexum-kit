//! EIP-1193 Provider Extension Trait
//!
//! This module provides the `Eip1193` trait that extends any Alloy provider with
//! EIP-1193 mandated wallet operations. These methods are automatically available
//! on any provider using an EIP-1193 compatible transport.

use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::network::Network;
use alloy::transports::{TransportResult, TransportErrorKind};
use async_trait::async_trait;
use crate::chain::ChainConfig;

/// EIP-1193 Provider Extension
///
/// This trait provides EIP-1193 mandated RPC methods as ergonomic APIs on Alloy providers.
/// All methods are automatically available on any `Provider<N>` implementation, as long as
/// the underlying transport supports the EIP-1193 protocol (e.g., `Eip1193Transport`).
///
/// # Mandated RPC Methods
///
/// This trait implements the following EIP-1193 methods:
/// - `eth_requestAccounts` - Request wallet accounts (user consent)
/// - `wallet_switchEthereumChain` - Switch active blockchain network
/// - `wallet_addEthereumChain` - Add a new blockchain network to wallet
/// - `wallet_watchAsset` - Request wallet to track a token
///
/// # Example
///
/// ```rust,ignore
/// use alloy::providers::ProviderBuilder;
/// use alloy_eip1193::{Eip1193Transport, ext::Eip1193};
///
/// let transport = Eip1193Transport::new(ethereum);
/// let provider = ProviderBuilder::new().on_transport(transport);
///
/// // EIP-1193 methods are automatically available
/// let accounts = provider.request_accounts().await?;
/// provider.switch_chain(137).await?; // Switch to Polygon
/// ```
///
/// # Trait Implementation
///
/// This trait is automatically implemented for all `Provider<N>` types via a blanket
/// implementation. The methods delegate to the provider's underlying transport using
/// the standard `client().request()` pattern.
#[async_trait(?Send)]
pub trait Eip1193<N: Network>: Send + Sync {
    /// Request accounts from the wallet (prompts user if needed)
    ///
    /// This method implements `eth_requestAccounts` which requests that the user
    /// provides an Ethereum address to be identified by. This causes a wallet UI
    /// popup for the user to select/authorize accounts.
    ///
    /// # EIP-1193 Specification
    ///
    /// - Method: `eth_requestAccounts`
    /// - Parameters: None
    /// - Returns: Array of address strings
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - User rejects the request (EIP-1193 error code 4001)
    /// - Accounts are unavailable
    /// - Invalid address format in response
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let accounts = provider.request_accounts().await?;
    /// println!("Connected account: {:?}", accounts[0]);
    /// ```
    async fn request_accounts(&self) -> TransportResult<Vec<Address>>;

    /// Switch to a different blockchain network
    ///
    /// This method implements `wallet_switchEthereumChain` which requests that the
    /// wallet switches its active chain to the specified chain ID.
    ///
    /// # EIP-1193 Specification
    ///
    /// - Method: `wallet_switchEthereumChain`
    /// - Parameters: `[{ chainId: "0x..." }]`
    /// - Returns: `null` on success
    ///
    /// # Arguments
    ///
    /// * `chain_id` - The chain ID to switch to (e.g., 1 for Ethereum mainnet, 137 for Polygon)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - User rejects the request (EIP-1193 error code 4001)
    /// - Chain is not configured in wallet (error code 4902)
    /// - Provider is disconnected (error code 4900)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Switch to Polygon
    /// provider.switch_chain(137).await?;
    ///
    /// // Switch to Ethereum mainnet
    /// provider.switch_chain(1).await?;
    /// ```
    async fn switch_chain(&self, chain_id: u64) -> TransportResult<()>;

    /// Add a new blockchain network to the wallet
    ///
    /// This method implements `wallet_addEthereumChain` which requests that the
    /// wallet tracks the specified chain. This is useful for adding custom networks
    /// or L2s that may not be preconfigured in the wallet.
    ///
    /// # EIP-1193 Specification
    ///
    /// - Method: `wallet_addEthereumChain`
    /// - Parameters: `[{ chainId, chainName, rpcUrls, nativeCurrency?, blockExplorerUrls?, iconUrls? }]`
    /// - Returns: `null` on success
    ///
    /// # Arguments
    ///
    /// * `config` - Chain configuration including ID, name, RPC URLs, and optional metadata
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - User rejects the request (EIP-1193 error code 4001)
    /// - Chain is already configured
    /// - Invalid chain configuration
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use alloy_eip1193::ChainConfig;
    ///
    /// let config = ChainConfig::builder()
    ///     .chain(42161) // Arbitrum One
    ///     .rpc_url("https://arb1.arbitrum.io/rpc")
    ///     .block_explorer("https://arbiscan.io")
    ///     .build();
    ///
    /// provider.add_chain(config).await?;
    /// ```
    async fn add_chain(&self, config: ChainConfig) -> TransportResult<()>;

    /// Request wallet to track a token
    ///
    /// This method implements `wallet_watchAsset` which requests that the wallet
    /// tracks the specified token. This allows users to view their token balance
    /// in their wallet UI.
    ///
    /// # EIP-1193 Specification
    ///
    /// - Method: `wallet_watchAsset`
    /// - Parameters: `[{ type: "ERC20", options: { address, symbol?, decimals?, image? } }]`
    /// - Returns: `true` if added, `false` if rejected
    ///
    /// # Arguments
    ///
    /// * `token` - Token contract address
    /// * `symbol` - Token symbol (e.g., "USDC")
    /// * `decimals` - Token decimals (usually 18 for ERC20)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Invalid token address
    /// - Wallet doesn't support `wallet_watchAsset`
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Add USDC to wallet
    /// let usdc = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse()?;
    /// let added = provider.watch_asset(usdc, "USDC", 6).await?;
    /// assert!(added);
    /// ```
    async fn watch_asset(
        &self,
        token: Address,
        symbol: &str,
        decimals: u8,
    ) -> TransportResult<bool>;

    /// Get currently connected accounts (without prompting)
    ///
    /// This method implements `eth_accounts` which returns accounts that the
    /// provider is already authorized to access, without prompting the user.
    ///
    /// # EIP-1193 Specification
    ///
    /// - Method: `eth_accounts`
    /// - Parameters: None
    /// - Returns: Array of address strings (may be empty)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Invalid address format in response
    /// - Transport error
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Check if already connected (no popup)
    /// let accounts = provider.accounts().await?;
    /// if accounts.is_empty() {
    ///     // Not connected, request permission
    ///     provider.request_accounts().await?;
    /// }
    /// ```
    async fn accounts(&self) -> TransportResult<Vec<Address>>;
}

/// Blanket implementation for any Provider
///
/// This implementation makes EIP-1193 methods available on any Alloy provider,
/// as long as the underlying transport supports these RPC methods. The implementation
/// uses the provider's `client().request()` to delegate calls to the transport layer.
///
/// # Transport Compatibility
///
/// This will work with any transport that implements the EIP-1193 RPC methods:
/// - `Eip1193Transport` - Direct browser wallet integration
/// - HTTP/WebSocket transports connected to EIP-1193 providers
/// - Custom transports that implement EIP-1193 methods
#[async_trait(?Send)]
impl<N, P> Eip1193<N> for P
where
    N: Network,
    P: Provider<N>,
{
    async fn request_accounts(&self) -> TransportResult<Vec<Address>> {
        let accounts: Vec<String> = self
            .client()
            .request("eth_requestAccounts", ())
            .await?;

        accounts
            .into_iter()
            .map(|s| {
                s.parse().map_err(|_| {
                    TransportErrorKind::custom_str("Invalid address format")
                })
            })
            .collect()
    }

    async fn switch_chain(&self, chain_id: u64) -> TransportResult<()> {
        let params = serde_json::json!([{
            "chainId": format!("0x{:x}", chain_id)
        }]);

        self.client()
            .request("wallet_switchEthereumChain", params)
            .await
    }

    async fn add_chain(&self, config: ChainConfig) -> TransportResult<()> {
        let symbol = config
            .native_currency_symbol()
            .unwrap_or_else(|| "ETH".to_string());
        let name = config
            .native_currency_name
            .clone()
            .unwrap_or_else(|| symbol.clone());
        let decimals = config.native_currency_decimals.unwrap_or(18);

        let params = serde_json::json!([{
            "chainId": format!("0x{:x}", config.chain_id()),
            "chainName": config.chain_name(),
            "rpcUrls": config.rpc_urls,
            "blockExplorerUrls": config.block_explorer_urls,
            "nativeCurrency": {
                "name": name,
                "symbol": symbol,
                "decimals": decimals,
            }
        }]);

        self.client()
            .request("wallet_addEthereumChain", params)
            .await
    }

    async fn watch_asset(
        &self,
        token: Address,
        symbol: &str,
        decimals: u8,
    ) -> TransportResult<bool> {
        let params = serde_json::json!([{
            "type": "ERC20",
            "options": {
                "address": format!("{:?}", token),
                "symbol": symbol,
                "decimals": decimals,
            }
        }]);

        self.client().request("wallet_watchAsset", params).await
    }

    async fn accounts(&self) -> TransportResult<Vec<Address>> {
        let accounts: Vec<String> = self
            .client()
            .request("eth_accounts", ())
            .await?;

        accounts
            .into_iter()
            .map(|s| {
                s.parse().map_err(|_| {
                    TransportErrorKind::custom_str("Invalid address format")
                })
            })
            .collect()
    }
}
