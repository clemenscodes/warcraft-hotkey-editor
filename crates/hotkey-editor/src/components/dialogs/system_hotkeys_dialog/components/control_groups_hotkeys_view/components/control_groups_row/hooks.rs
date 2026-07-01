use dioxus::prelude::*;
use warcraft_keybinds::SystemBindingMap;

use super::props::ControlGroupsRowProps;

const SLOT_FRAME_GOLD: Asset = asset!("/assets/webui/widgets/listitems/list-item-focus-border.png");

/// The row's shaped setup: the gold-frame variable its slots read and the resolved
/// binding map.
pub(super) struct ControlGroupsRowModel {
    pub(super) frame: String,
    pub(super) binding_map: Memo<SystemBindingMap>,
}

/// Builds the binding map and the gold-frame variable for the row.
pub(super) fn use_control_groups_row(props: &ControlGroupsRowProps) -> ControlGroupsRowModel {
    let loaded_keys = props.loaded_keys;
    let binding_map = use_memo(move || {
        let guard = loaded_keys.read();
        SystemBindingMap::build(guard.as_ref())
    });
    let frame_url = SLOT_FRAME_GOLD;
    let frame = format!("--wc3-slot-frame: url('{frame_url}');");
    ControlGroupsRowModel { frame, binding_map }
}
