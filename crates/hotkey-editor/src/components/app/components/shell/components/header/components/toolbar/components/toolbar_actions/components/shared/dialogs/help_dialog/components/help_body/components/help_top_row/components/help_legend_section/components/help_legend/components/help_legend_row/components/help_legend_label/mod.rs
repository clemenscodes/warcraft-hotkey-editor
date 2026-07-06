mod props;
mod style;

use dioxus::prelude::*;
pub use props::HelpLegendLabelProps;
use style::CLASS;
use tw_macro::assert_component;
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
