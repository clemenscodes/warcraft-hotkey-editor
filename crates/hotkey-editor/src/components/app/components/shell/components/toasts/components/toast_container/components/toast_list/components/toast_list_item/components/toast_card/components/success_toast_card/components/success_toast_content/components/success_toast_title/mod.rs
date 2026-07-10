mod props;
mod style;

use dioxus::prelude::*;
pub use props::SuccessToastTitleProps;
use style::CLASS;
use tw_macro::assert_component;

/// The success toast headline: the uppercase gold heading look tinted for success.
#[component]
pub fn SuccessToastTitle(props: SuccessToastTitleProps) -> Element {
    let title = props.title;
    rsx! {
        div {
            class: CLASS,
            {title}
        }
    }
}

assert_component!(SuccessToastTitle);
