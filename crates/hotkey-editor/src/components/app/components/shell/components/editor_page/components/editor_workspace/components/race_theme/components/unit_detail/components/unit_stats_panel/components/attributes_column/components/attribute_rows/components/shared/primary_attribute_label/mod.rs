mod props;
mod style;

use dioxus::prelude::*;
pub use props::PrimaryAttributeLabelProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(PrimaryAttributeLabel);

/// An attribute row's label: gold, brightening to full gold when its row reports itself
/// the hero's primary attribute — a reaction to the row's own `data-primary` group.
#[component]
pub fn PrimaryAttributeLabel(props: PrimaryAttributeLabelProps) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}
