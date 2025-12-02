use leptos::prelude::*;
use leptos::callback::{Callback, UnsyncCallback};
use leptos_rainbowkit::prelude::*;
use leptos_rainbowkit::components::modals::{ConnectModal, AccountModal};
use leptos_rainbowkit::theme::{LightTheme, DarkTheme, MidnightTheme, ThemeOptions, BorderRadius, FontStack, OverlayBlur};

fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <App />
        }
    })
}

#[derive(Clone, Copy, PartialEq)]
enum ThemeMode {
    Light,
    Dark,
    Midnight,
}

impl ThemeMode {
    fn next(&self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Midnight,
            Self::Midnight => Self::Light,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Dark => "Dark",
            Self::Midnight => "Midnight",
        }
    }
}

#[component]
fn App() -> impl IntoView {
    let (theme_mode, set_theme_mode) = signal(ThemeMode::Light);

    let bg_color = Callback::new(move |_: ()| match theme_mode.get() {
        ThemeMode::Light => "#f5f5f5",
        ThemeMode::Dark => "#1A1B1F",
        ThemeMode::Midnight => "#000",
    });

    let text_color = Callback::new(move |_: ()| match theme_mode.get() {
        ThemeMode::Light => "#25292E",
        ThemeMode::Dark => "#FFF",
        ThemeMode::Midnight => "#FFF",
    });

    let card_bg = Callback::new(move |_: ()| match theme_mode.get() {
        ThemeMode::Light => "#FFF",
        ThemeMode::Dark => "#1A1B1F",
        ThemeMode::Midnight => "#000",
    });

    let handle_theme_toggle = UnsyncCallback::new(move |_: leptos::ev::MouseEvent| {
        set_theme_mode.set(theme_mode.get().next());
    });

    // Create theme options with blur enabled
    let theme_options = ThemeOptions {
        accent_color: None,
        accent_color_foreground: None,
        border_radius: BorderRadius::Large,
        font_stack: FontStack::Rounded,
        overlay_blur: OverlayBlur::Small, // Enable blur on modal backdrop
    };

    view! {
        {move || match theme_mode.get() {
            ThemeMode::Light => view! {
                <RainbowKitProvider theme=LightTheme theme_options=theme_options.clone()>
                    <AppContent
                        theme_mode=theme_mode
                        bg_color=bg_color
                        text_color=text_color
                        card_bg=card_bg
                        on_toggle=handle_theme_toggle
                    />
                </RainbowKitProvider>
            }.into_any(),
            ThemeMode::Dark => view! {
                <RainbowKitProvider theme=DarkTheme theme_options=theme_options.clone()>
                    <AppContent
                        theme_mode=theme_mode
                        bg_color=bg_color
                        text_color=text_color
                        card_bg=card_bg
                        on_toggle=handle_theme_toggle
                    />
                </RainbowKitProvider>
            }.into_any(),
            ThemeMode::Midnight => view! {
                <RainbowKitProvider theme=MidnightTheme theme_options=theme_options.clone()>
                    <AppContent
                        theme_mode=theme_mode
                        bg_color=bg_color
                        text_color=text_color
                        card_bg=card_bg
                        on_toggle=handle_theme_toggle
                    />
                </RainbowKitProvider>
            }.into_any(),
        }}
    }
}

#[component]
fn AppContent(
    theme_mode: ReadSignal<ThemeMode>,
    bg_color: Callback<(), &'static str>,
    text_color: Callback<(), &'static str>,
    card_bg: Callback<(), &'static str>,
    on_toggle: UnsyncCallback<leptos::ev::MouseEvent>,
) -> impl IntoView {
    view! {
        <div
            class="min-h-screen flex flex-col items-center justify-center gap-6 p-4"
            style=move || format!("background: {}; transition: background 0.3s ease;", bg_color.run(()))
        >
            <div class="text-center space-y-2">
                <h1
                    class="text-5xl font-bold"
                    style=move || format!(
                        "color: {}; transition: color 0.3s ease;",
                        text_color.run(())
                    )
                >
                    "Leptos RainbowKit"
                </h1>
                <p
                    class="text-lg"
                    style=move || format!("color: {}; opacity: 0.8;", text_color.run(()))
                >
                    "Phase 2: Core Components Complete ✨"
                </p>
            </div>

            <div
                class="rounded-2xl p-8 shadow-lg max-w-md w-full space-y-4"
                style=move || format!(
                    "background: {}; transition: background 0.3s ease;",
                    card_bg.run(())
                )
            >
                <div class="flex justify-between items-center mb-4">
                    <h2
                        class="text-2xl font-bold"
                        style=move || format!("color: {};", text_color.run(()))
                    >
                        "Features Implemented"
                    </h2>
                    <button
                        class="px-3 py-1 rounded-lg text-sm font-medium"
                        style="background: var(--rk-colors-accentColor); color: var(--rk-colors-accentColorForeground);"
                        on:click=move |ev| on_toggle.run(ev)
                    >
                        {move || format!("Theme: {}", theme_mode.get().name())}
                    </button>
                </div>
                <ul
                    class="space-y-2"
                    style=move || format!("color: {}; opacity: 0.8;", text_color.run(()))
                >
                    <li>"✅ Theme system with CSS variables"</li>
                    <li>"✅ Modal state management"</li>
                    <li>"✅ Animated dialog component"</li>
                    <li>"✅ Connect button with theming"</li>
                    <li>"✅ Light, Dark & Midnight themes"</li>
                    <li>"✅ Box & Text primitive components"</li>
                </ul>

                <div class="pt-4">
                    <ConnectButton />
                </div>

                <ConnectModal />
                <AccountModal />

                <p
                    class="text-sm pt-2"
                    style=move || format!("color: {}; opacity: 0.6;", text_color.run(()))
                >
                    "Click the button to see the modal! 🎉"
                </p>
            </div>

            <ConnectModal />
        </div>
    }
}
