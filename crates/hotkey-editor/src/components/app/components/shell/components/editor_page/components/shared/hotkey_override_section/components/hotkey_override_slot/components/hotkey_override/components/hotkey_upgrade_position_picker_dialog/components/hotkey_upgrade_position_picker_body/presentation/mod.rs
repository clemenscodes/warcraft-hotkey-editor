use super::model::HotkeyUpgradePositionPickerBodyModel;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::shared::grid_editor::GridEditorView;
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::drag_state::context::use_drag_state;
use crate::services::grid_layout::context::use_grid_layout;
use dioxus::prelude::*;
use std::collections::HashMap;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

pub(super) struct HotkeyUpgradePositionPickerBodyPresentation {
    pub(super) explainer_text: String,
    pub(super) grid_config: GridEditorView,
}

pub(super) fn use_hotkey_upgrade_position_picker_body(
    props: &HotkeyUpgradePositionPickerBodyModel,
) -> HotkeyUpgradePositionPickerBodyPresentation {
    let loaded_keys = use_loaded_keys();
    let grid_layout = use_grid_layout();
    let drag_state = use_drag_state();
    let explainer_text = String::from(
        "Drag the upgraded-form button to a different cell. Cells holding another ability are protected. Drops on top of them are rejected so the unit's primary layout stays intact.",
    );
    let upgrade_unit_id = props.upgrade_unit_id;
    let picker_selected_slot =
        use_signal::<Option<GridSlotId>>(move || Some(GridSlotId::ability(upgrade_unit_id)));
    // The picker is a single self-contained grid; matching `host_unit_id` keeps
    // the highlight gate satisfied.
    let picker_selected_unit =
        use_signal::<Option<WarcraftObjectId>>(|| Some(WarcraftObjectId::default()));
    let picker_selected_research = use_signal::<bool>(|| false);
    let picker_selected_uprooted = use_signal::<bool>(|| false);
    let picker_tier_overrides = use_signal::<HashMap<WarcraftObjectId, usize>>(HashMap::new);
    let update_hotkeys_on_move = use_signal(|| true);
    let hotkey_assign_request = use_signal(|| false);
    let restrict_draggable: Vec<GridSlotId> = vec![GridSlotId::ability(upgrade_unit_id)];
    let grid_config = GridEditorView {
        heading: "Upgraded-form position",
        slot_ids: props.picker_slots.clone(),
        loaded_keys,
        selected_slot: picker_selected_slot,
        selected_unit: picker_selected_unit,
        selected_from_research: picker_selected_research,
        selected_from_uprooted: picker_selected_uprooted,
        tier_overrides: picker_tier_overrides,
        dragging_slot: drag_state.dragging_slot(),
        drop_target_tile: drag_state.drop_target_tile(),
        drag_follower: drag_state.drag_follower(),
        grid_layout,
        update_hotkeys_on_move,
        hotkey_assign_request,
        prevent_swap_on_drop: true,
        restrict_draggable_to: restrict_draggable,
        host_unit_id: WarcraftObjectId::default(),
    };
    HotkeyUpgradePositionPickerBodyPresentation {
        explainer_text,
        grid_config,
    }
}
