use super::types::{Theme, ThemeOptions, ThemeVars};

#[derive(Default, Clone, Copy)]
pub struct MidnightTheme;

impl Theme for MidnightTheme {
    fn name(&self) -> &'static str {
        "midnight"
    }

    fn build(&self, options: &ThemeOptions) -> ThemeVars {
        let accent_color = options
            .accent_color
            .as_deref()
            .unwrap_or("#3898FF");
        let accent_color_foreground = options
            .accent_color_foreground
            .as_deref()
            .unwrap_or("#FFF");

        ThemeVars {
            // Accent colors
            accent_color: accent_color.to_string(),
            accent_color_foreground: accent_color_foreground.to_string(),

            // Action buttons
            action_button_border: "rgba(255, 255, 255, 0.04)".to_string(),
            action_button_border_mobile: "rgba(255, 255, 255, 0.1)".to_string(),
            action_button_secondary_background: "rgba(255, 255, 255, 0.08)".to_string(),

            // Close button
            close_button: "rgba(255, 255, 255, 0.7)".to_string(),
            close_button_background: "rgba(255, 255, 255, 0.08)".to_string(),

            // Connect button
            connect_button_background: "#000".to_string(),
            connect_button_background_error: "#FF494A".to_string(),
            connect_button_inner_background: "linear-gradient(0deg, rgba(255, 255, 255, 0.06), rgba(255, 255, 255, 0.12))".to_string(),
            connect_button_text: "#FFF".to_string(),
            connect_button_text_error: "#FFF".to_string(),

            // Connection indicator
            connection_indicator: "#30E000".to_string(),

            // Download cards
            download_bottom_card_background: "linear-gradient(126deg, rgba(0, 0, 0, 0) 9.49%, rgba(120, 120, 120, 0.1) 71.04%), #050505".to_string(),
            download_top_card_background: "linear-gradient(126deg, rgba(120, 120, 120, 0.1) 9.49%, rgba(0, 0, 0, 0) 71.04%), #050505".to_string(),

            // Error
            error: "#FF494A".to_string(),

            // Borders
            general_border: "rgba(255, 255, 255, 0.08)".to_string(),
            general_border_dim: "rgba(255, 255, 255, 0.04)".to_string(),

            // Menu items
            menu_item_background: "rgba(255, 255, 255, 0.08)".to_string(),

            // Modal
            modal_backdrop: "rgba(0, 0, 0, 0.7)".to_string(),
            modal_background: "#000".to_string(),
            modal_border: "rgba(255, 255, 255, 0.08)".to_string(),
            modal_text: "#FFF".to_string(),
            modal_text_dim: "rgba(255, 255, 255, 0.2)".to_string(),
            modal_text_secondary: "rgba(255, 255, 255, 0.6)".to_string(),

            // Profile
            profile_action: "rgba(255, 255, 255, 0.1)".to_string(),
            profile_action_hover: "rgba(255, 255, 255, 0.2)".to_string(),
            profile_foreground: "rgba(255, 255, 255, 0.06)".to_string(),

            // Selected options
            selected_option_border: "rgba(224, 232, 255, 0.1)".to_string(),

            // Standby
            standby: "#FFD641".to_string(),

            // Fonts
            font_body: options.font_stack.css_value().to_string(),

            // Radii
            radii_action_button: options.border_radius.action_button().to_string(),
            radii_connect_button: options.border_radius.connect_button().to_string(),
            radii_menu_button: options.border_radius.action_button().to_string(),
            radii_modal: options.border_radius.modal().to_string(),
            radii_modal_mobile: options.border_radius.modal_mobile().to_string(),

            // Shadows
            shadow_connect_button: "0px 4px 12px rgba(0, 0, 0, 0.1)".to_string(),
            shadow_dialog: "0px 8px 32px rgba(0, 0, 0, 0.32)".to_string(),
            shadow_profile_details_action: "0px 2px 6px rgba(37, 41, 46, 0.04)".to_string(),
            shadow_selected_option: "0px 2px 6px rgba(0, 0, 0, 0.24)".to_string(),
            shadow_selected_wallet: "0px 2px 6px rgba(0, 0, 0, 0.24)".to_string(),
            shadow_wallet_logo: "0px 2px 16px rgba(0, 0, 0, 0.16)".to_string(),

            // Blurs
            blur_modal_overlay: options.overlay_blur.css_value().to_string(),
        }
    }
}
