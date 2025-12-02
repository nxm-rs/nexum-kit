use crate::wallets::wallet::{WalletConnector, WalletMetadata, DownloadUrls, ConnectionMethod};
use crate::wallets::connector::{get_injected_provider, is_metamask, ProviderFlag};
use alloy::primitives::Address;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{Array, Object, Reflect};

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

        // Request accounts using eth_requestAccounts
        let request_args = Object::new();
        Reflect::set(
            &request_args,
            &JsValue::from_str("method"),
            &JsValue::from_str("eth_requestAccounts"),
        )?;
        Reflect::set(
            &request_args,
            &JsValue::from_str("params"),
            &Array::new(),
        )?;

        // Call window.ethereum.request({ method: "eth_requestAccounts", params: [] })
        let request_fn = Reflect::get(&ethereum, &JsValue::from_str("request"))?;
        let promise = Reflect::apply(
            request_fn.unchecked_ref(),
            &ethereum,
            &Array::of1(&request_args),
        )?;

        // Await the promise
        let result = JsFuture::from(js_sys::Promise::from(promise)).await?;

        // Parse accounts array
        let accounts = Array::from(&result);
        let address_str = accounts
            .get(0)
            .as_string()
            .ok_or_else(|| JsValue::from_str("No accounts returned from MetaMask"))?;

        log::info!("MetaMask connected: {}", address_str);

        // Parse address string to Alloy Address
        address_str
            .parse::<Address>()
            .map_err(|e| JsValue::from_str(&format!("Invalid address format: {:?}", e)))
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
