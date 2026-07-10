pub mod components;
mod hooks;
mod logic;
mod props;
mod state;

use dioxus::prelude::*;

use components::tile_override_alt_picker::TileOverrideAltPicker;
use components::tile_override_card::TileOverrideCard;
use components::tile_override_key_picker::TileOverrideKeyPicker;
use components::tile_override_upgrade_picker::TileOverrideUpgradePicker;
use hooks::use_tile_override;
use tw_macro::assert_component;

pub use props::TileOverrideProps;

/// The per-tile override editor. A pure renderer: the composed hook shapes the
/// card's whole nested props tree and the pickers, and the body only places them.
#[component]
pub fn TileOverride(props: TileOverrideProps) -> Element {
    let model = use_tile_override(&props);
    rsx! {
        TileOverrideCard { ..model.card }
        TileOverrideKeyPicker { ..model.key_picker }
        TileOverrideAltPicker { ..model.alt_picker }
        TileOverrideUpgradePicker { ..model.upgrade_picker }
    }
}

assert_component!(TileOverride);
