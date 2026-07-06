mod props;
mod style;

use dioxus::prelude::*;
pub use props::UnitCardNameProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnitCardName);

/// The unit's display name inside a card.
#[component]
pub fn UnitCardName(props: UnitCardNameProps) -> Element {
    let text = props.text;
    let is_selected = props.is_selected;
    rsx! {
        span { class: CLASS, "data-selected": is_selected, {text} }
    }
}
