mod logic;
mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
use logic::ToastTitlePresentation;
pub use props::ToastTitleProps;
assert_component!(ToastTitle);

/// The toast headline, tinted by type.
#[component]
pub fn ToastTitle(props: ToastTitleProps) -> Element {
    let ToastTitlePresentation { class, title } = ToastTitlePresentation::from(&props);
    rsx! {
        div {
            class,
            {title}
        }
    }
}
