mod props;
mod style;

use dioxus::prelude::*;
pub use props::ErrorToastTitleProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ErrorToastTitle);

/// The error toast headline: the uppercase gold heading look tinted for error.
#[component]
pub fn ErrorToastTitle(props: ErrorToastTitleProps) -> Element {
    let title = props.title;
    rsx! {
        div {
            class: CLASS,
            {title}
        }
    }
}
