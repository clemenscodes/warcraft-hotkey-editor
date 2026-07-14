mod model;
mod presentation;
mod style;
mod view;

pub use view::ToastCloseView;

use dioxus::prelude::*;
use model::ToastCloseModel;
use presentation::ToastClosePresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ToastClose(props: ToastCloseModel) -> Element {
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
