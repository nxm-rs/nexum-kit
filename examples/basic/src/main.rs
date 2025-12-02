use leptos::prelude::*;
use leptos_rainbowkit::prelude::*;
use leptos_rainbowkit::components::modals::ConnectModal;

fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <App />
        }
    })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <RainbowKitProvider>
            <div class="min-h-screen bg-gray-100 dark:bg-gray-900 flex flex-col items-center justify-center gap-4">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-white">
                    "Leptos RainbowKit"
                </h1>
                <p class="text-gray-600 dark:text-gray-400">
                    "Phase 1: Foundation Complete"
                </p>
                <ConnectButton />
                <ConnectModal />
            </div>
        </RainbowKitProvider>
    }
}
