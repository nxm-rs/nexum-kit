//! EIP-1193 Error Types
//!
//! This module provides comprehensive error handling for EIP-1193 provider requests,
//! implementing wallet-specific error codes defined in the EIP-1193 specification.
//!
//! # Relationship with Alloy's Error System
//!
//! Alloy's `alloy-json-rpc` crate provides `ErrorPayload` which handles standard JSON-RPC 2.0
//! errors (-32700 to -32600) and server errors (-32000 to -32099). This module extends that with:
//! - **EIP-1193 provider errors** (4000-4999): Wallet-specific errors unique to browser wallets
//! - **Helper methods**: Classification and user-friendly message formatting
//! - **Integration utilities**: Convert between Alloy's `ErrorPayload` and `Eip1193Error`
//!
//! # EIP-1193 Error Codes (4000-4999)
//!
//! - **4001**: User Rejected Request - User explicitly denied the request
//! - **4100**: Unauthorized - Request not authorized by the user
//! - **4200**: Unsupported Method - Provider doesn't support the requested method
//! - **4900**: Disconnected - Provider is disconnected from all chains
//! - **4901**: Chain Disconnected - Provider not connected to requested chain
//! - **4902**: Unrecognized Chain - Requested chain hasn't been added to provider

use thiserror::Error;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use alloy::transports::{TransportError, TransportErrorKind};
use alloy_json_rpc::ErrorPayload;

/// EIP-1193 Provider Error
///
/// Error type for EIP-1193 provider operations covering wallet-specific error codes.
/// This focuses on EIP-1193 provider errors (4000-4999) which are unique to browser wallets.
/// Standard JSON-RPC errors are handled by Alloy's ErrorPayload system.
#[derive(Debug, Error)]
pub enum Eip1193Error {
    // ============================================================================
    // EIP-1193 Provider Errors (4000-4999)
    // ============================================================================

    /// User rejected the request (EIP-1193 error code 4001)
    ///
    /// This error occurs when the user explicitly denies a request in their wallet UI.
    /// Most common scenarios:
    /// - User clicks "Reject" on account access request (eth_requestAccounts)
    /// - User clicks "Cancel" on transaction approval (eth_sendTransaction)
    /// - User declines signature request (personal_sign, eth_signTypedData_v4)
    /// - User refuses chain switch (wallet_switchEthereumChain)
    #[error("User rejected the request")]
    UserRejectedRequest,

    /// Request is not authorized by the user (EIP-1193 error code 4100)
    ///
    /// The requested method and/or account has not been authorized by the user.
    /// This typically means the dApp needs to call eth_requestAccounts first.
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// The requested method is not supported (EIP-1193 error code 4200)
    ///
    /// The provider does not support the requested method.
    /// Common for newer RPC methods not implemented by all wallets.
    #[error("Unsupported method: {0}")]
    UnsupportedMethod(String),

    /// The provider is disconnected (EIP-1193 error code 4900)
    ///
    /// The provider is disconnected from all chains and cannot process requests.
    #[error("Provider disconnected from all chains")]
    Disconnected,

    /// The provider is not connected to the requested chain (EIP-1193 error code 4901)
    ///
    /// The provider is connected to a blockchain, but not to the chain
    /// required to fulfill the request.
    #[error("Provider not connected to requested chain: {0}")]
    ChainDisconnected(u64),

    /// Chain has not been added to the provider (EIP-1193 error code 4902)
    ///
    /// Thrown when attempting to switch to a chain that hasn't been added via
    /// wallet_addEthereumChain. The dApp should prompt to add the chain first.
    #[error("Chain {0} has not been added to the provider")]
    UnrecognizedChain(u64),

    // ============================================================================
    // Generic Errors (for Alloy/RPC errors)
    // ============================================================================

    /// JavaScript error from the provider
    ///
    /// Wraps a JavaScript error from the EIP-1193 provider.
    #[error("JavaScript error: {0}")]
    JsError(String),

    /// Unknown error with custom code
    ///
    /// An error was returned with a code that doesn't match known standards.
    #[error("Error {code}: {message}")]
    UnknownError {
        /// The error code
        code: i32,
        /// The error message
        message: String
    },

