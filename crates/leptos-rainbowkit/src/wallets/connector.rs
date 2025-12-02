use wasm_bindgen::prelude::*;
use web_sys::js_sys::Reflect;
use web_sys::Window;

/// Provider flags used to identify specific wallets
///
/// Follows the pattern from the original RainbowKit's WalletProviderFlags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderFlag {
    IsMetaMask,
    IsCoinbaseWallet,
    IsTrust,
    IsTrustWallet,
    IsPhantom,
    IsRabby,
    IsBraveWallet,
    IsExodus,
    IsFrame,
    IsOkxWallet,
    IsOneKey,
    IsTokenPocket,
    IsZerion,
}

impl ProviderFlag {
    /// Convert flag to the JavaScript property name
    pub fn as_str(&self) -> &'static str {
        match self {
            ProviderFlag::IsMetaMask => "isMetaMask",
            ProviderFlag::IsCoinbaseWallet => "isCoinbaseWallet",
            ProviderFlag::IsTrust => "isTrust",
            ProviderFlag::IsTrustWallet => "isTrustWallet",
            ProviderFlag::IsPhantom => "isPhantom",
            ProviderFlag::IsRabby => "isRabby",
            ProviderFlag::IsBraveWallet => "isBraveWallet",
            ProviderFlag::IsExodus => "isExodus",
            ProviderFlag::IsFrame => "isFrame",
            ProviderFlag::IsOkxWallet => "isOkxWallet",
            ProviderFlag::IsOneKey => "$onekey",
            ProviderFlag::IsTokenPocket => "isTokenPocket",
            ProviderFlag::IsZerion => "isZerion",
        }
    }
}

/// Get injected provider from window.ethereum
///
/// This function searches for a specific provider based on flags or namespace.
/// It handles the case where multiple wallets are installed (window.ethereum.providers[])
pub fn get_injected_provider(flag: Option<ProviderFlag>, namespace: Option<&str>) -> Option<JsValue> {
    let window = web_sys::window()?;

    // If namespace is specified, search for it in the window object
    if let Some(ns) = namespace {
        return get_window_provider_namespace(&window, ns);
    }

    // Get window.ethereum
    let ethereum = get_ethereum(&window)?;

    // If a specific flag is requested, search for that provider
    if let Some(flag) = flag {
        return get_explicit_injected_provider(&ethereum, flag);
    }

    // Return the default ethereum provider
    Some(ethereum)
}

/// Get window.ethereum
fn get_ethereum(window: &Window) -> Option<JsValue> {
    Reflect::get(window, &JsValue::from_str("ethereum")).ok()
}

/// Get a provider by searching for a specific flag
///
/// This handles cases where multiple wallets inject providers into window.ethereum.providers[]
fn get_explicit_injected_provider(ethereum: &JsValue, flag: ProviderFlag) -> Option<JsValue> {
    let flag_str = flag.as_str();

    // Check if window.ethereum.providers exists (multiple wallets)
    if let Ok(providers) = Reflect::get(ethereum, &JsValue::from_str("providers")) {
        // If providers is an array, search through it
        if providers.is_array() {
            let providers_array = js_sys::Array::from(&providers);
            for i in 0..providers_array.length() {
                let provider = providers_array.get(i);
                // Check if this provider has the flag we're looking for
                if let Ok(has_flag) = Reflect::get(&provider, &JsValue::from_str(flag_str)) {
                    if has_flag.is_truthy() {
                        return Some(provider);
                    }
                }
            }
        }
    }

    // Check if window.ethereum itself has the flag
    if let Ok(has_flag) = Reflect::get(ethereum, &JsValue::from_str(flag_str)) {
        if has_flag.is_truthy() {
            return Some(ethereum.clone());
        }
    }

    None
}

/// Get provider by namespace (e.g., window.phantom.ethereum)
fn get_window_provider_namespace(window: &Window, namespace: &str) -> Option<JsValue> {
    let parts: Vec<&str> = namespace.split('.').collect();

    let mut current = JsValue::from(window.clone());

    for part in parts {
        match Reflect::get(&current, &JsValue::from_str(part)) {
            Ok(next) => {
                if next.is_undefined() {
                    return None;
                }
                current = next;
            }
            Err(_) => return None,
        }
    }

    Some(current)
}

/// Check if a provider has a specific flag
pub fn has_provider_flag(provider: &JsValue, flag: ProviderFlag) -> bool {
    Reflect::get(provider, &JsValue::from_str(flag.as_str()))
        .map(|v| v.is_truthy())
        .unwrap_or(false)
}

/// Check if MetaMask is the injected provider
pub fn is_metamask(provider: &JsValue) -> bool {
    has_provider_flag(provider, ProviderFlag::IsMetaMask)
}
