use crate::wallets::wallet::{WalletConnector, WalletMetadata, DownloadUrls, ConnectionMethod};
use crate::wallets::connector::{get_injected_provider, is_metamask, ProviderFlag};
use crate::provider::Eip1193;
use alloy::primitives::Address;
use alloy::providers::RootProvider;
use alloy::network::Ethereum;
use alloy_eip1193::Eip1193Transport;
use wasm_bindgen::prelude::*;

/// MetaMask wallet connector
///
/// Implements connection to MetaMask via browser extension (window.ethereum)
pub struct MetaMaskConnector {
    metadata: WalletMetadata,
}

impl MetaMaskConnector {
    pub fn new() -> Self {
        Self {
            metadata: WalletMetadata {
                id: "metamask".to_string(),
                name: "MetaMask".to_string(),
                rdns: Some("io.metamask".to_string()),
                // MetaMask fox icon (simplified SVG as data URL)
                icon_url: "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMzIiIGhlaWdodD0iMzIiIHZpZXdCb3g9IjAgMCAzMiAzMiIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHJlY3Qgd2lkdGg9IjMyIiBoZWlnaHQ9IjMyIiBmaWxsPSIjRjY4NTFBIi8+Cjwvc3ZnPgo=".to_string(),
                icon_background: "#fff".to_string(),
                icon_accent: Some("#f6851a".to_string()),
                download_urls: Some(DownloadUrls {
                    chrome: Some("https://chrome.google.com/webstore/detail/metamask/nkbihfbeogaeaoehlefnkodbefgpgknn".to_string()),
                    firefox: Some("https://addons.mozilla.org/en-US/firefox/addon/ether-metamask/".to_string()),
                    browser_extension: Some("https://metamask.io/download/".to_string()),
                    ..Default::default()
                }),
            },
        }
    }

    /// Get the ethereum provider object
    fn get_ethereum(&self) -> Option<JsValue> {
        get_injected_provider(Some(ProviderFlag::IsMetaMask), None)
    }
}

impl Default for MetaMaskConnector {
    fn default() -> Self {
        Self::new()
    }
}

impl WalletConnector for MetaMaskConnector {
    fn metadata(&self) -> &WalletMetadata {
        &self.metadata
    }

    async fn connect(&self) -> Result<Address, JsValue> {
        let ethereum = self
            .get_ethereum()
            .ok_or_else(|| JsValue::from_str("MetaMask not installed"))?;

        // Use modern RpcClient + Provider pattern with Eip1193 trait
        let transport = Eip1193Transport::new(ethereum);
        let client = transport.into_client();
        let provider = RootProvider::<Ethereum>::new(client);

        let accounts = provider.request_accounts().await
            .map_err(|e| JsValue::from_str(&format!("Failed to request accounts: {:?}", e)))?;

        let address = accounts
            .first()
            .copied()
            .ok_or_else(|| JsValue::from_str("No accounts returned from MetaMask"))?;

        log::info!("MetaMask connected: {:?}", address);

        Ok(address)
    }

    async fn disconnect(&self) -> Result<(), JsValue> {
        // MetaMask doesn't have a programmatic disconnect method
        // The user must disconnect through the MetaMask UI
        log::info!("MetaMask disconnect requested (manual disconnect required)");
        Ok(())
    }

    fn is_installed(&self) -> bool {
        if let Some(ethereum) = self.get_ethereum() {
            is_metamask(&ethereum)
        } else {
            false
        }
    }

    fn get_provider(&self) -> Option<JsValue> {
        self.get_ethereum()
    }

    fn preferred_method(&self) -> ConnectionMethod {
        if self.is_installed() {
            ConnectionMethod::Injected
        } else {
            // Fall back to WalletConnect QR code if not installed
            ConnectionMethod::WalletConnect
        }
    }

    fn mobile_uri(&self, wc_uri: &str) -> Option<String> {
        // MetaMask mobile deep link
        Some(format!("https://metamask.app.link/wc?uri={}", urlencoding::encode(wc_uri)))
    }

    fn qr_code_uri(&self, wc_uri: &str) -> Option<String> {
        // For QR code, return the WalletConnect URI wrapped in MetaMask deep link
        Some(format!("https://metamask.app.link/wc?uri={}", urlencoding::encode(wc_uri)))
    }
}
