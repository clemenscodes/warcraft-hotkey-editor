use std::collections::HashMap;
use std::rc::Rc;

use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

use crate::components::command_grid::{CommandGridSection, CommandGridSectionProps};
use crate::grid_layout::GridLayout;
use crate::grid_slot::{DragFollower, DraggingSlot, DropTargetCell, GridSlotId};

#[component]
pub(crate) fn UnitCommandGrids(
    unit_id: String,
    command_card_slots: Rc<[GridSlotId]>,
    build_menu_slots: Option<Rc<[GridSlotId]>>,
    uprooted_menu_slots: Option<Rc<[GridSlotId]>>,
    research_menu_slots: Option<Rc<[GridSlotId]>>,
    loaded_keys: Signal<Option<CustomKeys>>,
    selected_slot: Signal<Option<GridSlotId>>,
    selected_from_research: Signal<bool>,
    selected_from_uprooted: Signal<bool>,
    tier_overrides: Signal<HashMap<String, usize>>,
    dragging_slot: Signal<Option<DraggingSlot>>,
    drop_target_cell: Signal<Option<DropTargetCell>>,
    drag_follower: Signal<Option<DragFollower>>,
    grid_layout: Signal<GridLayout>,
) -> Element {
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
