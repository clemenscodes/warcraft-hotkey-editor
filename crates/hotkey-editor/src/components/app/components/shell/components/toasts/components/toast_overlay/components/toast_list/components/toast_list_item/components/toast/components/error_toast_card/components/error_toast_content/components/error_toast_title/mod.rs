mod model;
mod view;

pub use view::ErrorToastTitleView;
mod style;

use dioxus::prelude::*;
use model::ErrorToastTitleModel;
use style::CLASS;
use tw_macro::assert_component;

/// The error toast headline: the uppercase gold heading look tinted for error.
#[component]
pub fn ErrorToastTitle(props: ErrorToastTitleModel) -> Element {
    let title = props.title;
    rsx! {
        div {
            class: CLASS,
            {title}
        }
    }
}

assert_component!(ErrorToastTitle);
