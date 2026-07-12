mod model;
mod view;

pub use view::SelectedUnitCardIdView;
mod style;

use dioxus::prelude::*;
use model::SelectedUnitCardIdModel;
use style::CLASS;
use tw_macro::assert_component;

/// The selected look of a unit card's database id: it takes the card's race accent at
/// reduced opacity, read from the theme container's `--race-color`. Presentational —
/// the dispatcher builds its props and renders it when the card is selected.
#[component]
pub fn SelectedUnitCardId(props: SelectedUnitCardIdModel) -> Element {
    rsx! {
        code {
            class: CLASS,
            {props.unit_id.value()}
        }
    }
}

assert_component!(SelectedUnitCardId);
