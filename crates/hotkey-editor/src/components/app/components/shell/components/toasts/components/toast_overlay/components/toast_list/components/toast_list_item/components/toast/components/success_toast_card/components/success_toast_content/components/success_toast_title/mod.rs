mod model;
mod view;

pub use view::SuccessToastTitleView;
mod style;

use dioxus::prelude::*;
use model::SuccessToastTitleModel;
use style::CLASS;
use tw_macro::assert_component;

/// The success toast headline: the uppercase gold heading look tinted for success.
#[component]
pub fn SuccessToastTitle(props: SuccessToastTitleModel) -> Element {
    let title = props.title;
    rsx! {
        div {
            class: CLASS,
            {title}
        }
    }
}

assert_component!(SuccessToastTitle);
