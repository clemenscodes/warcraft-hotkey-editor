mod model;
mod view;

pub use view::InfoToastTitleView;
mod style;

use dioxus::prelude::*;
use model::InfoToastTitleModel;
use style::CLASS;
use tw_macro::assert_component;

/// The info toast headline: the uppercase gold heading look tinted for info.
#[component]
pub fn InfoToastTitle(props: InfoToastTitleModel) -> Element {
    let title = props.title;
    rsx! {
        div {
            class: CLASS,
            {title}
        }
    }
}

assert_component!(InfoToastTitle);
