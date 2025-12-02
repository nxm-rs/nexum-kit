use super::types::Theme;

pub struct DarkTheme;

impl Theme for DarkTheme {
    fn name(&self) -> &'static str {
        "dark"
    }

    fn css_vars(&self) -> &'static str {
        ""
    }
}
