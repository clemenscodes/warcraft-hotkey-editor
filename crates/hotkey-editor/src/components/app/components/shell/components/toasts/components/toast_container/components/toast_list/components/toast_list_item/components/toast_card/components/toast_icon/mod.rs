mod data;
mod logic;
mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
use logic::ToastIconPresentation;
pub use props::ToastIconProps;
assert_component!(ToastIcon);

/// The circular type glyph at the leading edge of a toast.
#[component]
pub fn ToastIcon(props: ToastIconProps) -> Element {
    let ToastIconPresentation { class, svg } = ToastIconPresentation::from(&props);
    rsx! {
        span {
            class,
            aria_hidden: "true",
            dangerous_inner_html: svg,
        }
    }
}
