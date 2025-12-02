use super::types::{Theme, ThemeOptions, ThemeVars};

#[derive(Default, Clone, Copy)]
pub struct LightTheme;

impl Theme for LightTheme {
    fn name(&self) -> &'static str {
        "light"
    }

    fn build(&self, options: &ThemeOptions) -> ThemeVars {
        // Get accent colors (use provided or default to blue)
        let (accent_color, accent_color_foreground) = if let (Some(ac), Some(acf)) =
            (&options.accent_color, &options.accent_color_foreground) {
            (ac.clone(), acf.clone())
        } else {
            let default = super::types::AccentColorPreset::Blue.to_colors();
            (default.0, default.1)
        };

        ThemeVars {
            // Colors - matching lightTheme.ts exactly
            accent_color,
            accent_color_foreground,
            action_button_border: "rgba(0, 0, 0, 0.04)".to_string(),
            action_button_border_mobile: "rgba(0, 0, 0, 0.06)".to_string(),
            action_button_secondary_background: "rgba(0, 0, 0, 0.06)".to_string(),
            close_button: "rgba(60, 66, 66, 0.8)".to_string(),
            close_button_background: "rgba(0, 0, 0, 0.06)".to_string(),
            connect_button_background: "#FFF".to_string(),
            connect_button_background_error: "#FF494A".to_string(),
            connect_button_inner_background: "linear-gradient(0deg, rgba(0, 0, 0, 0.03), rgba(0, 0, 0, 0.06))".to_string(),
            connect_button_text: "#25292E".to_string(),
            connect_button_text_error: "#FFF".to_string(),
            connection_indicator: "#30E000".to_string(),
            download_bottom_card_background: "linear-gradient(126deg, rgba(255, 255, 255, 0) 9.49%, rgba(171, 171, 171, 0.04) 71.04%), #FFFFFF".to_string(),
            download_top_card_background: "linear-gradient(126deg, rgba(171, 171, 171, 0.2) 9.49%, rgba(255, 255, 255, 0) 71.04%), #FFFFFF".to_string(),
            error: "#FF494A".to_string(),
            general_border: "rgba(0, 0, 0, 0.06)".to_string(),
            general_border_dim: "rgba(0, 0, 0, 0.03)".to_string(),
            menu_item_background: "rgba(60, 66, 66, 0.1)".to_string(),
            modal_backdrop: "rgba(0, 0, 0, 0.3)".to_string(),
            modal_background: "#FFF".to_string(),
            modal_border: "transparent".to_string(),
            modal_text: "#25292E".to_string(),
            modal_text_dim: "rgba(60, 66, 66, 0.3)".to_string(),
            modal_text_secondary: "rgba(60, 66, 66, 0.6)".to_string(),
            profile_action: "#FFF".to_string(),
            profile_action_hover: "rgba(255, 255, 255, 0.5)".to_string(),
            profile_foreground: "rgba(60, 66, 66, 0.06)".to_string(),
            selected_option_border: "rgba(60, 66, 66, 0.1)".to_string(),
            standby: "#FFD641".to_string(),

            // Fonts
            font_body: options.font_stack.css_value().to_string(),

            // Radii
            radii_action_button: options.border_radius.action_button().to_string(),
            radii_connect_button: options.border_radius.connect_button().to_string(),
            radii_menu_button: options.border_radius.connect_button().to_string(),
            radii_modal: options.border_radius.modal().to_string(),
            radii_modal_mobile: options.border_radius.modal_mobile().to_string(),

            // Shadows
            shadow_connect_button: "0px 4px 12px rgba(0, 0, 0, 0.1)".to_string(),
            shadow_dialog: "0px 8px 32px rgba(0, 0, 0, 0.32)".to_string(),
            shadow_profile_details_action: "0px 2px 6px rgba(37, 41, 46, 0.04)".to_string(),
            shadow_selected_option: "0px 2px 6px rgba(0, 0, 0, 0.24)".to_string(),
            shadow_selected_wallet: "0px 2px 6px rgba(0, 0, 0, 0.12)".to_string(),
            shadow_wallet_logo: "0px 2px 16px rgba(0, 0, 0, 0.16)".to_string(),

            // Blurs
            blur_modal_overlay: options.overlay_blur.css_value().to_string(),
        }
    }
}
