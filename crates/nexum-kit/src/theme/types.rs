/// Accent color presets matching original RainbowKit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccentColorPreset {
    Blue,
    Green,
    Orange,
    Pink,
    Purple,
    Red,
}

impl AccentColorPreset {
    pub fn to_colors(&self) -> (String, String) {
        match self {
            Self::Blue => ("#0E76FD".to_string(), "#FFF".to_string()),
            Self::Green => ("#1DB847".to_string(), "#FFF".to_string()),
            Self::Orange => ("#FF801F".to_string(), "#FFF".to_string()),
            Self::Pink => ("#FF5CA0".to_string(), "#FFF".to_string()),
            Self::Purple => ("#5F5AFA".to_string(), "#FFF".to_string()),
            Self::Red => ("#FA423C".to_string(), "#FFF".to_string()),
        }
    }
}

/// Border radius scale
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderRadius {
    None,
    Small,
    Medium,
    Large,
}

impl BorderRadius {
    pub fn action_button(&self) -> &'static str {
        match self {
            Self::None => "0px",
            Self::Small => "4px",
            Self::Medium => "10px",
            Self::Large => "9999px",
        }
    }

    pub fn connect_button(&self) -> &'static str {
        match self {
            Self::None => "0px",
            Self::Small => "4px",
            Self::Medium => "8px",
            Self::Large => "12px",
        }
    }

    pub fn modal(&self) -> &'static str {
        match self {
            Self::None => "0px",
            Self::Small => "8px",
            Self::Medium => "16px",
            Self::Large => "24px",
        }
    }

    pub fn modal_mobile(&self) -> &'static str {
        match self {
            Self::None => "0px",
            Self::Small => "8px",
            Self::Medium => "18px",
            Self::Large => "28px",
        }
    }
}

/// Font stack
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStack {
    System,
    Rounded,
}

impl FontStack {
    pub fn css_value(&self) -> &'static str {
        match self {
            Self::System => "-apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, Helvetica, Arial, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\"",
            Self::Rounded => "SFRounded, ui-rounded, \"SF Pro Rounded\", -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, Helvetica, Arial, sans-serif",
        }
    }
}

/// Overlay blur
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayBlur {
    None,
    Small,
    Large,
}

impl OverlayBlur {
    pub fn css_value(&self) -> &'static str {
        match self {
            Self::None => "blur(0px)",
            Self::Small => "blur(4px)",
            Self::Large => "blur(20px)",
        }
    }
}

/// Theme options (customization)
#[derive(Debug, Clone)]
pub struct ThemeOptions {
    pub accent_color: Option<String>,
    pub accent_color_foreground: Option<String>,
    pub border_radius: BorderRadius,
    pub font_stack: FontStack,
    pub overlay_blur: OverlayBlur,
}

impl Default for ThemeOptions {
    fn default() -> Self {
        Self {
            accent_color: None,
            accent_color_foreground: None,
            border_radius: BorderRadius::Large,
            font_stack: FontStack::Rounded,
            overlay_blur: OverlayBlur::None,
        }
    }
}

/// Complete theme definition with all CSS custom properties
#[derive(Debug, Clone)]
pub struct ThemeVars {
    // Colors
    pub accent_color: String,
    pub accent_color_foreground: String,
    pub action_button_border: String,
    pub action_button_border_mobile: String,
    pub action_button_secondary_background: String,
    pub close_button: String,
    pub close_button_background: String,
    pub connect_button_background: String,
    pub connect_button_background_error: String,
    pub connect_button_inner_background: String,
    pub connect_button_text: String,
    pub connect_button_text_error: String,
    pub connection_indicator: String,
    pub download_bottom_card_background: String,
    pub download_top_card_background: String,
    pub error: String,
    pub general_border: String,
    pub general_border_dim: String,
    pub menu_item_background: String,
    pub modal_backdrop: String,
    pub modal_background: String,
    pub modal_border: String,
    pub modal_text: String,
    pub modal_text_dim: String,
    pub modal_text_secondary: String,
    pub profile_action: String,
    pub profile_action_hover: String,
    pub profile_foreground: String,
    pub selected_option_border: String,
    pub standby: String,

