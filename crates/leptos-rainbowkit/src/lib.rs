//! Leptos-RainbowKit - Web3 wallet connection library for Leptos
//!
//! This library provides a set of components for connecting Ethereum wallets
//! in Leptos applications with a focus on CSR (client-side rendering).

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
    RainbowKitProvider,
};

pub use theme::{Theme, LightTheme, DarkTheme};
