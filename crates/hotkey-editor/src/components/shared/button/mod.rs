mod props;
mod style;

use dioxus::prelude::*;

use props::ButtonPresentation;
use style::BUTTON_STYLES;

pub use props::{ButtonProps, ButtonVariant};

/// A WC3 action button in primary or secondary weight. A leaf: it owns the
/// `.button` class family and forwards one click. Callers pick the variant and
/// pass the label as children.
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let ButtonPresentation { class, onclick } = ButtonPresentation::from(&props);
    let label = props.children.clone();
    rsx! {
        document::Stylesheet { href: BUTTON_STYLES }
        button {
            class,
            r#type: "button",
            onclick,
            {label}
        }
    }
}
