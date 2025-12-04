//! NexumKit - Web3 wallet connection library for Leptos
//!
//! This library provides a set of components for connecting Ethereum wallets
//! in Leptos applications with a focus on CSR (client-side rendering).
//!
//! # Platform Support
//!
//! **This crate only supports wasm32 targets** as it interacts with browser-based
//! Ethereum wallet extensions via EIP-1193. Attempting to compile for non-WASM
//! targets will result in compilation errors.
//!
//! To use this crate, compile with:
//! ```bash
//! cargo build --target wasm32-unknown-unknown
//! ```

#![cfg(target_arch = "wasm32")]

// Module declarations
pub mod components;
pub mod provider;
pub mod state;
pub mod theme;
pub mod wallets;
pub mod hooks;
pub mod utils;
pub mod i18n;
pub mod prelude;

// Re-exports
pub use components::{
    ConnectButton,
    NexumKitProvider,
    ConnectModal,
    AccountModal,
};

pub use theme::{
    Theme, LightTheme, DarkTheme, MidnightTheme,
    ThemeProvider, ThemeOptions, AccentColorPreset,
    BorderRadius, FontStack, OverlayBlur
};

pub use hooks::{
    use_wallet,
    use_balance,
    use_ens_name,
};

pub use state::{
    ModalState,
    ConnectionState,
    ConnectionStatus,
    TransactionStore,
    TransactionStatus,
    Transaction,
};

pub use i18n::{
    Locale,
    use_i18n,
};

pub use provider::{
    Eip1193Transport,
    ChainConfig,
};

#[cfg(target_arch = "wasm32")]
pub use provider::Eip1193Signer;
