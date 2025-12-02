use leptos::prelude::*;
use leptos::portal::Portal;
use leptos::callback::UnsyncCallback;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{MouseEvent, KeyboardEvent};

#[component]
pub fn Dialog(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] on_close: UnsyncCallback<()>,
    children: ChildrenFn,
) -> impl IntoView {
    // Add global ESC key listener when modal is open
    Effect::new(move |_| {
        if open.get() {
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                let on_close_clone = on_close.clone();
                let closure = Closure::<dyn Fn(KeyboardEvent)>::new(move |event: KeyboardEvent| {
                    if event.key() == "Escape" {
                        on_close_clone.run(());
                    }
                });

                let _ = document.add_event_listener_with_callback(
                    "keydown",
                    closure.as_ref().unchecked_ref()
                );

                // Store closure to keep it alive
                closure.forget();
            }
        }
    });

    view! {
        <Portal>
            <div
                class="rk-modal-overlay"
                style=move || {
                    let display = if open.get() { "flex" } else { "none" };
                    format!(
                        "position: fixed; top: 0; left: 0; right: 0; bottom: 0; \
                         display: {}; align-items: center; justify-content: center; \
                         background: var(--rk-colors-modalBackdrop, rgba(0, 0, 0, 0.3)); \
                         backdrop-filter: var(--rk-blurs-modalOverlay, blur(0px)); \
                         z-index: 999999999; animation: fadeIn 150ms ease;",
                        display
                    )
                }
                on:click=move |ev: MouseEvent| {
                    if let Some(target) = ev.target() {
                        if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                            if element.class_list().contains("rk-modal-overlay") {
                                on_close.run(());
                            }
                        }
                    }
                }
                data-rk=""
            >
                <div
                    class="rk-modal-content"
                    style="background: var(--rk-colors-modalBackground, #FFF); \
                           border-radius: var(--rk-radii-modal, 24px); \
                           padding: 24px; max-width: 400px; width: 100%; margin: 16px; \
                           position: relative; \
                           box-shadow: var(--rk-shadows-dialog, 0px 8px 32px rgba(0, 0, 0, 0.32)); \
                           border: 1px solid var(--rk-colors-modalBorder, transparent); \
                           animation: slideUp 350ms cubic-bezier(0.15, 1.15, 0.6, 1.00), fadeIn 150ms ease;"
                    on:click=|ev: MouseEvent| {
                        ev.stop_propagation();
                    }
                >
                    {children()}
                </div>
            </div>
        </Portal>
    }
}
