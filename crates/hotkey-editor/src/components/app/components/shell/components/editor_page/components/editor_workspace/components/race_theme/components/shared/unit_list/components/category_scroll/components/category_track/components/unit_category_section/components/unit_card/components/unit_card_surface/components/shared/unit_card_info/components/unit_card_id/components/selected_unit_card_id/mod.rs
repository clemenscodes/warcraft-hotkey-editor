mod props;
mod style;

use dioxus::prelude::*;
pub use props::SelectedUnitCardIdProps;
use style::CLASS;
use tw_macro::assert_component;

/// The selected look of a unit card's database id: it takes the card's race accent at
/// reduced opacity, read from the theme container's `--race-accent`. Presentational —
/// the dispatcher builds its props and renders it when the card is selected.
#[component]
pub fn SelectedUnitCardId(props: SelectedUnitCardIdProps) -> Element {
    rsx! {
        code {
            class: CLASS,
            {props.unit_id.value()}
        }
    }
}

assert_component!(SelectedUnitCardId);
