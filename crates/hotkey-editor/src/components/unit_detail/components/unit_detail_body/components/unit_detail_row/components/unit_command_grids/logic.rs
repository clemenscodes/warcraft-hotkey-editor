use super::data;
use super::props::UnitCommandGridsProps;
use crate::components::grid_editors::grid_editor::GridEditorConfig;
use std::rc::Rc;
use warcraft_keybinds::GridSlotId;

/// The four finished grid-editor configs: the always-present command card, and the
/// build/uprooted/research menus when the unit has them.
pub(super) struct UnitCommandGridsModel {
    pub(super) command_card: GridEditorConfig,
    pub(super) build_menu: Option<GridEditorConfig>,
    pub(super) uprooted: Option<GridEditorConfig>,
    pub(super) research: Option<GridEditorConfig>,
}

impl From<&UnitCommandGridsProps> for UnitCommandGridsModel {
    fn from(props: &UnitCommandGridsProps) -> Self {
        let command_card = config(props, data::COMMAND_CARD, props.command_card_slots.clone());
        let build_menu = props
            .build_menu_slots
            .clone()
            .map(|ids| config(props, data::BUILD_MENU, ids));
        let uprooted = props
            .uprooted_menu_slots
            .clone()
            .map(|ids| config(props, data::UPROOTED, ids));
        let research = props
            .research_menu_slots
            .clone()
            .map(|ids| config(props, data::RESEARCH_MENU, ids));
        Self {
            command_card,
            build_menu,
            uprooted,
            research,
        }
    }
}

/// Builds one grid-editor config for the given menu, sharing the unit's editor
/// signals and behavior flags.
fn config(
    props: &UnitCommandGridsProps,
    heading: &'static str,
    slot_ids: Rc<[GridSlotId]>,
) -> GridEditorConfig {
    let race = props.race;
    let loaded_keys = props.loaded_keys;
    let selected_slot = props.selected_slot;
    let selected_from_research = props.selected_from_research;
    let selected_from_uprooted = props.selected_from_uprooted;
    let tier_overrides = props.tier_overrides;
    let dragging_slot = props.dragging_slot;
    let drop_target_tile = props.drop_target_tile;
    let drag_follower = props.drag_follower;
    let grid_layout = props.grid_layout;
    let update_hotkeys_on_move = props.update_hotkeys_on_move;
    let hotkey_assign_request = props.hotkey_assign_request;
    let host_unit_id = props.unit_id.clone();
    GridEditorConfig {
        heading,
        race,
        slot_ids,
        loaded_keys,
        selected_slot,
        selected_from_research,
        selected_from_uprooted,
        tier_overrides,
        dragging_slot,
        drop_target_tile,
        drag_follower,
        grid_layout,
        update_hotkeys_on_move,
        hotkey_assign_request,
        prevent_swap_on_drop: false,
        restrict_draggable_to: Vec::new(),
        host_unit_id,
    }
}
