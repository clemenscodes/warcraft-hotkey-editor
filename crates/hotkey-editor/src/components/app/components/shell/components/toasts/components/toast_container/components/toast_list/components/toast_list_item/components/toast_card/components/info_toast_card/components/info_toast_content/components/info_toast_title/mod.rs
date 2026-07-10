mod props;
mod view;

pub use view::InfoToastTitleView;
mod style;

use dioxus::prelude::*;
use props::InfoToastTitleProps;
use style::CLASS;
use tw_macro::assert_component;

/// The info toast headline: the uppercase gold heading look tinted for info.
#[component]
pub fn InfoToastTitle(props: InfoToastTitleProps) -> Element {
    let title = props.title;
    rsx! {
        div {
            class: CLASS,
            {title}
        }
    }
}

assert_component!(InfoToastTitle);
