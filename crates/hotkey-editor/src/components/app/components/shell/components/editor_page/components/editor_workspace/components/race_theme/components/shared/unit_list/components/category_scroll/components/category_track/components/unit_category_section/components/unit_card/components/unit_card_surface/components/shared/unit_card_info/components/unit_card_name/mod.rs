mod props;
mod style;

use dioxus::prelude::*;
use props::UnitCardNameProps;
use style::CLASS;
use tw_macro::assert_component;

/// The unit's display name inside a card. Its colour follows the card's state through
/// the `--name-color` the surface publishes — not a `data-selected` attribute.
#[component]
pub fn UnitCardName(props: UnitCardNameProps) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(UnitCardName);
