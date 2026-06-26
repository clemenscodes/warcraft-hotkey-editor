use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::{
    CommandGridSection, DragFollower, DraggingSlot, DropTargetCell, GridCell, GridCellProps,
    GridTile, GridTileProps, ToastMount,
};
use warcraft_database::WARCRAFT_DATABASE;
use warcraft_keybinds::{CustomKeys, GridLayout, GridSlotId, UnitCommandSlots};

use super::fixtures;

pub fn stories() -> Vec<Story> {
    vec![
        Story::new("Command grid", "Grid cell — empty", grid_cell_empty),
        Story::new(
            "Command grid",
            "Grid tile — Footman column 0 row 0",
            grid_tile_footman,
        ),
        Story::new(
            "Command grid",
            "Command grid section — Footman",
            command_grid_section_footman,
        ),
    ]
}

fn grid_cell_empty() -> Element {
    let class_name = "grid-tile".to_string();
    let column: u8 = 0;
    let row: u8 = 0;
    let heading_text: &'static str = "Main Commands";
    let icon_src_option = None;
    let label_text = String::new();
    let displayed_letter: Option<String> = Some("Q".to_string());
    let hotkey_overlay_class: &'static str = "hotkey-overlay";
    let is_focusable = false;
    let tile_is_draggable = false;
    let is_research_grid = false;
    let is_uprooted_grid = false;
    let is_passive_on_command_grid = false;
    let is_command_cell = false;
    let prevent_swap_on_drop = false;
    let layout_snapshot = GridLayout::qwerty_grid();
    let empty_restrictions: Vec<GridSlotId> = Vec::new();
    let restrict_draggable_to: Rc<[GridSlotId]> = empty_restrictions.into();
    let selected_slot = use_signal(|| None::<GridSlotId>);
    let selected_from_research = use_signal(|| false);
    let selected_from_uprooted = use_signal(|| false);
    let dragging_slot: Signal<Option<DraggingSlot>> = use_signal(|| None);
    let drop_target_cell: Signal<Option<DropTargetCell>> = use_signal(|| None);
    let drag_follower: Signal<Option<DragFollower>> = use_signal(|| None);
    let keys_signal: Signal<Option<CustomKeys>> = use_signal(|| None);
    let empty_drop_ids: Vec<GridSlotId> = Vec::new();
    let slot_ids_for_drop: Rc<[GridSlotId]> = empty_drop_ids.into();
    let occupant_slot: Option<GridSlotId> = None;
    let update_hotkeys_on_move = use_signal(|| true);
    let hotkey_assign_request = use_signal(|| false);
    let cell_props = GridCellProps {
        class_name,
        column,
        row,
        heading_text,
        icon_src_option,
        label_text,
        displayed_letter,
        hotkey_overlay_class,
        is_focusable,
        tile_is_draggable,
        is_research_grid,
        is_uprooted_grid,
        is_passive_on_command_grid,
        is_command_cell,
        prevent_swap_on_drop,
        layout_snapshot,
        restrict_draggable_to,
        selected_slot,
        selected_from_research,
        selected_from_uprooted,
        dragging_slot,
        drop_target_cell,
        drag_follower,
        keys_signal,
        slot_ids_for_drop,
        occupant_slot,
        update_hotkeys_on_move,
        hotkey_assign_request,
    };
    rsx! {
        ToastMount {
            GridCell { ..cell_props }
        }
    }
}

fn footman_command_slots() -> Rc<[GridSlotId]> {
    let unit_id = fixtures::sample_unit_id();
    WARCRAFT_DATABASE
        .by_id_and_key(&unit_id)
        .map(|(object_id, _)| {
            WARCRAFT_DATABASE
                .command_card(object_id)
                .filled_slots()
                .collect::<Rc<[GridSlotId]>>()
        })
        .unwrap_or_else(|| Rc::from(Vec::<GridSlotId>::new()))
}

fn grid_tile_footman() -> Element {
    let slot_ids = footman_command_slots();
    let column: u8 = 0;
    let row: u8 = 0;
    let heading: &'static str = "Main Commands";
    let loaded_keys = use_signal(|| None::<CustomKeys>);
    let selected_slot = use_signal(|| None::<GridSlotId>);
    let selected_from_research = use_signal(|| false);
    let selected_from_uprooted = use_signal(|| false);
    let tier_overrides: Signal<HashMap<String, usize>> = use_signal(HashMap::new);
    let dragging_slot: Signal<Option<DraggingSlot>> = use_signal(|| None);
    let drop_target_cell: Signal<Option<DropTargetCell>> = use_signal(|| None);
    let drag_follower: Signal<Option<DragFollower>> = use_signal(|| None);
    let grid_layout = use_signal(fixtures::sample_grid_layout);
    let empty_conflicts: HashSet<String> = HashSet::new();
    let conflicting_hotkeys: Rc<HashSet<String>> = Rc::new(empty_conflicts);
    let is_research_grid = false;
    let is_uprooted_grid = false;
    let prevent_swap_on_drop = false;
    let empty_restrictions: Vec<GridSlotId> = Vec::new();
    let restrict_draggable_to: Rc<[GridSlotId]> = empty_restrictions.into();
    let host_unit_id = fixtures::sample_unit_id();
    let update_hotkeys_on_move = use_signal(|| true);
    let hotkey_assign_request = use_signal(|| false);
    let tile_props = GridTileProps {
        column,
        row,
        heading,
        slot_ids,
        loaded_keys,
        selected_slot,
        selected_from_research,
        selected_from_uprooted,
        tier_overrides,
        dragging_slot,
        drop_target_cell,
        drag_follower,
        grid_layout,
        conflicting_hotkeys,
        is_research_grid,
        is_uprooted_grid,
        prevent_swap_on_drop,
        restrict_draggable_to,
        host_unit_id,
        update_hotkeys_on_move,
        hotkey_assign_request,
    };
    rsx! {
        ToastMount {
            GridTile { ..tile_props }
        }
    }
}

fn command_grid_section_footman() -> Element {
    let slot_ids = footman_command_slots();
    let loaded_keys = use_signal(|| None::<CustomKeys>);
    let selected_slot = use_signal(|| None::<GridSlotId>);
    let selected_from_research = use_signal(|| false);
    let selected_from_uprooted = use_signal(|| false);
    let tier_overrides: Signal<HashMap<String, usize>> = use_signal(HashMap::new);
    let dragging_slot: Signal<Option<DraggingSlot>> = use_signal(|| None);
    let drop_target_cell: Signal<Option<DropTargetCell>> = use_signal(|| None);
    let drag_follower: Signal<Option<DragFollower>> = use_signal(|| None);
    let grid_layout = use_signal(fixtures::sample_grid_layout);
    let host_unit_id = fixtures::sample_unit_id();
    let update_hotkeys_on_move = use_signal(|| true);
    let hotkey_assign_request = use_signal(|| false);
    rsx! {
        ToastMount {
            CommandGridSection {
                heading: "Main Commands",
                slot_ids,
                loaded_keys,
                selected_slot,
                selected_from_research,
                selected_from_uprooted,
                tier_overrides,
                dragging_slot,
                drop_target_cell,
                drag_follower,
                grid_layout,
                host_unit_id,
                update_hotkeys_on_move,
                hotkey_assign_request,
            }
        }
    }
}
