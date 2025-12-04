use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlStyleElement;
use std::collections::HashMap;
use crate::state::modal::provide_modal_state;
use crate::state::connection::provide_connection_state;
use crate::state::transaction::provide_transaction_store;
use crate::theme::{Theme, ThemeOptions, LightTheme};
use crate::i18n::{Locale, provide_i18n};

#[component]
pub fn NexumKitProvider<T: Theme + Clone + 'static>(
    /// RPC URL mappings for each chain (chain_id -> rpc_url)
    ///
    /// Example:
    /// ```rust
    /// let mut transports = HashMap::new();
    /// transports.insert(1, "https://eth-mainnet.g.alchemy.com/v2/YOUR-API-KEY".to_string());
    /// transports.insert(137, "https://polygon-mainnet.g.alchemy.com/v2/YOUR-API-KEY".to_string());
    /// ```
    transports: HashMap<u64, String>,
    #[prop(optional)] theme: Option<T>,
    #[prop(optional)] theme_options: Option<ThemeOptions>,
    #[prop(optional)] locale: Option<Locale>,
    children: Children,
) -> impl IntoView where T: Default {
    // Provide modal state
    provide_modal_state();

    // Provide connection state with transports
    provide_connection_state(transports.clone());

    // Provide transaction store
    provide_transaction_store();

    // Provide i18n
    provide_i18n(locale.unwrap_or_default());

    // Build theme with options
    let theme_instance = theme.unwrap_or_default();
    let options = theme_options.unwrap_or_default();
    let theme_vars = theme_instance.build(&options);
    let css_string = theme_vars.to_css_string();

    // Inject CSS variables globally using a style tag in the head
    Effect::new(move |_| {
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(head) = document.head() {
                // Create or update style element
                let style_id = "nexumkit-theme-vars";
                let style_element = if let Some(existing) = document.get_element_by_id(style_id) {
                    existing.dyn_into::<HtmlStyleElement>().ok()
                } else {
                    document.create_element("style").ok()
                        .and_then(|el| {
                            el.set_id(style_id);
                            let _ = head.append_child(&el);
                            el.dyn_into::<HtmlStyleElement>().ok()
                        })
                };

                if let Some(style) = style_element {
                    let global_css = format!(":root {{ {} }}", css_string);
                    style.set_inner_html(&global_css);
                }
            }
        }
    });

    view! {
        <div data-nk="">
            {children()}
        </div>
    }
}

// Simplified provider for when you want to use LightTheme (most common case)
#[component]
pub fn NexumKitProviderSimple(
    /// RPC URL mappings for each chain (chain_id -> rpc_url)
    transports: HashMap<u64, String>,
    #[prop(optional)] theme_options: Option<ThemeOptions>,
    #[prop(optional)] locale: Option<Locale>,
    children: Children,
) -> impl IntoView {
    provide_modal_state();
    provide_connection_state(transports);
    provide_transaction_store();
    provide_i18n(locale.unwrap_or_default());

    let options = theme_options.unwrap_or_default();
    let theme_vars = LightTheme.build(&options);
    let css_string = theme_vars.to_css_string();

    // Inject CSS variables globally using a style tag in the head
    Effect::new(move |_| {
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(head) = document.head() {
                let style_id = "nexumkit-theme-vars";
                let style_element = if let Some(existing) = document.get_element_by_id(style_id) {
                    existing.dyn_into::<HtmlStyleElement>().ok()
                } else {
                    document.create_element("style").ok()
                        .and_then(|el| {
                            el.set_id(style_id);
                            let _ = head.append_child(&el);
                            el.dyn_into::<HtmlStyleElement>().ok()
                        })
                };

                if let Some(style) = style_element {
                    let global_css = format!(":root {{ {} }}", css_string);
                    style.set_inner_html(&global_css);
                }
            }
        }
    });

    view! {
        <div data-nk="">
            {children()}
        </div>
    }
}