    /// Serialization error
    ///
    /// Failed to serialize/deserialize data.
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl Eip1193Error {
    /// Create an Eip1193Error from Alloy's ErrorPayload
    ///
    /// This allows converting JSON-RPC error responses from the server into typed EIP-1193 errors.
    ///
    /// # Example
    /// ```rust,ignore
    /// use alloy_json_rpc::ErrorPayload;
    /// use alloy_eip1193::Eip1193Error;
    ///
    /// let payload = ErrorPayload { code: 4001, message: "User rejected".into(), data: None };
    /// let error = Eip1193Error::from_error_payload(&payload);
    /// assert!(error.is_user_rejection());
    /// ```
    pub fn from_error_payload<T>(payload: &ErrorPayload<T>) -> Self {
        Self::from_code(payload.code as i32, payload.message.to_string())
    }

    /// Parse an EIP-1193 error from a JsValue
    ///
    /// Attempts to extract the error code and message from a JavaScript error object.
    /// Follows the EIP-1193 error object structure: `{ code: number, message: string }`
    pub fn from_js_value(value: JsValue) -> Self {
        // Try to extract error code and message from the JS object
        if let Some(obj) = value.as_ref().dyn_ref::<web_sys::js_sys::Object>() {
            let code = web_sys::js_sys::Reflect::get(obj, &"code".into())
                .ok()
                .and_then(|v| v.as_f64())
                .map(|f| f as i32);

            let message = web_sys::js_sys::Reflect::get(obj, &"message".into())
                .ok()
                .and_then(|v| v.as_string())
                .unwrap_or_else(|| "Unknown error".to_string());

            if let Some(code) = code {
                return Self::from_code(code, message);
            }
        }

        // Fallback: try to convert to string
        if let Some(s) = value.as_string() {
            Self::JsError(s)
        } else {
            Self::JsError(format!("{:?}", value))
        }
    }

    /// Create an error from an error code and message
    ///
    /// Maps error codes to the appropriate variant according to EIP-1193.
    /// Non-EIP-1193 error codes are wrapped in UnknownError.
    pub fn from_code(code: i32, message: String) -> Self {
        match code {
            // EIP-1193 Provider Errors (4000-4999)
            4001 => Self::UserRejectedRequest,
            4100 => Self::Unauthorized(message),
            4200 => Self::UnsupportedMethod(message),
            4900 => Self::Disconnected,
            4901 => {
                // Try to parse chain ID from message
                message.split_whitespace()
                    .last()
                    .and_then(|s| s.parse().ok())
                    .map(Self::ChainDisconnected)
                    .unwrap_or(Self::ChainDisconnected(0))
            }
            4902 => {
                // Try to parse chain ID from message
                message.split_whitespace()
                    .find(|s| s.starts_with("0x") || s.chars().all(|c| c.is_ascii_digit()))
                    .and_then(|s| {
                        if s.starts_with("0x") {
                            u64::from_str_radix(s.trim_start_matches("0x"), 16).ok()
                        } else {
                            s.parse().ok()
                        }
                    })
                    .map(Self::UnrecognizedChain)
                    .unwrap_or(Self::UnrecognizedChain(0))
            }

            // All other error codes (JSON-RPC, server errors, etc.) are wrapped as unknown
            // These are handled by Alloy's ErrorPayload system
            _ => Self::UnknownError { code, message },
        }
    }

    /// Get the error code for this error
    ///
    /// Returns the EIP-1193 error code that represents this error.
    /// For generic errors (JsError, SerializationError), returns 0 as there's no specific code.
    pub fn code(&self) -> i32 {
        match self {
            Self::UserRejectedRequest => 4001,
            Self::Unauthorized(_) => 4100,
            Self::UnsupportedMethod(_) => 4200,
            Self::Disconnected => 4900,
            Self::ChainDisconnected(_) => 4901,
            Self::UnrecognizedChain(_) => 4902,
            Self::UnknownError { code, .. } => *code,
            Self::JsError(_) | Self::SerializationError(_) => 0,
        }
    }

    /// Check if this error represents a user rejection
    pub fn is_user_rejection(&self) -> bool {
        matches!(self, Self::UserRejectedRequest)
    }

    /// Check if this error is related to authorization
    pub fn is_authorization_error(&self) -> bool {
        matches!(self, Self::Unauthorized(_) | Self::UserRejectedRequest)
    }

    /// Check if this error is related to chain connectivity
    pub fn is_chain_error(&self) -> bool {
        matches!(
            self,
            Self::Disconnected | Self::ChainDisconnected(_) | Self::UnrecognizedChain(_)
        )
    }

    /// Convert to an Alloy TransportError
    ///
    /// Integrates EIP-1193 errors with Alloy's error handling system.
    /// The error is converted to a string-based custom transport error.
    pub fn into_transport_error(self) -> TransportError {
        // Use custom with the error boxed to preserve type information
        TransportErrorKind::custom(self)
    }

    /// Try to extract an Eip1193Error from a TransportError by parsing the error message
    ///
    /// Since Alloy's TransportError doesn't preserve the original error type after conversion,
    /// we parse the error message to detect EIP-1193 error codes and reconstruct the error.
    ///
    /// # Example
    /// ```rust,ignore
    /// match provider.send_transaction(tx).await {
    ///     Err(transport_err) => {
    ///         if let Some(eip1193_err) = Eip1193Error::from_transport_error(&transport_err) {
    ///             if eip1193_err.is_user_rejection() {
    ///                 println!("User cancelled");
    ///             }
    ///         }
    ///     }
    ///     Ok(result) => { /* ... */ }
    /// }
    /// ```
    pub fn from_transport_error(err: &TransportError) -> Option<Self> {
        let err_str = err.to_string();

        // Try to extract EIP-1193 error codes from the message
        // Common patterns: "Error: User rejected" or "code 4001" or "error 4001"

        if err_str.contains("User rejected") || err_str.contains("4001") {
            return Some(Self::UserRejectedRequest);
        }

        if err_str.contains("4100") || err_str.contains("Unauthorized") {
            return Some(Self::Unauthorized(err_str));
        }

        if err_str.contains("4200") || err_str.contains("Unsupported method") {
            return Some(Self::UnsupportedMethod(err_str));
        }

        if err_str.contains("4900") || err_str.contains("Disconnected") {
            return Some(Self::Disconnected);
        }

        if err_str.contains("4901") || err_str.contains("not connected to requested chain") {
            return Some(Self::ChainDisconnected(0));
        }

        if err_str.contains("4902") || err_str.contains("Unrecognized chain") {
            return Some(Self::UnrecognizedChain(0));
        }

        None
    }

    /// Format a user-friendly error message for this error
    ///
    /// Returns a concise, user-facing error message suitable for displaying in UI.
    pub fn user_message(&self) -> String {
        match self {
            Self::UserRejectedRequest => "Cancelled by user".to_string(),
            Self::Unauthorized(_) => "Not authorized - please connect your wallet first".to_string(),
            Self::UnsupportedMethod(_) => "This operation is not supported by your wallet".to_string(),
            Self::Disconnected => "Wallet disconnected - please reconnect".to_string(),
            Self::ChainDisconnected(chain_id) => format!("Wrong network - please switch to chain {}", chain_id),
            Self::UnrecognizedChain(chain_id) => format!("Chain {} not configured - please add it to your wallet first", chain_id),
            _ => format!("Error: {}", self),
        }
    }
}

impl From<Eip1193Error> for TransportError {
    fn from(err: Eip1193Error) -> Self {
        err.into_transport_error()
    }
}

impl From<JsValue> for Eip1193Error {
    fn from(value: JsValue) -> Self {
        Self::from_js_value(value)
    }
}

impl From<serde_json::Error> for Eip1193Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerializationError(err.to_string())
    }
}

