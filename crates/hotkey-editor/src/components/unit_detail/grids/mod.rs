use std::collections::HashMap;
use std::rc::Rc;

use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

use crate::components::command_grid::{CommandGridSection, CommandGridSectionProps};
use crate::model::grid::GridLayout;
use crate::model::grid::{DragFollower, DraggingSlot, DropTargetCell, GridSlotId};

#[derive(Props, Clone, PartialEq)]
pub(crate) struct UnitCommandGridsProps {
    pub(crate) unit_id: String,
    pub(crate) command_card_slots: Rc<[GridSlotId]>,
    pub(crate) build_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(crate) uprooted_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(crate) research_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(crate) loaded_keys: Signal<Option<CustomKeys>>,
    pub(crate) selected_slot: Signal<Option<GridSlotId>>,
    pub(crate) selected_from_research: Signal<bool>,
    pub(crate) selected_from_uprooted: Signal<bool>,
    pub(crate) tier_overrides: Signal<HashMap<String, usize>>,
    pub(crate) dragging_slot: Signal<Option<DraggingSlot>>,
    pub(crate) drop_target_cell: Signal<Option<DropTargetCell>>,
    pub(crate) drag_follower: Signal<Option<DragFollower>>,
    pub(crate) grid_layout: Signal<GridLayout>,
}

#[component]
pub(crate) fn UnitCommandGrids(props: UnitCommandGridsProps) -> Element {
    let unit_id = props.unit_id;
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
    let drop_target_cell = props.drop_target_cell;
    let drag_follower = props.drag_follower;
    let grid_layout = props.grid_layout;
    let command_card_props = CommandGridSectionProps {
        heading: "Command card",
        slot_ids: command_card_slots,
        loaded_keys,
        selected_slot,
        selected_from_research,
        selected_from_uprooted,
        tier_overrides,
        dragging_slot,
        drop_target_cell,
        drag_follower,
        grid_layout,
        is_research_grid: false,
        is_uprooted_grid: false,
        prevent_swap_on_drop: false,
        restrict_draggable_to: Vec::new(),
        host_unit_id: unit_id.clone(),
    };
    rsx! {
        div { class: "unit-detail-grids",
            CommandGridSection { ..command_card_props }
            if let Some(build_menu_ids) = build_menu_slots {
                {
                    let build_menu_props = CommandGridSectionProps {
                        heading: "Build menu",
                        slot_ids: build_menu_ids,
                        loaded_keys,
                        selected_slot,
                        selected_from_research,
                        selected_from_uprooted,
                        tier_overrides,
                        dragging_slot,
                        drop_target_cell,
                        drag_follower,
                        grid_layout,
                        is_research_grid: false,
                        is_uprooted_grid: false,
                        prevent_swap_on_drop: false,
                        restrict_draggable_to: Vec::new(),
                        host_unit_id: unit_id.clone(),
                    };
                    rsx! { CommandGridSection { ..build_menu_props } }
                }
            }
            if let Some(uprooted_menu_ids) = uprooted_menu_slots {
                {
                    let uprooted_props = CommandGridSectionProps {
                        heading: "Uprooted",
                        slot_ids: uprooted_menu_ids,
                        loaded_keys,
                        selected_slot,
                        selected_from_research,
                        selected_from_uprooted,
                        tier_overrides,
                        dragging_slot,
                        drop_target_cell,
                        drag_follower,
                        grid_layout,
                        is_research_grid: false,
                        is_uprooted_grid: true,
                        prevent_swap_on_drop: false,
                        restrict_draggable_to: Vec::new(),
                        host_unit_id: unit_id.clone(),
                    };
                    rsx! { CommandGridSection { ..uprooted_props } }
                }
            }
            if let Some(research_menu_ids) = research_menu_slots {
                {
                    let research_props = CommandGridSectionProps {
                        heading: "Research menu",
                        slot_ids: research_menu_ids,
                        loaded_keys,
                        selected_slot,
                        selected_from_research,
                        selected_from_uprooted,
                        tier_overrides,
                        dragging_slot,
                        drop_target_cell,
                        drag_follower,
                        grid_layout,
                        is_research_grid: true,
                        is_uprooted_grid: false,
                        prevent_swap_on_drop: false,
                        restrict_draggable_to: Vec::new(),
                        host_unit_id: unit_id.clone(),
                    };
                    rsx! { CommandGridSection { ..research_props } }
                }
            }
        }
    }
}
