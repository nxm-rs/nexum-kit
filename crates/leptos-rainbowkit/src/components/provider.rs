use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlStyleElement;
use crate::state::modal::provide_modal_state;
use crate::theme::{Theme, ThemeOptions, LightTheme};

#[component]
pub fn RainbowKitProvider<T: Theme + Clone + 'static>(
    #[prop(optional)] theme: Option<T>,
    #[prop(optional)] theme_options: Option<ThemeOptions>,
    children: Children,
) -> impl IntoView where T: Default {
    // Provide modal state
    provide_modal_state();

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
                let style_id = "rainbowkit-theme-vars";
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
        <div data-rk="">
            {children()}
        </div>
    }
}

// Simplified provider for when you want to use LightTheme (most common case)
#[component]
pub fn RainbowKitProviderSimple(
    #[prop(optional)] theme_options: Option<ThemeOptions>,
    children: Children,
) -> impl IntoView {
    provide_modal_state();

    let options = theme_options.unwrap_or_default();
    let theme_vars = LightTheme.build(&options);
    let css_string = theme_vars.to_css_string();

    // Inject CSS variables globally using a style tag in the head
    Effect::new(move |_| {
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(head) = document.head() {
                let style_id = "rainbowkit-theme-vars";
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
        <div data-rk="">
            {children()}
        </div>
    }
}