impl From<Eip1193Error> for JsValue {
    fn from(err: Eip1193Error) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        assert_eq!(Eip1193Error::UserRejectedRequest.code(), 4001);
        assert_eq!(Eip1193Error::Unauthorized("test".into()).code(), 4100);
        assert_eq!(Eip1193Error::UnsupportedMethod("test".into()).code(), 4200);
        assert_eq!(Eip1193Error::Disconnected.code(), 4900);
        assert_eq!(Eip1193Error::ChainDisconnected(137).code(), 4901);
        assert_eq!(Eip1193Error::UnrecognizedChain(42161).code(), 4902);
    }

    #[test]
    fn test_from_code() {
        match Eip1193Error::from_code(4001, "User denied".into()) {
            Eip1193Error::UserRejectedRequest => (),
            _ => panic!("Expected UserRejectedRequest"),
        }

        match Eip1193Error::from_code(4100, "Not authorized".into()) {
            Eip1193Error::Unauthorized(_) => (),
            _ => panic!("Expected Unauthorized"),
        }

        match Eip1193Error::from_code(4902, "Chain 137 not added".into()) {
            Eip1193Error::UnrecognizedChain(_) => (),
            _ => panic!("Expected UnrecognizedChain"),
        }

        // Non-EIP-1193 codes should be wrapped as UnknownError
        match Eip1193Error::from_code(-32602, "Invalid params".into()) {
            Eip1193Error::UnknownError { code, .. } => {
                assert_eq!(code, -32602);
            },
            _ => panic!("Expected UnknownError"),
        }
    }

    #[test]
    fn test_error_classification() {
        let user_rejection = Eip1193Error::UserRejectedRequest;
        assert!(user_rejection.is_user_rejection());
        assert!(user_rejection.is_authorization_error());

        let chain_error = Eip1193Error::ChainDisconnected(1);
        assert!(chain_error.is_chain_error());
        assert!(!chain_error.is_user_rejection());

        let unauthorized = Eip1193Error::Unauthorized("test".into());
        assert!(unauthorized.is_authorization_error());
        assert!(!unauthorized.is_chain_error());
    }

    #[test]
    fn test_user_messages() {
        let user_rejection = Eip1193Error::UserRejectedRequest;
        assert_eq!(user_rejection.user_message(), "Cancelled by user");

        let chain_disconnected = Eip1193Error::ChainDisconnected(137);
        assert_eq!(chain_disconnected.user_message(), "Wrong network - please switch to chain 137");

        let unrecognized = Eip1193Error::UnrecognizedChain(42161);
        assert_eq!(unrecognized.user_message(), "Chain 42161 not configured - please add it to your wallet first");
    }
}
