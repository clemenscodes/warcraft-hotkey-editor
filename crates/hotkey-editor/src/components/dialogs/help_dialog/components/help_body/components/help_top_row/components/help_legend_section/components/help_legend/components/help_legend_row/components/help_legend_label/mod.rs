mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::HelpLegendLabelProps;
use style::CLASS;
assert_component!(HelpLegendLabel);

/// The toolbar button's name in a legend row. A leaf: the row passes the name as
/// children.
#[component]
pub fn HelpLegendLabel(props: HelpLegendLabelProps) -> Element {
    let label = props.children.clone();
    rsx! {
        span { class: CLASS, {label} }
    }
}
