use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CustomEvent, Event};
use serde::{Deserialize, Serialize};
use js_sys::Reflect;

/// EIP-6963 Provider Information
///
/// This matches the structure defined in EIP-6963:
/// https://eips.ethereum.org/EIPS/eip-6963
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EIP6963ProviderInfo {
    /// Unique identifier for this provider instance (UUIDv4)
    pub uuid: String,
    /// Human-readable wallet name
    pub name: String,
    /// Icon as a data URL (SVG or PNG)
    pub icon: String,
    /// EIP-6963 Reverse DNS identifier (e.g., "io.metamask", "com.trustwallet.app")
    pub rdns: String,
}

/// EIP-6963 Provider - combines info with the actual provider object
#[derive(Debug, Clone)]
pub struct EIP6963Provider {
    pub info: EIP6963ProviderInfo,
    pub provider: JsValue,
}

/// Setup EIP-6963 discovery with a callback for reactive updates
///
/// This is the Leptos-friendly version that calls a callback whenever a new provider
/// is discovered. Use this with a WriteSignal to reactively update your UI.
pub fn setup_eip6963_discovery<F>(on_provider_discovered: F)
where
    F: Fn(EIP6963Provider) + 'static,
{
    let window = match web_sys::window() {
        Some(w) => w,
        None => {
            log::warn!("No window object available for EIP-6963 discovery");
            return;
        }
    };

    // Create event listener
    let closure = Closure::wrap(Box::new(move |event: Event| {
        log::debug!("EIP-6963: Received event");

        if let Ok(custom_event) = event.dyn_into::<CustomEvent>() {
            let detail = custom_event.detail();
            log::debug!("EIP-6963: Got event detail: {:?}", detail);

            // Parse provider info from detail.info
            if let Ok(info_obj) = Reflect::get(&detail, &JsValue::from_str("info")) {
                log::debug!("EIP-6963: Got info object");

                // Extract fields manually
                let uuid = Reflect::get(&info_obj, &JsValue::from_str("uuid"))
                    .ok()
                    .and_then(|v| v.as_string())
                    .unwrap_or_else(|| "unknown".to_string());

                let name = Reflect::get(&info_obj, &JsValue::from_str("name"))
                    .ok()
                    .and_then(|v| v.as_string())
                    .unwrap_or_else(|| "Unknown Wallet".to_string());

                let icon = Reflect::get(&info_obj, &JsValue::from_str("icon"))
                    .ok()
                    .and_then(|v| v.as_string())
                    .unwrap_or_default();

                let rdns = Reflect::get(&info_obj, &JsValue::from_str("rdns"))
                    .ok()
                    .and_then(|v| v.as_string())
                    .unwrap_or_default();

                // Get provider object from detail.provider
                if let Ok(provider_obj) = Reflect::get(&detail, &JsValue::from_str("provider")) {
                    let info = EIP6963ProviderInfo {
                        uuid,
                        name: name.clone(),
                        icon,
                        rdns,
                    };

                    let provider = EIP6963Provider {
                        info,
                        provider: provider_obj,
                    };

                    log::info!("EIP-6963: Discovered {}", name);
                    on_provider_discovered(provider);
                } else {
                    log::warn!("EIP-6963: No provider object in detail");
                }
            } else {
                log::warn!("EIP-6963: No info object in detail");
            }
        }
    }) as Box<dyn FnMut(_)>);

    // Add event listener
    if let Err(e) = window.add_event_listener_with_callback(
        "eip6963:announceProvider",
        closure.as_ref().unchecked_ref(),
    ) {
        log::error!("Failed to add EIP-6963 event listener: {:?}", e);
        return;
    }

    // Keep closure alive
    closure.forget();

    log::info!("EIP-6963: Event listener registered");

    // Request providers - dispatch the request event
    let request_event = CustomEvent::new("eip6963:requestProvider")
        .expect("Failed to create eip6963:requestProvider event");

    if let Err(e) = window.dispatch_event(&request_event) {
        log::error!("Failed to dispatch EIP-6963 request: {:?}", e);
    } else {
        log::info!("EIP-6963: Dispatched requestProvider event");
    }
}
