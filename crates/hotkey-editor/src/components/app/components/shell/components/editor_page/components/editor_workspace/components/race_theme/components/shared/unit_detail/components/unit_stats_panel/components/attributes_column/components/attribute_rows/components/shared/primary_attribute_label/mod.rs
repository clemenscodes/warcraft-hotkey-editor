mod props;
mod view;

pub use view::PrimaryAttributeLabelView;
mod style;

use dioxus::prelude::*;
use props::PrimaryAttributeLabelProps;
use style::CLASS;
use tw_macro::assert_component;

/// An attribute row's label: gold, reading its exact tint from the `--attribute-label-color`
/// its row publishes (full gold when the row is the hero's primary attribute, dimmer otherwise).
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

assert_component!(PrimaryAttributeLabel);
