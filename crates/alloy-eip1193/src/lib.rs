//! # alloy-eip1193
//!
//! EIP-1193 provider and signer implementation for Alloy in WebAssembly environments.
//!
//! This crate provides three complementary patterns for browser wallet integration:
//!
//! ## Components
//!
//! - **`Eip1193Transport`**: Tower Service implementation for JSON-RPC requests via browser wallets
//! - **`WalletLayer`**: Provider layer for smart request routing
//! - **`Eip1193Signer`**: Signer implementation (⚠️ uses eth_sign, shows warnings)
//! - **`ext::Eip1193`**: Trait extension for EIP-1193 mandated wallet operations (automatically available on any provider)
//!
//! ## Usage Patterns
//!
//! ### Pattern 1: Smart Routing with WalletLayer (Recommended)
//!
//! Use `WalletLayer` to add wallet operations to any provider:
//!
//! ```rust,ignore
//! use alloy::providers::ProviderBuilder;
//! use alloy_eip1193::{WalletLayer, ext::Eip1193};
//!
//! // Create wallet layer from window.ethereum
//! let wallet_layer = WalletLayer::from_window()?;
//!
//! // Add to any provider (RPC reads go to HTTP, wallet ops to browser wallet)
//! let provider = ProviderBuilder::new()
//!     .layer(wallet_layer)
//!     .on_http("https://eth.llamarpc.com".parse()?);
//!
//! // Eip1193 trait methods automatically available!
//! let accounts = provider.request_accounts().await?;
//! provider.switch_chain(137).await?;
//! ```
//!
//! ### Pattern 2: Standard Alloy with Transport
//!
//! Use `Eip1193Transport` with standard Alloy providers:
//!
//! ```rust,ignore
//! use alloy::providers::ProviderBuilder;
//! use alloy_eip1193::{Eip1193Transport, ext::Eip1193};
//!
//! let transport = Eip1193Transport::new(ethereum);
//! let provider = ProviderBuilder::new().on_transport(transport);
//!
//! // Eip1193 trait methods automatically available
//! let accounts = provider.request_accounts().await?;
//! provider.switch_chain(137).await?;
//! ```
//!
//! ### Pattern 3: With Signer (Caveat: Shows Warnings)
//!
//! Use `Eip1193Signer` when you need full NetworkWallet compatibility:
//!
//! ```rust,ignore
//! use alloy::providers::ProviderBuilder;
//! use alloy_eip1193::Eip1193Signer;
//!
//! let signer = Eip1193Signer::from_window().await?;
//!
//! // ⚠️ WARNING: This uses eth_sign internally
//! // MetaMask will show scary warnings to users
//! // Only use if you need signing without broadcasting
//! let provider = ProviderBuilder::new()
//!     .wallet(signer)
//!     .on_http(rpc_url);
//! ```

#![cfg_attr(not(target_family = "wasm"), allow(unused))]
#![warn(missing_docs)]

mod transport;
mod signer;
mod chain;
mod error;

pub use transport::Eip1193Transport;
pub use signer::Eip1193Signer;
pub use chain::ChainConfig;
pub use provider::{WalletLayer, WalletProvider};
pub use error::Eip1193Error;

// Re-export provider module for docs
pub mod provider;

// EIP-1193 extension traits and types
pub mod ext;

// Re-export alloy-chains types for convenience
pub use alloy_chains::{Chain, NamedChain};

/// Re-export commonly used types from dependencies
pub mod prelude {
    pub use crate::{
        Eip1193Transport,
        WalletLayer,
        WalletProvider,
        Eip1193Signer,
        ChainConfig,
        Eip1193Error,
    };
    pub use crate::ext::Eip1193;
    pub use alloy::primitives::{Address, Signature, B256};
    pub use alloy::signers::Signer;
    pub use alloy_chains::{Chain, NamedChain};

    /// Helper function to format user-friendly error messages from TransportErrors
    ///
    /// This function attempts to extract EIP-1193 error information from Alloy transport errors
    /// and return a user-friendly message. Falls back to the full error message if the error
    /// is not an EIP-1193 error.
    ///
    /// # Example
    /// ```rust,ignore
    /// use alloy_eip1193::prelude::*;
    ///
    /// match provider.send_transaction(tx).await {
    ///     Ok(result) => { /* success */ },
    ///     Err(err) => {
    ///         let message = format_transport_error(&err);
    ///         println!("Error: {}", message);
    ///     }
    /// }
    /// ```
    pub fn format_transport_error(err: &alloy::transports::TransportError) -> String {
        if let Some(eip1193_err) = Eip1193Error::from_transport_error(err) {
            format!("❌ {}", eip1193_err.user_message())
        } else {
            format!("❌ {}", err)
        }
    }
}
