use leptos::prelude::*;
use leptos::callback::UnsyncCallback;
use wasm_bindgen::JsCast;
use web_sys::{MouseEvent, KeyboardEvent};

#[component]
pub fn Dialog(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] on_close: UnsyncCallback<()>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <Show when=move || open.get()>
            <div
                class="rk-modal-overlay fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4"
                on:click=move |ev: MouseEvent| {
                    if let Some(target) = ev.target() {
                        if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                            if element.class_list().contains("rk-modal-overlay") {
                                on_close.run(());
                            }
                        }
                    }
                }
                on:keydown=move |ev: KeyboardEvent| {
                    if ev.key() == "Escape" {
                        on_close.run(());
                    }
                }
                data-rk=""
            >
                <div class="rk-modal-content bg-white dark:bg-gray-800 rounded-xl p-6 max-w-md w-full shadow-xl border border-gray-200 dark:border-gray-700">
                    {children()}
                </div>
            </div>
        </Show>
    }
}
