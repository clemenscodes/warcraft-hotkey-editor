mod props;
mod style;

use dioxus::prelude::*;
pub use props::SelectedUnitCardIdProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(SelectedUnitCardId);

/// The selected look of a unit card's database id: it takes the card's race accent
/// (chosen off the `data-race` attribute it renders) at reduced opacity.
/// Presentational — the dispatcher builds its props and renders it when the card is
/// selected.
#[component]
pub fn SelectedUnitCardId(props: SelectedUnitCardIdProps) -> Element {
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
