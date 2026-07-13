mod model;
mod view;

pub use view::HelpLegendLabelView;
mod style;

use dioxus::prelude::*;
use model::HelpLegendLabelModel;
use style::CLASS;
use tw_macro::assert_component;

/// The toolbar button's name in a legend row. A leaf: the row passes the name.
#[component]
pub fn HelpLegendLabel(props: HelpLegendLabelModel) -> Element {
    let label = props.label.clone();
    rsx! {
        span { class: CLASS, {label} }
    }
}

assert_component!(HelpLegendLabel);
