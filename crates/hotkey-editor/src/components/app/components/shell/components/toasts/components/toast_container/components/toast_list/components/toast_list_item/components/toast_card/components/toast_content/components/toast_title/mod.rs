mod logic;
mod props;
mod style;

use dioxus::prelude::*;
use logic::ToastTitlePresentation;
pub use props::ToastTitleProps;
use tw_macro::assert_component;
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
