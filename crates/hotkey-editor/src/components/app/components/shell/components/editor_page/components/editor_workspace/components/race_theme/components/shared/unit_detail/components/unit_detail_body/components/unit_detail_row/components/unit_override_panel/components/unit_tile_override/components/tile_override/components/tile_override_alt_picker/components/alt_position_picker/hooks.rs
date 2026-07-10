use dioxus::prelude::*;
use std::collections::HashMap;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

use super::props::AltPositionPickerProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::GridEditorView;
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::editor_state::context::use_editor_state;
use crate::services::grid_layout::context::use_grid_layout;

/// The off-state picker's shaped view: the open signal, the dialog title, the
/// explainer copy, and the fully-built grid editor config (the local picker signals
/// and the config assembly live here, so the body stays a single hook line plus RSX).
pub(super) struct AltPositionPickerModel {
    pub(super) open: Signal<bool>,
    pub(super) dialog_title: String,
    pub(super) explainer_text: String,
    pub(super) grid_config: GridEditorView,
}

pub(super) fn use_alt_position_picker(props: &AltPositionPickerProps) -> AltPositionPickerModel {
    let loaded_keys = use_loaded_keys();
    let grid_layout = use_grid_layout();
    let editor = use_editor_state();
    let open = props.alt_position_picker_open;
    let explainer_text = String::from(
        "Drag the off-state button to a different cell. Cells holding another ability are protected; drops on top of them are rejected so the unit's primary layout stays intact.",
    );
    let object_id = props.object_id;
    let picker_selected_slot =
        use_signal::<Option<GridSlotId>>(move || Some(GridSlotId::ability_off(object_id)));
    let picker_selected_research = use_signal::<bool>(|| false);
    let picker_selected_uprooted = use_signal::<bool>(|| false);
    let picker_tier_overrides = use_signal::<HashMap<WarcraftObjectId, usize>>(HashMap::new);
    let update_hotkeys_on_move = use_signal(|| true);
    let hotkey_assign_request = use_signal(|| false);
    let dialog_title = format!("Position: {}", props.display_name);
    let restrict_draggable: Vec<GridSlotId> = vec![GridSlotId::ability_off(object_id)];
    let grid_config = GridEditorView {
        heading: "Off-state position",
        slot_ids: props.picker_slots.clone(),
        loaded_keys,
        selected_slot: picker_selected_slot,
        selected_from_research: picker_selected_research,
        selected_from_uprooted: picker_selected_uprooted,
        tier_overrides: picker_tier_overrides,
        dragging_slot: editor.dragging_slot(),
        drop_target_tile: editor.drop_target_tile(),
        drag_follower: editor.drag_follower(),
        grid_layout,
        update_hotkeys_on_move,
        hotkey_assign_request,
        prevent_swap_on_drop: true,
        restrict_draggable_to: restrict_draggable,
        host_unit_id: WarcraftObjectId::default(),
    };
    AltPositionPickerModel {
        open,
        dialog_title,
        explainer_text,
        grid_config,
    }
}
