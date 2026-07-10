use super::data;
use super::props::UnitCommandGridsProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::GridEditorView;
use crate::services::editor_state::EditorState;
use dioxus::prelude::Signal;
use std::rc::Rc;
use warcraft_keybinds::{CustomKeys, GridLayout, GridSlotId};

/// The four finished grid-editor configs: the always-present command card, and the
/// build/uprooted/research menus when the unit has them.
pub(super) struct UnitCommandGridsModel {
    pub(super) command_card: GridEditorView,
    pub(super) build_menu: Option<GridEditorView>,
    pub(super) uprooted: Option<GridEditorView>,
    pub(super) research: Option<GridEditorView>,
}

impl UnitCommandGridsModel {
    /// Shapes the four configs from the unit's per-menu identity (props) plus the
    /// shared editor signals, which the component's hook sources from context.
    pub(super) fn build(
        props: &UnitCommandGridsProps,
        loaded_keys: Signal<Option<CustomKeys>>,
        grid_layout: Signal<GridLayout>,
        editor: EditorState,
    ) -> Self {
        let command_slots = props.command_card_slots.clone();
        let command_card = Self::config(
            props,
            loaded_keys,
            grid_layout,
            editor,
            data::COMMAND_CARD,
            command_slots,
        );
        let build_menu = props.build_menu_slots.clone().map(|ids| {
            Self::config(
                props,
                loaded_keys,
                grid_layout,
                editor,
                data::BUILD_MENU,
                ids,
            )
        });
        let uprooted = props
            .uprooted_menu_slots
            .clone()
            .map(|ids| Self::config(props, loaded_keys, grid_layout, editor, data::UPROOTED, ids));
        let research = props.research_menu_slots.clone().map(|ids| {
            Self::config(
                props,
                loaded_keys,
                grid_layout,
                editor,
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

    /// Builds one grid-editor config for the given menu, sharing the unit's editor
    /// signals and behavior flags.
    fn config(
        props: &UnitCommandGridsProps,
        loaded_keys: Signal<Option<CustomKeys>>,
        grid_layout: Signal<GridLayout>,
        editor: EditorState,
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
            dragging_slot: editor.dragging_slot(),
            drop_target_tile: editor.drop_target_tile(),
            drag_follower: editor.drag_follower(),
            grid_layout,
            update_hotkeys_on_move: editor.update_hotkeys_on_move(),
            hotkey_assign_request: editor.hotkey_assign_request(),
            prevent_swap_on_drop: false,
            restrict_draggable_to: Vec::new(),
            host_unit_id,
        }
    }
}
