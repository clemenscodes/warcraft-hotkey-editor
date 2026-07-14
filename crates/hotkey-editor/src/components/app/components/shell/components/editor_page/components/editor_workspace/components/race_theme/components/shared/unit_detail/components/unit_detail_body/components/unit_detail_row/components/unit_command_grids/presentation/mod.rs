use super::data;
use super::model::UnitCommandGridsModel;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::shared::grid_editor::GridEditorView;
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::drag_state::DragState;
use crate::services::drag_state::context::use_drag_state;
use crate::services::editor_state::EditorState;
use crate::services::editor_state::context::use_editor_state;
use crate::services::grid_layout::context::use_grid_layout;
use dioxus::prelude::Signal;
use std::rc::Rc;
use warcraft_keybinds::{CustomKeys, GridLayout, GridSlotId};

pub(super) struct UnitCommandGridsPresentation {
    pub(super) command_card: GridEditorView,
    pub(super) build_menu: Option<GridEditorView>,
    pub(super) uprooted: Option<GridEditorView>,
    pub(super) research: Option<GridEditorView>,
}

impl UnitCommandGridsPresentation {
    pub(super) fn build(
        props: &UnitCommandGridsModel,
        loaded_keys: Signal<Option<CustomKeys>>,
        grid_layout: Signal<GridLayout>,
        editor: EditorState,
        drag_state: DragState,
    ) -> Self {
        let command_slots = props.command_card_slots.clone();
        let command_card = Self::config(
            props,
            loaded_keys,
            grid_layout,
            editor,
            drag_state,
            data::COMMAND_CARD,
            command_slots,
        );
        let build_menu = props.build_menu_slots.clone().map(|ids| {
            Self::config(
                props,
                loaded_keys,
                grid_layout,
                editor,
                drag_state,
                data::BUILD_MENU,
                ids,
            )
        });
        let uprooted = props.uprooted_menu_slots.clone().map(|ids| {
            Self::config(
                props,
                loaded_keys,
                grid_layout,
                editor,
                drag_state,
                data::UPROOTED,
                ids,
            )
        });
        let research = props.research_menu_slots.clone().map(|ids| {
            Self::config(
                props,
                loaded_keys,
                grid_layout,
                editor,
                drag_state,
                data::RESEARCH_MENU,
                ids,
            )
        });
        Self {
            command_card,
            build_menu,
            uprooted,
            research,
        }
    }

    fn config(
        props: &UnitCommandGridsModel,
        loaded_keys: Signal<Option<CustomKeys>>,
        grid_layout: Signal<GridLayout>,
        editor: EditorState,
        drag_state: DragState,
        heading: &'static str,
        slot_ids: Rc<[GridSlotId]>,
    ) -> GridEditorView {
        let host_unit_id = props.unit_id;
        GridEditorView {
            heading,
            slot_ids,
            loaded_keys,
            selected_slot: editor.selected_slot(),
            selected_from_research: editor.selected_from_research(),
            selected_from_uprooted: editor.selected_from_uprooted(),
            tier_overrides: editor.tier_overrides(),
            dragging_slot: drag_state.dragging_slot(),
            drop_target_tile: drag_state.drop_target_tile(),
            drag_follower: drag_state.drag_follower(),
            grid_layout,
            update_hotkeys_on_move: editor.update_hotkeys_on_move(),
            hotkey_assign_request: editor.hotkey_assign_request(),
            prevent_swap_on_drop: false,
            restrict_draggable_to: Vec::new(),
            host_unit_id,
        }
    }
}

pub(super) fn use_unit_command_grids(
    props: &UnitCommandGridsModel,
) -> UnitCommandGridsPresentation {
    let loaded_keys = use_loaded_keys();
    let grid_layout = use_grid_layout();
    let editor = use_editor_state();
    let drag_state = use_drag_state();
    UnitCommandGridsPresentation::build(props, loaded_keys, grid_layout, editor, drag_state)
}
