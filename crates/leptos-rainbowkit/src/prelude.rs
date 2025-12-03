//! Common imports for Leptos-RainbowKit applications

pub use crate::components::{ConnectButton, RainbowKitProvider, RainbowKitProviderSimple};
pub use crate::components::primitives::{Box, Text, BoxDisplay, BoxFontWeight, BoxTextAlign, QrCode};
pub use crate::theme::{Theme, LightTheme, DarkTheme, MidnightTheme, ThemeOptions};
pub use crate::hooks::{use_wallet, use_balance, use_ens_name};
pub use crate::state::{use_transaction_store, TransactionStatus, WalletProvider};
pub use crate::provider::{Eip1193Transport, ChainConfig};
pub use crate::i18n::{use_i18n, Locale};
pub use leptos::prelude::*;
pub use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
pub use crate::provider::Eip1193Signer;
