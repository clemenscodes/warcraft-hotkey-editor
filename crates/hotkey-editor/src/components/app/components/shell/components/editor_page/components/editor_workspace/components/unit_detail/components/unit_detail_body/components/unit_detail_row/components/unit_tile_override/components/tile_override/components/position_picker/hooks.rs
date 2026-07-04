use dioxus::prelude::*;
use std::collections::HashMap;
use warcraft_api::Race;
use warcraft_keybinds::GridSlotId;

use super::alt_position_picker_explainer::AltPositionPickerExplainerProps;
use super::props::AltPositionPickerProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::GridEditorConfig;

/// The off-state picker's shaped view: the open signal, the dialog title, the
/// explainer copy, and the fully-built grid editor config (the local picker signals
/// and the config assembly live here, so the body stays a single hook line plus RSX).
pub(super) struct AltPositionPickerModel {
    pub(super) open: Signal<bool>,
    pub(super) dialog_title: String,
    pub(super) explainer: AltPositionPickerExplainerProps,
    pub(super) grid_config: GridEditorConfig,
}

pub(super) fn use_alt_position_picker(props: &AltPositionPickerProps) -> AltPositionPickerModel {
    let open = props.alt_position_picker_open;
    let explainer = AltPositionPickerExplainerProps::from(props);
    let object_id = props.object_id;
    let picker_selected_slot =
        use_signal::<Option<GridSlotId>>(move || Some(GridSlotId::ability_off(object_id)));
    let picker_selected_research = use_signal::<bool>(|| false);
    let picker_selected_uprooted = use_signal::<bool>(|| false);
    let picker_tier_overrides = use_signal::<HashMap<String, usize>>(HashMap::new);
    let update_hotkeys_on_move = use_signal(|| true);
    let hotkey_assign_request = use_signal(|| false);
    let dialog_title = format!("Position: {}", props.display_name);
    let restrict_draggable: Vec<GridSlotId> = vec![GridSlotId::ability_off(object_id)];
    let grid_config = GridEditorConfig {
        heading: "Off-state position",
        race: Race::Neutral,
        slot_ids: props.picker_slots.clone(),
        loaded_keys: props.loaded_keys,
        selected_slot: picker_selected_slot,
        selected_from_research: picker_selected_research,
        selected_from_uprooted: picker_selected_uprooted,
        tier_overrides: picker_tier_overrides,
        dragging_slot: props.dragging_slot,
        drop_target_tile: props.drop_target_tile,
        drag_follower: props.drag_follower,
        grid_layout: props.grid_layout,
        update_hotkeys_on_move,
        hotkey_assign_request,
        prevent_swap_on_drop: true,
        restrict_draggable_to: restrict_draggable,
        host_unit_id: String::new(),
    };
    AltPositionPickerModel {
        open,
        dialog_title,
        explainer,
        grid_config,
    }
}
