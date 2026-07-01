use crate::components::grid_editors::command_grid_editor::CommandGridEditor;
use crate::components::grid_editors::grid_editor::GridEditorConfig;
use crate::components::grid_editors::research_grid_editor::ResearchGridEditor;
use crate::components::grid_editors::uprooted_grid_editor::UprootedGridEditor;
use crate::model::grid::{DragFollower, DraggingSlot, DropTargetTile};
use dioxus::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use warcraft_api::Race;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::GridLayout;
use warcraft_keybinds::GridSlotId;

#[derive(Props, Clone, PartialEq)]
pub struct UnitCommandGridsProps {
    pub unit_id: String,
    pub race: Race,
    pub command_card_slots: Rc<[GridSlotId]>,
    pub build_menu_slots: Option<Rc<[GridSlotId]>>,
    pub uprooted_menu_slots: Option<Rc<[GridSlotId]>>,
    pub research_menu_slots: Option<Rc<[GridSlotId]>>,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
    pub selected_from_research: Signal<bool>,
    pub selected_from_uprooted: Signal<bool>,
    pub tier_overrides: Signal<HashMap<String, usize>>,
    pub dragging_slot: Signal<Option<DraggingSlot>>,
    pub drop_target_tile: Signal<Option<DropTargetTile>>,
    pub drag_follower: Signal<Option<DragFollower>>,
    pub grid_layout: Signal<GridLayout>,
    pub update_hotkeys_on_move: Signal<bool>,
    pub hotkey_assign_request: Signal<bool>,
}

#[component]
pub fn UnitCommandGrids(props: UnitCommandGridsProps) -> Element {
    let unit_id = props.unit_id;
    let race = props.race;
    let command_card_slots = props.command_card_slots;
    let build_menu_slots = props.build_menu_slots;
    let uprooted_menu_slots = props.uprooted_menu_slots;
    let research_menu_slots = props.research_menu_slots;
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
    let command_card_props = GridEditorConfig {
        heading: "Command card",
        race,
        slot_ids: command_card_slots,
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
        host_unit_id: unit_id.clone(),
    };
    rsx! {
        div { class: "unit-detail-grids",
            CommandGridEditor { ..command_card_props }
            if let Some(build_menu_ids) = build_menu_slots {
                {
                    let build_menu_props = GridEditorConfig {
                        heading: "Build menu",
                        race,
                        slot_ids: build_menu_ids,
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
                        host_unit_id: unit_id.clone(),
                    };
                    rsx! {
                        CommandGridEditor { ..build_menu_props }
                    }
                }
            }
            if let Some(uprooted_menu_ids) = uprooted_menu_slots {
                {
                    let uprooted_props = GridEditorConfig {
                        heading: "Uprooted",
                        race,
                        slot_ids: uprooted_menu_ids,
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
                        host_unit_id: unit_id.clone(),
                    };
                    rsx! {
                        UprootedGridEditor { ..uprooted_props }
                    }
                }
            }
            if let Some(research_menu_ids) = research_menu_slots {
                {
                    let research_props = GridEditorConfig {
                        heading: "Research menu",
                        race,
                        slot_ids: research_menu_ids,
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
                        host_unit_id: unit_id.clone(),
                    };
                    rsx! {
                        ResearchGridEditor { ..research_props }
                    }
                }
            }
        }
    }
}
