pub mod client;

// Re-export EIP-1193 functionality from alloy-eip1193 crate
pub use alloy_eip1193::{
    Eip1193Transport,
    Eip1193Signer,
    Eip1193Requester,
    ChainConfig,
    WalletOperations,
};

pub use client::create_http_provider;
