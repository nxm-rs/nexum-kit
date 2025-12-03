use leptos::prelude::*;
use qrcode::{QrCode as QrCodeGen, render::svg};

/// QR Code component for displaying WalletConnect URIs and other data
///
/// # Arguments
/// * `data` - The data to encode in the QR code (typically a WalletConnect URI)
/// * `size` - The size of the QR code in pixels (default: 256)
/// * `class` - Optional CSS classes to apply to the container
#[component]
pub fn QrCode(
    #[prop(into)] data: String,
    #[prop(default = 256)] size: usize,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let svg_data = move || {
        QrCodeGen::new(data.clone())
            .ok()
            .map(|code| {
                code.render::<svg::Color>()
                    .min_dimensions(size as u32, size as u32)
                    .dark_color(svg::Color("#000000"))
                    .light_color(svg::Color("#ffffff"))
                    .build()
            })
    };

    view! {
        <div class=format!("flex justify-center items-center {}", class)>
            {move || {
                if let Some(svg) = svg_data() {
                    view! {
                        <div
                            class="qr-code"
                            inner_html=svg
                        />
                    }.into_any()
                } else {
                    view! {
                        <div class="text-rk-text-secondary">
                            "Failed to generate QR code"
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}

/// QR Code component with custom styling for WalletConnect
#[component]
pub fn WalletConnectQrCode(
    #[prop(into)] uri: String,
    #[prop(default = 280)] size: usize,
) -> impl IntoView {
    view! {
        <div class="bg-white p-4 rounded-rk">
            <QrCode data=uri size=size />
            <p class="text-center text-sm text-gray-600 mt-3">
                "Scan with your wallet"
            </p>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_code_generation() {
        let test_data = "wc:test@1?relay-protocol=irn&symKey=test";
        let qr = QrCodeGen::new(test_data).unwrap();
        let svg = qr.render::<svg::Color>()
            .min_dimensions(100, 100)
            .build();

        assert!(svg.contains("svg"));
        assert!(svg.contains("rect"));
    }
}
