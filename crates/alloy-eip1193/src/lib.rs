//! # alloy-eip1193
//!
//! EIP-1193 provider and signer implementation for Alloy in WebAssembly environments.
//!
//! This crate provides:
//! - **`Eip1193Transport`**: A tower Service implementation for JSON-RPC requests via browser wallets
//! - **`Eip1193Signer`**: A signer implementation that delegates signing to browser wallets
//! - **`Eip1193Requester`**: Generic typed request handler for EIP-1193 providers
//!
//! ## Usage
//!
//! ### As a Transport (for read operations)
//!
//! ```rust,ignore
//! use alloy_eip1193::{Eip1193Transport};
//! use alloy::providers::ProviderBuilder;
//!
//! let ethereum = Eip1193Transport::get_ethereum()?;
//! let transport = Eip1193Transport::new(ethereum);
//! let provider = ProviderBuilder::new().on_transport(transport);
//! ```
//!
//! ### As a Signer (for write operations)
//!
//! ```rust,ignore
//! use alloy_eip1193::Eip1193Signer;
//! use alloy::providers::ProviderBuilder;
//!
//! let signer = Eip1193Signer::from_window().await?;
//! let provider = ProviderBuilder::new()
//!     .with_signer(signer)
//!     .on_http("https://eth.llamarpc.com".parse()?);
//! ```
//!
//! ### Combined (Transport + Signer)
//!
//! ```rust,ignore
//! use alloy_eip1193::{Eip1193Transport, Eip1193Signer};
//! use alloy::providers::ProviderBuilder;
//!
//! let ethereum = Eip1193Transport::get_ethereum()?;
//! let transport = Eip1193Transport::new(ethereum.clone());
//! let accounts = transport.request_accounts().await?;
//! let address = accounts[0].parse()?;
//! let signer = Eip1193Signer::new(ethereum, address);
//!
//! let provider = ProviderBuilder::new()
//!     .with_signer(signer)
//!     .on_transport(transport);
//! ```

#![cfg_attr(not(target_family = "wasm"), allow(unused))]
#![warn(missing_docs)]

mod request;
mod transport;
mod signer;
mod chain;
mod wallet;

pub use request::Eip1193Requester;
pub use transport::Eip1193Transport;
pub use signer::Eip1193Signer;
pub use chain::ChainConfig;
pub use wallet::WalletOperations;

// Re-export alloy-chains types for convenience
pub use alloy_chains::{Chain, NamedChain};

/// Re-export commonly used types from dependencies
pub mod prelude {
    pub use crate::{Eip1193Transport, Eip1193Signer, Eip1193Requester, ChainConfig, WalletOperations};
    pub use alloy::primitives::{Address, Signature, B256};
    pub use alloy::signers::Signer;
    pub use alloy_chains::{Chain, NamedChain};
}
