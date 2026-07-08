mod props;
mod style;

use dioxus::prelude::*;
pub use props::NormalUnitCardIdProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(NormalUnitCardId);

/// The muted (unselected) look of a unit card's database id: faint grey text.
/// Presentational — the dispatcher builds its props and renders it when the card is
/// not selected.
#[component]
pub fn NormalUnitCardId(props: NormalUnitCardIdProps) -> Element {
    let race_attribute = props.race_attribute;
    let text = props.text;
    rsx! {
        code {
            class: CLASS,
            "data-race": race_attribute,
            {text}
        }
    }
}
