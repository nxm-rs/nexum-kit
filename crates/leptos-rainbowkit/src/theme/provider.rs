use leptos::prelude::*;
use leptos_meta::Style;
use super::types::{Theme, ThemeOptions, ThemeVars};

#[derive(Clone)]
pub struct ThemeContext {
    pub theme_vars: RwSignal<ThemeVars>,
    pub options: RwSignal<ThemeOptions>,
}

impl ThemeContext {
    pub fn new<T: Theme>(theme: &T, options: ThemeOptions) -> Self {
        let theme_vars = RwSignal::new(theme.build(&options));
        let options = RwSignal::new(options);

        Self {
            theme_vars,
            options,
        }
    }

    pub fn set_options(&self, options: ThemeOptions) {
        self.options.set(options);
    }

    pub fn css_string(&self) -> String {
        self.theme_vars.with(|vars| vars.to_css_string())
    }
}

/// Provide theme in the Leptos context
pub fn provide_theme<T: Theme>(theme: &T, options: ThemeOptions) -> ThemeContext {
    let ctx = ThemeContext::new(theme, options);
    provide_context(ctx.clone());
    ctx
}

/// Get theme from Leptos context
pub fn use_theme() -> ThemeContext {
    expect_context::<ThemeContext>()
}

/// Theme Provider component that injects theme CSS variables
#[component]
pub fn ThemeProvider<T: Theme + Clone + 'static>(
    #[prop(optional)] theme: Option<T>,
    #[prop(optional)] options: Option<ThemeOptions>,
    children: Children,
) -> impl IntoView where T: Default {
    let theme_instance = theme.unwrap_or_default();
    let options = options.unwrap_or_default();

    let theme_ctx = provide_theme(&theme_instance, options);

    view! {
        <Style>
            {move || format!(":root {{ {} }}", theme_ctx.css_string())}
        </Style>
        {children()}
    }
}
