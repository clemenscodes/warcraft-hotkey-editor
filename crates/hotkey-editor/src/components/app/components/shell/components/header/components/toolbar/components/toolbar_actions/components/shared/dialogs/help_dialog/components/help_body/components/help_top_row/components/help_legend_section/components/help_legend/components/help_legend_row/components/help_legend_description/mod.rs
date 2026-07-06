mod props;
mod style;

use dioxus::prelude::*;
pub use props::HelpLegendDescriptionProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HelpLegendDescription);

/// The toolbar button's one-line description in a legend row. A leaf: the row
/// passes the copy as children.
#[component]
pub fn HelpLegendDescription(props: HelpLegendDescriptionProps) -> Element {
    let description = props.children.clone();
    rsx! {
        span { class: CLASS, {description} }
    }
}
