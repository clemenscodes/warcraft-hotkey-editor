mod props;

use dioxus::prelude::*;

use super::position_picker::{UpgradePositionPicker, UpgradePositionPickerProps};

pub use props::TileOverrideUpgradePickerProps;

/// Renders the upgraded-form position picker only when the ability has an upgraded
/// form; the picker's own dialog handles open/closed.
#[component]
pub fn TileOverrideUpgradePicker(props: TileOverrideUpgradePickerProps) -> Element {
    if !props.visible || props.upgrade_unit_id.is_none() {
        return rsx! {};
    }
    rsx! {
        UpgradePositionPicker { ..UpgradePositionPickerProps::from(&props) }
    }
}
