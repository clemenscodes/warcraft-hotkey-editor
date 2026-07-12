mod model;
mod view;

pub use view::HelpLegendDescriptionView;
mod style;

use dioxus::prelude::*;
use model::HelpLegendDescriptionModel;
use style::CLASS;
use tw_macro::assert_component;

/// The toolbar button's one-line description in a legend row. A leaf: the row
/// passes the copy, and the leaf renders it after the label with a leading gap.
#[component]
pub fn HelpLegendDescription(props: HelpLegendDescriptionModel) -> Element {
    let description = props.description.clone();
    rsx! {
        span { class: CLASS, " {description}" }
    }
}

assert_component!(HelpLegendDescription);
