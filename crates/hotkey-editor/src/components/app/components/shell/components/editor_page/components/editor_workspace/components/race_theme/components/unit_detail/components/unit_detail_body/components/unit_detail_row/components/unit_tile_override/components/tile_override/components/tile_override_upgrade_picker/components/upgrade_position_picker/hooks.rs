use dioxus::prelude::*;
use std::collections::HashMap;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::shared::alt_position_picker_body::components::alt_position_picker_explainer::AltPositionPickerExplainerProps;
use super::props::UpgradePositionPickerProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::GridEditorConfig;
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::editor_state::context::use_editor_state;
use crate::services::grid_layout::context::use_grid_layout;

/// The upgraded-form picker's shaped view: the open signal, dialog title, explainer
/// copy, and the built grid config.
pub(super) struct UpgradePositionPickerModel {
    pub(super) open: Signal<bool>,
    pub(super) dialog_title: String,
    pub(super) explainer: AltPositionPickerExplainerProps,
    pub(super) grid_config: GridEditorConfig,
}

pub(super) fn use_upgrade_position_picker(
    props: &UpgradePositionPickerProps,
) -> UpgradePositionPickerModel {
    let loaded_keys = use_loaded_keys();
    let grid_layout = use_grid_layout();
    let editor = use_editor_state();
    let open = props.upgrade_position_picker_open;
    let explainer = AltPositionPickerExplainerProps::from(props);
    let upgrade_unit_id = props.upgrade_unit_id;
    let picker_selected_slot =
        use_signal::<Option<GridSlotId>>(move || Some(GridSlotId::ability(upgrade_unit_id)));
    let picker_selected_research = use_signal::<bool>(|| false);
    let picker_selected_uprooted = use_signal::<bool>(|| false);
    let picker_tier_overrides = use_signal::<HashMap<WarcraftObjectId, usize>>(HashMap::new);
    let update_hotkeys_on_move = use_signal(|| true);
    let hotkey_assign_request = use_signal(|| false);
    let dialog_title = format!("Position: {} (upgraded)", props.display_name);
    let restrict_draggable: Vec<GridSlotId> = vec![GridSlotId::ability(upgrade_unit_id)];
    let grid_config = GridEditorConfig {
        heading: "Upgraded-form position",
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
    UpgradePositionPickerModel {
        open,
        dialog_title,
        explainer,
        grid_config,
    }
}
