mod model;
mod view;

pub use view::WarningToastTitleView;
mod style;

use dioxus::prelude::*;
use model::WarningToastTitleModel;
use style::CLASS;
use tw_macro::assert_component;

/// The warning toast headline: the uppercase gold heading look tinted for warning.
#[component]
pub fn WarningToastTitle(props: WarningToastTitleModel) -> Element {
    let title = props.title;
    rsx! {
        div {
            class: CLASS,
            {title}
        }
    }
}

assert_component!(WarningToastTitle);
