use alloy::transports::{TransportError, TransportErrorKind};
use alloy_json_rpc::{RequestPacket, ResponsePacket};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::Service;
use wasm_bindgen::prelude::*;
use web_sys::js_sys;

use super::request::{Eip1193Requester, SwitchChainParams, AddChainParams, NativeCurrencyParams};

/// EIP-1193 Transport implementation for Alloy
///
/// This implements tower's Service trait for JSON-RPC requests, wrapping
/// the browser's window.ethereum object to provide a standard Alloy provider interface.
///
/// Note: In WASM, Send/Sync are automatically implemented for all types since
/// there's only one thread. This is safe in the browser environment.
#[derive(Clone)]
pub struct Eip1193Transport {
    requester: Eip1193Requester,
}

// WASM is single-threaded, so Send/Sync are safe
unsafe impl Send for Eip1193Transport {}
unsafe impl Sync for Eip1193Transport {}

impl std::fmt::Debug for Eip1193Transport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Eip1193Transport").finish()
    }
}

impl Eip1193Transport {
    /// Create a new EIP-1193 transport from a wallet's ethereum provider object
    pub fn new(ethereum: JsValue) -> Self {
        Self {
            requester: Eip1193Requester::new(ethereum),
        }
    }

    /// Get the ethereum provider from window.ethereum
    ///
    /// This is a utility method for accessing the global ethereum object.
    /// Use this when you need to access the wallet's provider directly.
    pub fn get_ethereum() -> Result<JsValue, JsValue> {
        let window = web_sys::window()
            .ok_or_else(|| JsValue::from_str("No window object"))?;

        let ethereum = js_sys::Reflect::get(&window, &JsValue::from_str("ethereum"))?;

        if ethereum.is_undefined() || ethereum.is_null() {
            return Err(JsValue::from_str("Ethereum provider not available"));
        }

        Ok(ethereum)
    }

    /// Get a reference to the underlying ethereum provider object
    pub fn ethereum(&self) -> &JsValue {
        self.requester.ethereum()
    }

    /// Request accounts from the wallet (prompts user if needed)
    pub async fn request_accounts(&self) -> Result<Vec<String>, JsValue> {
        let empty_params: Vec<String> = Vec::new();
        self.requester.request("eth_requestAccounts", empty_params).await
    }
}

impl Service<RequestPacket> for Eip1193Transport {
    type Response = ResponsePacket;
    type Error = TransportError;
    // WASM doesn't actually use Send, so we can skip it
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: RequestPacket) -> Self::Future {
        let requester = self.requester.clone();

        Box::pin(async move {
            // Serialize the request to JSON for logging
            let request_json = serde_json::to_string(&req)
                .map_err(|e| TransportErrorKind::custom_str(&format!("{:?}", e)))?;

            log::debug!("EIP-1193 request: {}", request_json);

            // Parse to serde_json::Value for generic handling
            let request_value: serde_json::Value = serde_json::from_str(&request_json)
                .map_err(|e| TransportErrorKind::custom_str(&format!("{:?}", e)))?;

            // Make the request using raw JsValue since RequestPacket is already JSON-RPC formatted
            let result = requester.request_raw(
                request_value.get("method")
                    .and_then(|m| m.as_str())
                    .ok_or_else(|| TransportErrorKind::custom_str("Missing method in request"))?,
                request_value.get("params")
                    .ok_or_else(|| TransportErrorKind::custom_str("Missing params in request"))?
            ).await
                .map_err(|e| TransportErrorKind::custom_str(&format!("{:?}", e)))?;

            // Serialize back to JSON
            let result_json = js_sys::JSON::stringify(&result)
                .map_err(|e| TransportErrorKind::custom_str(&format!("{:?}", e)))?
                .as_string()
                .ok_or_else(|| TransportErrorKind::custom_str("Failed to stringify result"))?;

            log::debug!("EIP-1193 response: {}", result_json);

            // Deserialize to ResponsePacket
            serde_json::from_str(&result_json)
                .map_err(|e| TransportErrorKind::custom_str(&format!("{:?}", e)))
        })
    }
}

/// Chain configuration for adding new networks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainConfig {
    pub chain_id: u64,
    pub chain_name: String,
    pub rpc_urls: Vec<String>,
    pub native_currency: Option<NativeCurrency>,
    pub block_explorer_urls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeCurrency {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

/// Helper methods for wallet-specific EIP-1193 functionality
impl Eip1193Transport {
    /// Switch to a different chain
    pub async fn switch_chain(&self, chain_id: u64) -> Result<(), JsValue> {
        let params = vec![SwitchChainParams {
            chain_id: format!("0x{:x}", chain_id),
        }];

        self.requester
            .request::<_, serde_json::Value>("wallet_switchEthereumChain", params)
            .await?;

        Ok(())
    }

    /// Add a new chain to the wallet
    pub async fn add_chain(&self, config: ChainConfig) -> Result<(), JsValue> {
        let params = vec![AddChainParams {
            chain_id: format!("0x{:x}", config.chain_id),
            chain_name: config.chain_name,
            rpc_urls: config.rpc_urls,
            native_currency: config.native_currency.map(|c| NativeCurrencyParams {
                name: c.name,
                symbol: c.symbol,
                decimals: c.decimals,
            }),
            block_explorer_urls: config.block_explorer_urls,
        }];

        self.requester
            .request::<_, serde_json::Value>("wallet_addEthereumChain", params)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_config_serialization() {
        let config = ChainConfig {
            chain_id: 1,
            chain_name: "Ethereum Mainnet".to_string(),
            rpc_urls: vec!["https://eth.llamarpc.com".to_string()],
            native_currency: Some(NativeCurrency {
                name: "Ether".to_string(),
                symbol: "ETH".to_string(),
                decimals: 18,
            }),
            block_explorer_urls: vec!["https://etherscan.io".to_string()],
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("Ethereum Mainnet"));
    }
}
