mod props;
mod style;

use dioxus::prelude::*;
pub use props::NormalUnitCardIdProps;
use style::CLASS;
use tw_macro::assert_component;

/// The muted (unselected) look of a unit card's database id: faint grey text.
/// Presentational — the dispatcher builds its props and renders it when the card is
/// not selected.
#[component]
pub fn NormalUnitCardId(props: NormalUnitCardIdProps) -> Element {
    rsx! {
        code {
            class: CLASS,
            {props.unit_id.value()}
        }
    }
}

assert_component!(NormalUnitCardId);
