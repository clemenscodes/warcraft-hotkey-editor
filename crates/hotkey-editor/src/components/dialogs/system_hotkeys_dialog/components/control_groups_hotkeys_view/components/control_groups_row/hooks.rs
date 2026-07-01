use super::props::ControlGroupsRowProps;
use crate::components::dialogs::system_hotkeys_dialog::components::slot_button::SlotButtonProps;
use dioxus::prelude::*;
use warcraft_database::SystemHotkeysCategory;
use warcraft_keybinds::SystemBindingMap;

const SLOT_FRAME_GOLD: Asset = asset!("/assets/webui/widgets/listitems/list-item-focus-border.png");

/// The row's shaped setup: the gold-frame variable the container reads and the ten
/// finished control-group slots.
pub(super) struct ControlGroupsRowModel {
    pub(super) frame: String,
    pub(super) slots: Vec<SlotButtonProps>,
}

/// Builds the binding map, the gold-frame variable, and the ten slot buttons.
pub(super) fn use_control_groups_row(props: &ControlGroupsRowProps) -> ControlGroupsRowModel {
    let loaded_keys = props.loaded_keys;
    let editing_section = props.editing_section;
    let binding_map = use_memo(move || {
        let guard = loaded_keys.read();
        SystemBindingMap::build(guard.as_ref())
    });
    let binding_map_signal: ReadSignal<SystemBindingMap> = binding_map.into();
    let frame_url = SLOT_FRAME_GOLD;
    let frame = format!("--wc3-slot-frame: url('{frame_url}');");
    let entries = SystemHotkeysCategory::ControlGroups.entries();
    let slots = entries
        .iter()
        .enumerate()
        .map(|(slot_index, entry)| {
            let slot_label = format!("{}", slot_index + 1);
            let section_id = entry.section_id().to_string();
            let default_hotkey = entry.default_hotkey();
            let default_modifier = entry.default_modifier();
            SlotButtonProps {
                slot_label,
                section_id,
                default_hotkey,
                default_modifier,
                loaded_keys,
                editing_section,
                binding_map: binding_map_signal,
                compact: true,
            }
        })
        .collect();
    ControlGroupsRowModel { frame, slots }
}
