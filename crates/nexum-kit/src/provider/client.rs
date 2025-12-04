use alloy::providers::ProviderBuilder;
use wasm_bindgen::JsValue;

/// Create an HTTP provider for the given RPC URL
///
/// This function creates an Alloy provider that can be used for read operations
/// against an Ethereum RPC endpoint. Reqwest automatically uses WASM-compatible
/// HTTP when compiled for wasm32 targets.
///
/// For signing operations, we'll use the browser wallet's native signing
/// capabilities via window.ethereum.
pub async fn create_http_provider(rpc_url: &str) -> Result<impl alloy::providers::Provider + Clone, JsValue> {
    ProviderBuilder::new()
        .connect(rpc_url)
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to connect to provider: {:?}", e)))
}