    // Fonts
    pub font_body: String,

    // Radii
    pub radii_action_button: String,
    pub radii_connect_button: String,
    pub radii_menu_button: String,
    pub radii_modal: String,
    pub radii_modal_mobile: String,

    // Shadows
    pub shadow_connect_button: String,
    pub shadow_dialog: String,
    pub shadow_profile_details_action: String,
    pub shadow_selected_option: String,
    pub shadow_selected_wallet: String,
    pub shadow_wallet_logo: String,

    // Blurs
    pub blur_modal_overlay: String,
}

impl ThemeVars {
    /// Convert theme to CSS custom properties string
    pub fn to_css_string(&self) -> String {
        format!(
            "--rk-colors-accentColor:{};--rk-colors-accentColorForeground:{};--rk-colors-actionButtonBorder:{};--rk-colors-actionButtonBorderMobile:{};--rk-colors-actionButtonSecondaryBackground:{};--rk-colors-closeButton:{};--rk-colors-closeButtonBackground:{};--rk-colors-connectButtonBackground:{};--rk-colors-connectButtonBackgroundError:{};--rk-colors-connectButtonInnerBackground:{};--rk-colors-connectButtonText:{};--rk-colors-connectButtonTextError:{};--rk-colors-connectionIndicator:{};--rk-colors-downloadBottomCardBackground:{};--rk-colors-downloadTopCardBackground:{};--rk-colors-error:{};--rk-colors-generalBorder:{};--rk-colors-generalBorderDim:{};--rk-colors-menuItemBackground:{};--rk-colors-modalBackdrop:{};--rk-colors-modalBackground:{};--rk-colors-modalBorder:{};--rk-colors-modalText:{};--rk-colors-modalTextDim:{};--rk-colors-modalTextSecondary:{};--rk-colors-profileAction:{};--rk-colors-profileActionHover:{};--rk-colors-profileForeground:{};--rk-colors-selectedOptionBorder:{};--rk-colors-standby:{};--rk-fonts-body:{};--rk-radii-actionButton:{};--rk-radii-connectButton:{};--rk-radii-menuButton:{};--rk-radii-modal:{};--rk-radii-modalMobile:{};--rk-shadows-connectButton:{};--rk-shadows-dialog:{};--rk-shadows-profileDetailsAction:{};--rk-shadows-selectedOption:{};--rk-shadows-selectedWallet:{};--rk-shadows-walletLogo:{};--rk-blurs-modalOverlay:{};",
            self.accent_color,
            self.accent_color_foreground,
            self.action_button_border,
            self.action_button_border_mobile,
            self.action_button_secondary_background,
            self.close_button,
            self.close_button_background,
            self.connect_button_background,
            self.connect_button_background_error,
            self.connect_button_inner_background,
            self.connect_button_text,
            self.connect_button_text_error,
            self.connection_indicator,
            self.download_bottom_card_background,
            self.download_top_card_background,
            self.error,
            self.general_border,
            self.general_border_dim,
            self.menu_item_background,
            self.modal_backdrop,
            self.modal_background,
            self.modal_border,
            self.modal_text,
            self.modal_text_dim,
            self.modal_text_secondary,
            self.profile_action,
            self.profile_action_hover,
            self.profile_foreground,
            self.selected_option_border,
            self.standby,
            self.font_body,
            self.radii_action_button,
            self.radii_connect_button,
            self.radii_menu_button,
            self.radii_modal,
            self.radii_modal_mobile,
            self.shadow_connect_button,
            self.shadow_dialog,
            self.shadow_profile_details_action,
            self.shadow_selected_option,
            self.shadow_selected_wallet,
            self.shadow_wallet_logo,
            self.blur_modal_overlay,
        )
    }
}

/// Theme trait for light/dark/custom themes
pub trait Theme {
    fn name(&self) -> &'static str;
    fn build(&self, options: &ThemeOptions) -> ThemeVars;
}
