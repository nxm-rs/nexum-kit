use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum BoxDisplay {
    Block,
    Flex,
    InlineBlock,
    Inline,
    InlineFlex,
    None,
}

impl BoxDisplay {
    pub fn as_str(&self) -> &'static str {
        match self {
            BoxDisplay::Block => "block",
            BoxDisplay::Flex => "flex",
            BoxDisplay::InlineBlock => "inline-block",
            BoxDisplay::Inline => "inline",
            BoxDisplay::InlineFlex => "inline-flex",
            BoxDisplay::None => "none",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BoxTextAlign {
    Left,
    Center,
    Right,
    Inherit,
}

impl BoxTextAlign {
    pub fn as_str(&self) -> &'static str {
        match self {
            BoxTextAlign::Left => "left",
            BoxTextAlign::Center => "center",
            BoxTextAlign::Right => "right",
            BoxTextAlign::Inherit => "inherit",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BoxFontWeight {
    Regular,
    Medium,
    Semibold,
    Bold,
}

impl BoxFontWeight {
    pub fn as_str(&self) -> &'static str {
        match self {
            BoxFontWeight::Regular => "400",
            BoxFontWeight::Medium => "500",
            BoxFontWeight::Semibold => "600",
            BoxFontWeight::Bold => "700",
        }
    }
}

#[component]
pub fn Box(
    #[prop(optional, into)] children: Option<Children>,
    #[prop(optional)] as_element: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] display: Option<BoxDisplay>,
    #[prop(optional)] text_align: Option<BoxTextAlign>,
    #[prop(optional)] font_weight: Option<BoxFontWeight>,
    #[prop(optional)] font_size: Option<&'static str>,
    #[prop(optional)] color: Option<&'static str>,
    #[prop(optional)] padding: Option<&'static str>,
    #[prop(optional)] margin: Option<&'static str>,
    #[prop(optional)] width: Option<&'static str>,
    #[prop(optional)] height: Option<&'static str>,
    #[prop(optional)] background: Option<&'static str>,
    #[prop(optional)] border_radius: Option<&'static str>,
    #[prop(optional)] gap: Option<&'static str>,
    #[prop(optional)] align_items: Option<&'static str>,
    #[prop(optional)] justify_content: Option<&'static str>,
    #[prop(optional)] flex_direction: Option<&'static str>,
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
        if let Some(fs) = font_size {
            styles.push(format!("font-size: {};", fs));
        }
        if let Some(c) = color {
            styles.push(format!("color: var(--nk-colors-{}, {});", c, c));
        }
        if let Some(p) = padding {
            styles.push(format!("padding: {};", p));
        }
        if let Some(m) = margin {
            styles.push(format!("margin: {};", m));
        }
        if let Some(w) = width {
            styles.push(format!("width: {};", w));
        }
        if let Some(h) = height {
            styles.push(format!("height: {};", h));
        }
        if let Some(bg) = background {
            styles.push(format!("background: {};", bg));
        }
        if let Some(br) = border_radius {
            styles.push(format!("border-radius: {};", br));
        }
        if let Some(g) = gap {
            styles.push(format!("gap: {};", g));
        }
        if let Some(ai) = align_items {
            styles.push(format!("align-items: {};", ai));
        }
        if let Some(jc) = justify_content {
            styles.push(format!("justify-content: {};", jc));
        }
        if let Some(fd) = flex_direction {
            styles.push(format!("flex-direction: {};", fd));
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
                {children.map(|c| c())}
            </div>
        }.into_any(),
        "span" => view! {
            <span class=class_attr style=style data-nk="">
                {children.map(|c| c())}
            </span>
        }.into_any(),
        "p" => view! {
            <p class=class_attr style=style data-nk="">
                {children.map(|c| c())}
            </p>
        }.into_any(),
        "h1" => view! {
            <h1 class=class_attr style=style data-nk="">
                {children.map(|c| c())}
            </h1>
        }.into_any(),
        "h2" => view! {
            <h2 class=class_attr style=style data-nk="">
                {children.map(|c| c())}
            </h2>
        }.into_any(),
        "h3" => view! {
            <h3 class=class_attr style=style data-nk="">
                {children.map(|c| c())}
            </h3>
        }.into_any(),
        "h4" => view! {
            <h4 class=class_attr style=style data-nk="">
                {children.map(|c| c())}
            </h4>
        }.into_any(),
        "h5" => view! {
            <h5 class=class_attr style=style data-nk="">
                {children.map(|c| c())}
            </h5>
        }.into_any(),
        "h6" => view! {
            <h6 class=class_attr style=style data-nk="">
                {children.map(|c| c())}
            </h6>
        }.into_any(),
        "label" => view! {
            <label class=class_attr style=style data-nk="">
                {children.map(|c| c())}
            </label>
        }.into_any(),
        "code" => view! {
            <code class=class_attr style=style data-nk="">
                {children.map(|c| c())}
            </code>
        }.into_any(),
        _ => view! {
            <div class=class_attr style=style data-nk="">
                {children.map(|c| c())}
            </div>
        }.into_any(),
    }
}
