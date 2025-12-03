//! Chain configuration for wallet operations
//!
//! This module provides types and utilities for configuring chains when adding them
//! to browser wallets via EIP-1193's `wallet_addEthereumChain` method.

use alloy_chains::{Chain, NamedChain};

/// Chain configuration for adding new networks to the wallet
///
/// Uses `alloy_chains::Chain` to represent the chain, which automatically provides
/// chain ID, name, and native currency symbol. Only RPC URLs and block explorers
/// need to be specified by the user.
#[derive(Debug, Clone)]
pub struct ChainConfig {
    /// Chain (from alloy-chains) - includes chain ID, name, and native currency symbol
    pub chain: Chain,
    /// Array of RPC endpoint URLs
    pub rpc_urls: Vec<String>,
    /// Array of block explorer URLs (optional)
    pub block_explorer_urls: Vec<String>,
    /// Override for native currency name (if None, defaults to symbol)
    pub native_currency_name: Option<String>,
    /// Override for native currency decimals (if None, defaults to 18)
    pub native_currency_decimals: Option<u8>,
}

impl ChainConfig {
    /// Create a builder for ChainConfig
    ///
    /// # Example
    /// ```rust,ignore
    /// use alloy_chains::NamedChain;
    ///
    /// let config = ChainConfig::builder()
    ///     .chain(NamedChain::Polygon)
    ///     .rpc_url("https://polygon-rpc.com")
    ///     .block_explorer("https://polygonscan.com")
    ///     .build();
    /// ```
    pub fn builder() -> ChainConfigBuilder<NeedsChain> {
        ChainConfigBuilder::new()
    }

    /// Get the chain ID
    #[inline]
    pub fn chain_id(&self) -> u64 {
        self.chain.id()
    }

    /// Get the chain name
    #[inline]
    pub fn chain_name(&self) -> String {
        self.chain.to_string()
    }

    /// Get the native currency symbol (if available from alloy-chains)
    #[inline]
    pub fn native_currency_symbol(&self) -> Option<String> {
        NamedChain::try_from(self.chain)
            .ok()
            .and_then(|c| c.native_currency_symbol())
            .map(|s| s.to_string())
    }
}

/// Builder state: Chain needs to be specified
#[derive(Debug)]
pub struct NeedsChain;

/// Builder state: Chain has been specified
#[derive(Debug)]
pub struct HasChain;

/// Builder for ChainConfig with compile-time state tracking
///
/// This builder ensures that required fields (chain) are set before building.
///
/// # Example
/// ```rust,ignore
/// use alloy_chains::NamedChain;
///
/// let config = ChainConfig::builder()
///     .chain(NamedChain::Polygon)
///     .rpc_url("https://polygon-rpc.com")
///     .rpc_url("https://polygon-backup.com")  // Can add multiple RPCs
///     .block_explorer("https://polygonscan.com")
///     .currency_name("Matic Token")  // Optional overrides
///     .currency_decimals(18)
///     .build();
/// ```
#[derive(Debug)]
pub struct ChainConfigBuilder<State = NeedsChain> {
    chain: Option<Chain>,
    rpc_urls: Vec<String>,
    block_explorer_urls: Vec<String>,
    native_currency_name: Option<String>,
    native_currency_decimals: Option<u8>,
    _state: std::marker::PhantomData<State>,
}

impl ChainConfigBuilder<NeedsChain> {
    fn new() -> Self {
        Self {
            chain: None,
            rpc_urls: Vec::new(),
            block_explorer_urls: Vec::new(),
            native_currency_name: None,
            native_currency_decimals: None,
            _state: std::marker::PhantomData,
        }
    }

    /// Set the chain (required)
    ///
    /// Accepts anything that converts to `Chain`: `NamedChain`, `u64`, etc.
    pub fn chain(mut self, chain: impl Into<Chain>) -> ChainConfigBuilder<HasChain> {
        self.chain = Some(chain.into());
        ChainConfigBuilder {
            chain: self.chain,
            rpc_urls: self.rpc_urls,
            block_explorer_urls: self.block_explorer_urls,
            native_currency_name: self.native_currency_name,
            native_currency_decimals: self.native_currency_decimals,
            _state: std::marker::PhantomData,
        }
    }
}

