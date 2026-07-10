pub mod components;
mod data;
mod props;
mod view;

pub use view::UnitTileOverrideView;

use components::tile_override::TileOverride;
use components::tile_override_empty::TileOverrideEmpty;
use dioxus::prelude::*;
use props::UnitTileOverrideProps;
use tw_macro::assert_component;

/// The override slot: the override card for the selected tile, or the empty prompt
/// when nothing is selected. A pure dispatcher with no class of its own.
#[component]
pub fn UnitTileOverride(props: UnitTileOverrideProps) -> Element {
    let UnitTileOverrideProps {
        detail,
        active_container_slots,
    } = props;
    let Some(detail) = detail else {
        let message = data::EMPTY_PROMPT.to_string();
        return rsx! {
            TileOverrideEmpty { message }
        };
    };
    rsx! {
        TileOverride { detail, active_container_slots }
    }
}

assert_component!(UnitTileOverride);
