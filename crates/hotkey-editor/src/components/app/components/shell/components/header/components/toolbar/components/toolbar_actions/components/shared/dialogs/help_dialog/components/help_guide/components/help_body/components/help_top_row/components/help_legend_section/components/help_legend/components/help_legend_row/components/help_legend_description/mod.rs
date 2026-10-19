mod model;
mod view;

pub use view::HelpLegendDescriptionView;
mod style;

use dioxus::prelude::*;
use model::HelpLegendDescriptionModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HelpLegendDescription(props: HelpLegendDescriptionModel) -> Element {
    let description = props.description.clone();
    rsx! {
        span {
            class: CLASS,
            " {description}"
        }
    }
}

assert_component!(HelpLegendDescription);
