use super::types::Theme;

pub struct LightTheme;

impl Theme for LightTheme {
    fn name(&self) -> &'static str {
        "light"
    }

    fn css_vars(&self) -> &'static str {
        ""
    }
}
