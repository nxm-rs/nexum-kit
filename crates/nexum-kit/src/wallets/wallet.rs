use alloy::primitives::Address;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

/// Download URLs for different platforms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadUrls {
    pub android: Option<String>,
    pub ios: Option<String>,
    pub mobile: Option<String>,
    pub chrome: Option<String>,
    pub firefox: Option<String>,
    pub edge: Option<String>,
    pub safari: Option<String>,
    pub browser_extension: Option<String>,
    pub desktop: Option<String>,
}

impl Default for DownloadUrls {
    fn default() -> Self {
        Self {
            android: None,
            ios: None,
            mobile: None,
            chrome: None,
            firefox: None,
            edge: None,
            safari: None,
            browser_extension: None,
            desktop: None,
        }
    }
}

/// Wallet metadata - describes a wallet's identity and download links
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletMetadata {
    /// Unique identifier (e.g., "metamask", "trust")
    pub id: String,
    /// Display name (e.g., "MetaMask", "Trust Wallet")
    pub name: String,
    /// EIP-6963 Reverse DNS namespace (e.g., "io.metamask")
    pub rdns: Option<String>,
    /// Icon URL or data URL
    pub icon_url: String,
    /// Icon background color (hex)
    pub icon_background: String,
    /// Accent color (hex)
    pub icon_accent: Option<String>,
    /// Download URLs for various platforms
    pub download_urls: Option<DownloadUrls>,
}

/// Connection method for a wallet
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionMethod {
    /// Direct injection (browser extension)
    Injected,
    /// WalletConnect via QR code
    WalletConnect,
    /// Mobile deep linking
    MobileDeepLink,
}

/// Trait for wallet connectors
///
/// Each wallet implements this trait to provide connection functionality.
/// This follows the pattern from the original NexumKit but adapted for Rust/Leptos.
pub trait WalletConnector {
    /// Get wallet metadata
    fn metadata(&self) -> &WalletMetadata;

    /// Connect to the wallet
    ///
    /// Returns the connected address on success
    fn connect(&self) -> impl std::future::Future<Output = Result<Address, JsValue>> + '_;

    /// Disconnect from the wallet
    fn disconnect(&self) -> impl std::future::Future<Output = Result<(), JsValue>> + '_;

    /// Check if the wallet is installed/available
    fn is_installed(&self) -> bool;

    /// Get the raw provider object (window.ethereum or similar)
    fn get_provider(&self) -> Option<JsValue>;

    /// Get the preferred connection method for this wallet
    fn preferred_method(&self) -> ConnectionMethod {
        if self.is_installed() {
            ConnectionMethod::Injected
        } else {
            ConnectionMethod::WalletConnect
        }
    }

    /// Get mobile deep link URI if supported
    fn mobile_uri(&self, _wc_uri: &str) -> Option<String> {
        None
    }

    /// Get QR code URI for WalletConnect if supported
    fn qr_code_uri(&self, wc_uri: &str) -> Option<String> {
        Some(wc_uri.to_string())
    }
}
