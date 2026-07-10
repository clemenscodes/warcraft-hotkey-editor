pub mod components;
mod props;

use dioxus::prelude::*;

use components::upgrade_position_picker::{UpgradePositionPicker, UpgradePositionPickerProps};
use tw_macro::assert_component;

pub use props::TileOverrideUpgradePickerProps;

/// Renders the upgraded-form position picker only when the ability has an upgraded
/// form; the picker's own dialog handles open/closed.
#[component]
pub fn TileOverrideUpgradePicker(props: TileOverrideUpgradePickerProps) -> Element {
    if !*props.upgrade_position_picker_open.read() || props.upgrade_unit_id.is_none() {
        return rsx! {};
    }
    rsx! {
        UpgradePositionPicker { ..UpgradePositionPickerProps::from(&props) }
    }
}

assert_component!(TileOverrideUpgradePicker);
