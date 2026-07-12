mod model;
mod view;

pub use view::UnitCardNameView;
mod style;

use dioxus::prelude::*;
use model::UnitCardNameModel;
use style::CLASS;
use tw_macro::assert_component;

/// The unit's display name inside a card. Its colour follows the card's state through
/// the `--name-color` the surface publishes — not a `data-selected` attribute.
#[component]
pub fn UnitCardName(props: UnitCardNameModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(UnitCardName);
