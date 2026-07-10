mod logic;
mod props;
mod style;

use dioxus::prelude::*;
use logic::ToastClosePresentation;
pub use props::ToastCloseProps;
use style::CLASS;
use tw_macro::assert_component;

/// The dismiss control on a toast.
#[component]
pub fn ToastClose(props: ToastCloseProps) -> Element {
    let ToastClosePresentation { onclick } = ToastClosePresentation::from(&props);
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "aria-label": "close",
            onclick,
            "\u{00d7}"
        }
    }
}

assert_component!(ToastClose);
