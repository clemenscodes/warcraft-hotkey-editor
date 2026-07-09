pub mod components;
mod hooks;
mod props;
mod style;

use components::unit_card_surface::UnitCardSurface;
use dioxus::prelude::*;
use hooks::use_unit_card;
pub use props::UnitCardProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnitCard);

/// One selectable unit in the list: portrait plus name and id. Selecting it drives
/// the unit-detail panel. A thin identity wrapper that owns the card's placement box
/// and per-kind carousel filter and nests its own `UnitCardSurface` button for the
/// look, the generic `--race-color` accent, and the select handlers. Its selected look
/// and select handlers are shaped from context by `use_unit_card`.
#[component]
pub fn UnitCard(props: UnitCardProps) -> Element {
    let model = use_unit_card(&props);
    rsx! {
        div {
            class: CLASS,
            "data-unit-kind": model.kind_attr,
            UnitCardSurface { ..model.surface }
        }
    }
}
