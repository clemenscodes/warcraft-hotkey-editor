use super::model::HotkeyUpgradePositionPickerDialogModel;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::shared::grid_editor::GridEditorView;
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::editor_state::context::use_editor_state;
use crate::services::grid_layout::context::use_grid_layout;
use dioxus::prelude::*;
use std::collections::HashMap;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

/// The upgraded-form picker's own shell, shaped from its model: the open value driving
/// the backdrop, the change handler that writes the open signal, and the domain values
/// the bordered panel places into its header row and scroll-region grid body. Every
/// dialog owns its shell now — there is no base.
pub(super) struct HotkeyUpgradePositionPickerDialogShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) title: String,
    pub(super) on_close: EventHandler<()>,
    pub(super) explainer_text: String,
    pub(super) grid_config: GridEditorView,
}

impl From<&HotkeyUpgradePositionPickerDialogPresentation>
    for HotkeyUpgradePositionPickerDialogShell
{
    fn from(model: &HotkeyUpgradePositionPickerDialogPresentation) -> Self {
        let mut open_signal = model.open;
        let open = open_signal();
        let on_open_change = Callback::new(move |is_open| open_signal.set(is_open));
        let mut close_signal = model.open;
        let title = model.dialog_title.clone();
        let on_close = EventHandler::new(move |()| close_signal.set(false));
        let explainer_text = model.explainer_text.clone();
        let grid_config = model.grid_config.clone();
        Self {
            open,
            on_open_change,
            title,
            on_close,
            explainer_text,
            grid_config,
        }
    }
}

/// The upgraded-form picker's shaped view: the open signal, dialog title, explainer
/// copy, and the built grid config.
pub(super) struct HotkeyUpgradePositionPickerDialogPresentation {
    pub(super) open: Signal<bool>,
    pub(super) dialog_title: String,
    pub(super) explainer_text: String,
    pub(super) grid_config: GridEditorView,
}

pub(super) fn use_hotkey_upgrade_position_picker(
    props: &HotkeyUpgradePositionPickerDialogModel,
) -> HotkeyUpgradePositionPickerDialogPresentation {
    let loaded_keys = use_loaded_keys();
    let grid_layout = use_grid_layout();
    let editor = use_editor_state();
    let open = props.hotkey_upgrade_position_picker_open;
    let explainer_text = String::from(
        "Drag the upgraded-form button to a different cell. Cells holding another ability are protected; drops on top of them are rejected so the unit's primary layout stays intact.",
    );
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
    let grid_config = GridEditorView {
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
    HotkeyUpgradePositionPickerDialogPresentation {
        open,
        dialog_title,
        explainer_text,
        grid_config,
    }
}
