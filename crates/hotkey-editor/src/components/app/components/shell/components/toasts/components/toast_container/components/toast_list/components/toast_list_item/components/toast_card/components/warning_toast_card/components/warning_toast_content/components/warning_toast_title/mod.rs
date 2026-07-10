mod props;
mod style;

use dioxus::prelude::*;
pub use props::WarningToastTitleProps;
use style::CLASS;
use tw_macro::assert_component;

/// The warning toast headline: the uppercase gold heading look tinted for warning.
#[component]
pub fn WarningToastTitle(props: WarningToastTitleProps) -> Element {
    let title = props.title;
    rsx! {
        div {
            class: CLASS,
            {title}
        }
    }
}

assert_component!(WarningToastTitle);
