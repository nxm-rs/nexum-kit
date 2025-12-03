//! Generic EIP-1193 request handling
//!
//! This module provides a centralized, efficient mechanism for making typed RPC requests
//! to browser wallets via EIP-1193. It ensures:
//! - Type-safe request/response mapping
//! - Zero-copy where possible
//! - Minimal allocations
//! - Consistent error handling
//!
//! ## Design Philosophy
//!
//! We follow Alloy's idiomatic patterns for RPC calls:
//! - Use tuples for RPC parameters (e.g., `(address, data)`) instead of custom structs
//! - No type aliases for standard Ethereum RPC methods (those are in Alloy's Provider trait)
//! - Only define types for wallet-specific extensions (e.g., `wallet_switchEthereumChain`)
//!
//! This keeps our crate focused on what's unique to browser wallets while leveraging
//! Alloy's existing infrastructure for standard RPC methods.

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

// NOTE: We don't define type aliases for standard Ethereum RPC methods that are already
// covered by Alloy's Provider trait (like eth_chainId, eth_getBalance, etc.).
//
// For wallet-specific RPC methods that require objects with named fields in JSON,
// we provide internal helper structs. These are not exported in the public API since
// users will typically interact with higher-level methods that construct these internally.
