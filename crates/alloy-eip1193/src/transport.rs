//! EIP-1193 Transport Layer
//!
//! This module provides a Tower Service implementation for JSON-RPC requests via EIP-1193,
//! making browser wallet providers compatible with Alloy's provider architecture.
//!
//! ## Design Philosophy
//!
//! We follow Alloy's idiomatic patterns for RPC calls:
//! - Use tuples for RPC parameters (e.g., `(address, data)`) instead of custom structs
//! - No type aliases for standard Ethereum RPC methods (those are in Alloy's Provider trait)
//! - Only define types for wallet-specific extensions (handled in wallet.rs)

use alloy::transports::{TransportError, TransportErrorKind, TransportFut};
use alloy_json_rpc::{RequestPacket, ResponsePacket};
use serde::{Deserialize, Serialize};
use std::task::{Context, Poll};
use tower::Service;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys;

/// EIP-1193 Transport implementation for Alloy
///
/// This implements tower's Service trait for JSON-RPC requests, wrapping
/// the browser's window.ethereum object to provide a standard Alloy provider interface.
///
/// Note: In WASM, Send/Sync are automatically implemented for all types since
/// there's only one thread. This is safe in the browser environment.
#[derive(Clone)]
pub struct Eip1193Transport {
    ethereum: JsValue,
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
        Self { ethereum }
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
        &self.ethereum
    }

    /// Create an `RpcClient` from this transport
    ///
    /// This is the modern Alloy pattern for creating providers. The `RpcClient` can then be used
    /// to create a `RootProvider` which gives access to all provider methods including the
    /// `ext::Eip1193` trait.
    ///
    /// # Example
    /// ```rust,ignore
    /// use alloy_eip1193::{Eip1193Transport, ext::Eip1193};
    /// use alloy::providers::RootProvider;
    ///
    /// let transport = Eip1193Transport::get_ethereum()?;
    /// let client = transport.into_client();
    /// let provider = RootProvider::new(client);
    ///
    /// // Use Eip1193 trait methods
    /// let accounts = provider.request_accounts().await?;
    /// provider.switch_chain(137).await?;
    /// ```
    pub fn into_client(self) -> alloy::rpc::client::RpcClient {
        // Browser wallets are always "local" in the sense that they're in the same context
        alloy::rpc::client::RpcClient::new(self, true)
    }

    /// Create an `RpcClient` from window.ethereum
    ///
    /// This is a convenience method that combines `get_ethereum()` and `into_client()`.
    ///
    /// # Example
    /// ```rust,ignore
    /// use alloy_eip1193::Eip1193Transport;
    /// use alloy::providers::RootProvider;
    ///
    /// let client = Eip1193Transport::client_from_window()?;
    /// let provider = RootProvider::new(client);
    /// ```
    pub fn client_from_window() -> Result<alloy::rpc::client::RpcClient, JsValue> {
        let ethereum = Self::get_ethereum()?;
        let transport = Self::new(ethereum);
        Ok(transport.into_client())
    }


    /// Make a typed RPC request to the wallet
    ///
    /// This is the core request method for EIP-1193 calls.
    /// It handles serialization, the actual request, and deserialization.
    pub(crate) async fn request<P, R>(&self, method: &str, params: P) -> Result<R, JsValue>
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

    /// Make a request returning a raw JsValue
    ///
    /// Use this when you need to handle the response manually or when
    /// the response type is complex/dynamic.
    pub(crate) async fn request_raw<P>(&self, method: &str, params: P) -> Result<JsValue, JsValue>
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

impl Service<RequestPacket> for Eip1193Transport {
    type Response = ResponsePacket;
    type Error = TransportError;
    // Use Alloy's TransportFut which handles WASM/non-WASM Send differences
    type Future = TransportFut<'static>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: RequestPacket) -> Self::Future {
        let transport = self.clone();

        let fut = async move {
            // Serialize the request to JSON for logging
            let request_json = serde_json::to_string(&req)
                .map_err(|e| TransportErrorKind::custom_str(&format!("{:?}", e)))?;

            log::debug!("EIP-1193 request: {}", request_json);

            // Parse to serde_json::Value for generic handling
            let request_value: serde_json::Value = serde_json::from_str(&request_json)
                .map_err(|e| TransportErrorKind::custom_str(&format!("{:?}", e)))?;

            // Make the request using raw JsValue since RequestPacket is already JSON-RPC formatted
            let method = request_value.get("method")
                .and_then(|m| m.as_str())
                .ok_or_else(|| TransportErrorKind::custom_str("Missing method in request"))?;

            // Params might be missing for methods like eth_requestAccounts
            let params = request_value.get("params")
                .cloned()
                .unwrap_or(serde_json::Value::Array(vec![]));

            let result = transport.request_raw(method, params).await
                .map_err(|e| TransportErrorKind::custom_str(&format!("{:?}", e)))?;

            // Serialize back to JSON
            let result_json = js_sys::JSON::stringify(&result)
                .map_err(|e| TransportErrorKind::custom_str(&format!("{:?}", e)))?
                .as_string()
                .ok_or_else(|| TransportErrorKind::custom_str("Failed to stringify result"))?;

            log::debug!("EIP-1193 response: {}", result_json);

            // Get the request ID from the original request
            let id = request_value.get("id")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);

            // Build a proper JSON-RPC response packet
            let response = serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": serde_json::from_str::<serde_json::Value>(&result_json)
                    .map_err(|e| TransportErrorKind::custom_str(&format!("{:?}", e)))?
            });

            // Deserialize to ResponsePacket
            serde_json::from_value(response)
                .map_err(|e| TransportErrorKind::custom_str(&format!("{:?}", e)))
        };

        // Use BoxFuture which automatically handles Send for WASM vs non-WASM
        Box::pin(fut)
    }
}
