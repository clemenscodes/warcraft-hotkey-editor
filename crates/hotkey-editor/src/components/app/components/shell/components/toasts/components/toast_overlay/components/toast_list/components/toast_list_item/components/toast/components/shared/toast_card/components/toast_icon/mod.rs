mod model;
mod view;

pub use view::ToastIconView;
mod style;

use dioxus::prelude::*;
use model::ToastIconModel;
use style::CLASS;
use tw_macro::assert_component;

/// The circular glyph at the leading edge of a toast. Its disc and stroke tint come
/// from `--toast-accent`, so the severity wrapper decides the colour.
#[component]
pub fn ToastIcon(props: ToastIconModel) -> Element {
    let icon = props.icon;
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: icon,
        }
    }
}

assert_component!(ToastIcon);
