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
use crate::error::Eip1193Error;

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
    pub fn get_ethereum() -> Result<JsValue, Eip1193Error> {
        let window = web_sys::window()
            .ok_or_else(|| Eip1193Error::JsError("No window object".to_string()))?;

        let ethereum = js_sys::Reflect::get(&window, &JsValue::from_str("ethereum"))
            .map_err(|e| Eip1193Error::from_js_value(e))?;

        if ethereum.is_undefined() || ethereum.is_null() {
            return Err(Eip1193Error::JsError("Ethereum provider not available".to_string()));
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
    pub fn client_from_window() -> Result<alloy::rpc::client::RpcClient, Eip1193Error> {
        let ethereum = Self::get_ethereum()?;
        let transport = Self::new(ethereum);
        Ok(transport.into_client())
    }


    /// Make a raw EIP-1193 request and return a JSON-RPC formatted response
    ///
    /// This method is the core request handler that:
    /// 1. Calls the EIP-1193 provider's `request` method
    /// 2. Wraps the response in a proper JSON-RPC response structure
    ///
    /// This is used by the Service::call implementation to handle RequestPacket -> ResponsePacket
    /// conversion while maintaining EIP-1193 compatibility.
    ///
    /// # Arguments
    ///
    /// * `method` - The RPC method name (e.g., "eth_requestAccounts")
    /// * `params` - The method parameters (will be serialized to JSON)
    /// * `id` - The JSON-RPC request ID (from the original RequestPacket)
    ///
    /// # Returns
    ///
    /// A JSON-RPC 2.0 formatted response as a serde_json::Value with the structure:
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "id": <request_id>,
    ///   "result": <eip1193_response>
    /// }
    /// ```
    async fn request_raw<P>(&self, method: &str, params: P, id: u64) -> Result<serde_json::Value, Eip1193Error>
    where
        P: Serialize,
    {
        // Serialize params to JSON string first
        let params_json_str = serde_json::to_string(&params)?;

        log::debug!("EIP-1193 request params for {} (JSON string): {}", method, params_json_str);

        // Parse JSON string to JsValue using JavaScript's JSON.parse
        // This ensures correct object structure instead of using serde_wasm_bindgen
        let params_js = js_sys::JSON::parse(&params_json_str)
            .map_err(|_| Eip1193Error::SerializationError("Failed to parse params JSON".into()))?;

        // Build EIP-1193 request object: { method, params }
        let request_obj = js_sys::Object::new();
        js_sys::Reflect::set(&request_obj, &"method".into(), &method.into())
            .map_err(|e| Eip1193Error::from_js_value(e))?;
        js_sys::Reflect::set(&request_obj, &"params".into(), &params_js)
            .map_err(|e| Eip1193Error::from_js_value(e))?;

        // Get the request function from the provider
        let request_fn = js_sys::Reflect::get(&self.ethereum, &"request".into())
            .map_err(|e| Eip1193Error::from_js_value(e))?;
        let request_fn = request_fn.dyn_into::<js_sys::Function>()
            .map_err(|e| Eip1193Error::from_js_value(e))?;

        // Make the EIP-1193 request
        let promise = request_fn.call1(&self.ethereum, &request_obj)
            .map_err(|e| Eip1193Error::from_js_value(e))?;
        let promise = promise.dyn_into::<js_sys::Promise>()
            .map_err(|e| Eip1193Error::from_js_value(e))?;

        // Await the EIP-1193 response
        // This is where provider errors (user rejection, etc.) are caught
        let result = JsFuture::from(promise).await
            .map_err(|e| Eip1193Error::from_js_value(e))?;

        // Convert the JsValue result to JSON string
        let result_json = js_sys::JSON::stringify(&result)
            .map_err(|_| Eip1193Error::SerializationError("Failed to stringify result".into()))?
            .as_string()
            .ok_or_else(|| Eip1193Error::SerializationError("Failed to convert result to string".into()))?;

        log::debug!("EIP-1193 response for {}: {}", method, result_json);

        // Parse the JSON string to a serde_json::Value
        let result_value: serde_json::Value = serde_json::from_str(&result_json)?;

        // Build a proper JSON-RPC 2.0 response packet
        // This is what Alloy's RPC client expects
        Ok(serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": result_value
        }))
    }

    /// Make a typed RPC request to the wallet (convenience method)
    ///
    /// This is a convenience wrapper around request_raw for cases where you need
    /// to make a direct RPC call outside of the Service/RpcClient flow.
    ///
    /// Uses ID 1 for the JSON-RPC response structure. Since EIP-1193 uses Promises
    /// for request/response matching, the ID is only used for Alloy's internal
    /// bookkeeping and doesn't affect correctness.
    ///
    /// **Note:** This is rarely needed - prefer using the Provider's trait methods
    /// which will go through the RpcClient and Service::call flow automatically.
    pub async fn request<P, R>(&self, method: &str, params: P) -> Result<R, Eip1193Error>
    where
        P: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        // Use constant ID since EIP-1193 uses Promises for matching, not IDs
        let result = self.request_raw(method, params, 1).await?;

        // Deserialize response
        let result_field = result.get("result")
            .ok_or_else(|| Eip1193Error::SerializationError("Missing result field in response".into()))?;

        serde_json::from_value(result_field.clone())
            .map_err(|e| Eip1193Error::SerializationError(format!("Failed to deserialize response: {}", e)))
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
            // Serialize the request to JSON for logging and parsing
            let request_json = serde_json::to_string(&req)
                .map_err(|e| TransportErrorKind::custom_str(&format!("{:?}", e)))?;

            log::debug!("EIP-1193 request: {}", request_json);

            // Parse to serde_json::Value for generic handling
            let request_value: serde_json::Value = serde_json::from_str(&request_json)
                .map_err(|e| TransportErrorKind::custom_str(&format!("{:?}", e)))?;

            // Extract method, params, and id from the request
            let method = request_value.get("method")
                .and_then(|m| m.as_str())
                .ok_or_else(|| TransportErrorKind::custom_str("Missing method in request"))?;

            // Params might be missing for methods like eth_requestAccounts
            let params = request_value.get("params")
                .cloned()
                .unwrap_or(serde_json::Value::Array(vec![]));

            // Get the request ID from the original request
            let id = request_value.get("id")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);

            // Make the request using request_raw which handles JSON-RPC response construction
            // Convert Eip1193Error to TransportError
            let response = transport.request_raw(method, params, id).await
                .map_err(|e| e.into_transport_error())?;

            log::debug!("EIP-1193 response: {}", serde_json::to_string(&response).unwrap_or_default());

            // Deserialize the JSON-RPC response to ResponsePacket
            serde_json::from_value(response)
                .map_err(|e| TransportErrorKind::custom_str(&format!("{:?}", e)))
        };

        // Use BoxFuture which automatically handles Send for WASM vs non-WASM
        Box::pin(fut)
    }
}
