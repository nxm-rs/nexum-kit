use leptos::prelude::*;

pub use super::box_component::{BoxDisplay, BoxFontWeight, BoxTextAlign};

#[component]
pub fn Text(
    children: Children,
    #[prop(optional)] as_element: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] color: Option<&'static str>,
    #[prop(optional)] font_weight: Option<BoxFontWeight>,
    #[prop(optional)] size: Option<&'static str>,
    #[prop(optional)] text_align: Option<BoxTextAlign>,
    #[prop(optional)] display: Option<BoxDisplay>,
    #[prop(optional)] additional_style: Option<&'static str>,
) -> impl IntoView {
    let tag = as_element.unwrap_or("div");

    let style = move || {
        let mut styles = Vec::new();

        if let Some(d) = &display {
            styles.push(format!("display: {};", d.as_str()));
        }
        if let Some(ta) = &text_align {
            styles.push(format!("text-align: {};", ta.as_str()));
        }
        if let Some(fw) = &font_weight {
            styles.push(format!("font-weight: {};", fw.as_str()));
        }
        if let Some(s) = size {
            styles.push(format!("font-size: {};", s));
        }
        if let Some(c) = color {
            styles.push(format!("color: var(--nk-colors-{}, {});", c, c));
        }
        if let Some(extra) = additional_style {
            styles.push(extra.to_string());
        }

        styles.join(" ")
    };

    let class_attr = class.unwrap_or("");

    // Create the element dynamically based on tag
    match tag {
        "div" => view! {
            <div class=class_attr style=style data-nk="">
                {children()}
            </div>
        }.into_any(),
        "span" => view! {
            <span class=class_attr style=style data-nk="">
                {children()}
            </span>
        }.into_any(),
        "p" => view! {
            <p class=class_attr style=style data-nk="">
                {children()}
            </p>
        }.into_any(),
        "h1" => view! {
            <h1 class=class_attr style=style data-nk="">
                {children()}
            </h1>
        }.into_any(),
        "h2" => view! {
            <h2 class=class_attr style=style data-nk="">
                {children()}
            </h2>
        }.into_any(),
        "h3" => view! {
            <h3 class=class_attr style=style data-nk="">
                {children()}
            </h3>
        }.into_any(),
        "h4" => view! {
            <h4 class=class_attr style=style data-nk="">
                {children()}
            </h4>
        }.into_any(),
        "h5" => view! {
            <h5 class=class_attr style=style data-nk="">
                {children()}
            </h5>
        }.into_any(),
        "h6" => view! {
            <h6 class=class_attr style=style data-nk="">
                {children()}
            </h6>
        }.into_any(),
        "label" => view! {
            <label class=class_attr style=style data-nk="">
                {children()}
            </label>
        }.into_any(),
        "code" => view! {
            <code class=class_attr style=style data-nk="">
                {children()}
            </code>
        }.into_any(),
        _ => view! {
            <div class=class_attr style=style data-nk="">
                {children()}
            </div>
        }.into_any(),
    }
}