impl ChainConfigBuilder<HasChain> {
    /// Add an RPC URL (can be called multiple times)
    pub fn rpc_url(mut self, url: impl Into<String>) -> Self {
        self.rpc_urls.push(url.into());
        self
    }

    /// Set multiple RPC URLs at once
    pub fn rpc_urls(mut self, urls: impl IntoIterator<Item = String>) -> Self {
        self.rpc_urls.extend(urls);
        self
    }

    /// Add a block explorer URL (can be called multiple times)
    pub fn block_explorer(mut self, url: impl Into<String>) -> Self {
        self.block_explorer_urls.push(url.into());
        self
    }

    /// Set multiple block explorer URLs at once
    pub fn block_explorers(mut self, urls: impl IntoIterator<Item = String>) -> Self {
        self.block_explorer_urls.extend(urls);
        self
    }

    /// Override the native currency name
    pub fn currency_name(mut self, name: impl Into<String>) -> Self {
        self.native_currency_name = Some(name.into());
        self
    }

    /// Override the native currency decimals
    pub fn currency_decimals(mut self, decimals: u8) -> Self {
        self.native_currency_decimals = Some(decimals);
        self
    }

    /// Build the ChainConfig
    pub fn build(self) -> ChainConfig {
        ChainConfig {
            chain: self.chain.expect("Chain must be set"),
            rpc_urls: self.rpc_urls,
            block_explorer_urls: self.block_explorer_urls,
            native_currency_name: self.native_currency_name,
            native_currency_decimals: self.native_currency_decimals,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_with_named_chain() {
        let config = ChainConfig::builder()
            .chain(NamedChain::Polygon)
            .rpc_url("https://polygon-rpc.com")
            .block_explorer("https://polygonscan.com")
            .build();

        assert_eq!(config.chain_id(), 137);
        assert_eq!(config.native_currency_symbol(), Some("MATIC".to_string()));
        assert_eq!(config.rpc_urls, vec!["https://polygon-rpc.com"]);
    }

    #[test]
    fn test_builder_with_chain_id() {
        let config = ChainConfig::builder()
            .chain(1u64) // Ethereum mainnet
            .rpc_url("https://eth.llamarpc.com")
            .rpc_url("https://cloudflare-eth.com") // Multiple RPCs
            .block_explorer("https://etherscan.io")
            .build();

        assert_eq!(config.chain_id(), 1);
        assert_eq!(config.rpc_urls.len(), 2);
        assert_eq!(config.native_currency_symbol(), Some("ETH".to_string()));
    }

    #[test]
    fn test_builder_with_currency_overrides() {
        let config = ChainConfig::builder()
            .chain(NamedChain::Gnosis)
            .rpc_url("https://rpc.gnosischain.com")
            .block_explorer("https://gnosisscan.io")
            .currency_name("xDAI Token")
            .currency_decimals(18)
            .build();

        assert_eq!(config.chain_id(), 100);
        assert_eq!(config.native_currency_name, Some("xDAI Token".to_string()));
        assert_eq!(config.native_currency_decimals, Some(18));
    }

    #[test]
    fn test_builder_multiple_explorers() {
        let config = ChainConfig::builder()
            .chain(NamedChain::Polygon)
            .rpc_urls(vec![
                "https://polygon-rpc.com".to_string(),
                "https://polygon-backup.com".to_string(),
            ])
            .block_explorer("https://polygonscan.com")
            .block_explorer("https://explorer.matic.network")
            .build();

        assert_eq!(config.rpc_urls.len(), 2);
        assert_eq!(config.block_explorer_urls.len(), 2);
    }

    // This test won't compile - demonstrating type-safety
    // #[test]
    // fn test_builder_missing_chain() {
    //     let config = ChainConfig::builder()
    //         .rpc_url("https://rpc.example.com")
    //         .build(); // Error: build() not available without chain()
    // }
}
