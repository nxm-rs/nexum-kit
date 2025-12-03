//! Generic EIP-1193 request handling
//!
//! This module provides a centralized, efficient mechanism for making typed RPC requests
//! to browser wallets via EIP-1193. It ensures:
//! - Type-safe request/response mapping
//! - Zero-copy where possible
//! - Minimal allocations
//! - Consistent error handling

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys;

/// Generic EIP-1193 request handler
///
/// This struct wraps the browser's `window.ethereum` object and provides
/// type-safe, efficient RPC request handling.
#[derive(Clone, Debug)]
pub struct Eip1193Requester {
    ethereum: JsValue,
}

impl Eip1193Requester {
    /// Create a new requester from the ethereum provider object
    #[inline]
    pub fn new(ethereum: JsValue) -> Self {
        Self { ethereum }
    }

    /// Get a reference to the underlying ethereum provider
    #[inline]
    pub fn ethereum(&self) -> &JsValue {
        &self.ethereum
    }

    /// Make a typed RPC request to the wallet
    ///
    /// This is the core request method that all other requests should use.
    /// It handles serialization, the actual request, and deserialization in
    /// the most efficient way possible.
    ///
    /// # Type Parameters
    /// - `P`: The parameter type (must be serializable)
    /// - `R`: The response type (must be deserializable)
    ///
    /// # Arguments
    /// - `method`: The RPC method name (e.g., "eth_requestAccounts")
    /// - `params`: The parameters to pass (will be serialized to JSON)
    ///
    /// # Returns
    /// The typed response from the wallet
    ///
    /// # Performance Notes
    /// - Uses `serde_wasm_bindgen` for zero-copy serialization where possible
    /// - Reuses the same request function reference
    /// - Minimal intermediate allocations
    pub async fn request<P, R>(&self, method: &str, params: P) -> Result<R, JsValue>
    where
        P: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        // Convert params to JsValue using serde_wasm_bindgen for efficiency
        let params_js = serde_wasm_bindgen::to_value(&params)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize params: {:?}", e)))?;

        // Build request object
        let request_obj = js_sys::Object::new();
        js_sys::Reflect::set(&request_obj, &"method".into(), &method.into())?;
        js_sys::Reflect::set(&request_obj, &"params".into(), &params_js)?;

        // Get request function (cached by JS engine)
        let request_fn = js_sys::Reflect::get(&self.ethereum, &"request".into())?;
        let request_fn = request_fn.dyn_into::<js_sys::Function>()?;

        // Make the request
        let promise = request_fn.call1(&self.ethereum, &request_obj)?;
        let promise = promise.dyn_into::<js_sys::Promise>()?;

        // Await the response
        let result = JsFuture::from(promise).await?;

        // Deserialize response using serde_wasm_bindgen for efficiency
        serde_wasm_bindgen::from_value(result)
            .map_err(|e| JsValue::from_str(&format!("Failed to deserialize response: {:?}", e)))
    }

    /// Make a request with no parameters
    ///
    /// Convenience method for methods that don't take parameters.
    /// Uses an empty array for params to satisfy EIP-1193 requirements.
    #[inline]
    pub async fn request_no_params<R>(&self, method: &str) -> Result<R, JsValue>
    where
        R: for<'de> Deserialize<'de>,
    {
        let empty_params: Vec<()> = Vec::new();
        self.request::<_, R>(method, empty_params).await
    }

    /// Make a request returning a raw JsValue
    ///
    /// Use this when you need to handle the response manually or when
    /// the response type is complex/dynamic.
    #[inline]
    pub async fn request_raw<P>(&self, method: &str, params: P) -> Result<JsValue, JsValue>
    where
        P: Serialize,
    {
        let params_js = serde_wasm_bindgen::to_value(&params)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize params: {:?}", e)))?;

        let request_obj = js_sys::Object::new();
        js_sys::Reflect::set(&request_obj, &"method".into(), &method.into())?;
        js_sys::Reflect::set(&request_obj, &"params".into(), &params_js)?;

        let request_fn = js_sys::Reflect::get(&self.ethereum, &"request".into())?;
        let request_fn = request_fn.dyn_into::<js_sys::Function>()?;

        let promise = request_fn.call1(&self.ethereum, &request_obj)?;
        let promise = promise.dyn_into::<js_sys::Promise>()?;

        JsFuture::from(promise).await
    }
}

// WASM is single-threaded, so Send/Sync are safe
unsafe impl Send for Eip1193Requester {}
unsafe impl Sync for Eip1193Requester {}

/// Common RPC request/response types for type-safe API usage

/// Parameters for eth_requestAccounts
pub type RequestAccountsParams = ();

/// Response for eth_requestAccounts
pub type RequestAccountsResponse = Vec<String>;

/// Parameters for eth_chainId
pub type ChainIdParams = ();

/// Response for eth_chainId (returns hex string like "0x1")
pub type ChainIdResponse = String;

/// Parameters for eth_sign (address, message)
pub type EthSignParams = (String, String);

/// Response for eth_sign (signature as hex string)
pub type EthSignResponse = String;

/// Parameters for personal_sign (message, address)
pub type PersonalSignParams = (String, String);

/// Response for personal_sign (signature as hex string)
pub type PersonalSignResponse = String;

/// Parameters for eth_signTypedData_v4 (address, typed_data)
pub type SignTypedDataV4Params = (String, serde_json::Value);

/// Response for eth_signTypedData_v4 (signature as hex string)
pub type SignTypedDataV4Response = String;

/// Parameters for wallet_switchEthereumChain
#[derive(Debug, Clone, Serialize)]
pub struct SwitchChainParams {
    #[serde(rename = "chainId")]
    pub chain_id: String,
}

/// Parameters for wallet_addEthereumChain
#[derive(Debug, Clone, Serialize)]
pub struct AddChainParams {
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "chainName")]
    pub chain_name: String,
    #[serde(rename = "rpcUrls")]
    pub rpc_urls: Vec<String>,
    #[serde(rename = "nativeCurrency", skip_serializing_if = "Option::is_none")]
    pub native_currency: Option<NativeCurrencyParams>,
    #[serde(rename = "blockExplorerUrls")]
    pub block_explorer_urls: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NativeCurrencyParams {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_serialization() {
        let params = EthSignParams {
            address: "0x1234".to_string(),
            message: "0xabcd".to_string(),
        };

        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains("0x1234"));
        assert!(json.contains("0xabcd"));
    }

    #[test]
    fn test_switch_chain_params() {
        let params = SwitchChainParams {
            chain_id: "0x1".to_string(),
        };

        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains("chainId"));
        assert!(json.contains("0x1"));
    }
}
