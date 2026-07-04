use super::keys_mount::CustomKeysMount;
use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::components::grid_editors::command_grid_editor::CommandGridEditor;
use hotkey_editor::components::grid_editors::grid_editor::components::drag_follower_overlay::DragFollowerOverlay;
use std::collections::HashMap;
use std::rc::Rc;

use hotkey_editor::components::grid_editors::grid_editor::components::grid_editor_tile::components::tile_face::components::hotkey_badge::{
    HotkeyBadge, HotkeyBadgeState,
};

use hotkey_editor::components::grid_editors::grid_editor::components::grid_editor_tile::GridEditorTile;
use hotkey_editor::components::grid_editors::grid_editor::components::grid_editor_tile::components::tile_face::TileFace;
use hotkey_editor::components::grid_editors::grid_editor::components::headed_grid::components::grid::components::grid_tile::GridTileState;

use hotkey_editor::components::grid_editors::grid_editor::components::headed_grid::components::grid_heading::GridHeading;
use hotkey_editor::components::shell::toasts::ToastMount;

use hotkey_editor::model::grid::{DragFollower, DragFollowerVisual, DraggingSlot, DropTargetTile};

use super::fixtures;
use warcraft_api::Race;
use warcraft_database::WARCRAFT_DATABASE;
use warcraft_keybinds::{GridSlotId, HotkeyToken, UnitCommandSlots};

pub fn stories() -> Vec<Story> {
    vec![
        Story::new("Grid", "HotkeyBadge", "Normal", hotkey_badge_normal),
        Story::new("Grid", "HotkeyBadge", "Passive", hotkey_badge_passive),
        Story::new("Grid", "HotkeyBadge", "Conflict", hotkey_badge_conflict),
        Story::new("Grid", "TileFace", "Empty", grid_tile_empty),
        Story::new("Grid", "TileFace", "Filled", grid_tile_filled),
        Story::new("Grid", "TileFace", "Selected", grid_tile_selected),
        Story::new("Grid", "TileFace", "Command", grid_tile_command),
        Story::new("Grid", "TileFace", "Drop target", grid_tile_drop_target),
        Story::new("Grid", "TileFace", "Blocked drop target", grid_tile_blocked),
        Story::new(
            "Grid",
            "GridEditorTile",
            "Dragging source",
            grid_tile_dragging,
        ),
        Story::new("Grid", "GridEditorTile", "Drag over", grid_tile_drag_over),
        Story::new("Grid", "TileFace", "Conflict", grid_tile_conflict),
        Story::new("Grid", "TileFace", "Passive", grid_tile_passive),
        Story::new("Grid", "TileFace", "Selected orc", grid_tile_selected_orc),
        Story::new(
            "Grid",
            "DragFollowerOverlay",
            "With ability",
            drag_follower_with_ability,
        ),
        Story::new("Grid", "DragFollowerOverlay", "Empty", drag_follower_empty),
        Story::single("Grid", "GridHeading", grid_heading),
        Story::new(
            "Grid",
            "Grid",
            "Update hotkeys on move",
            grid_update_hotkeys,
        ),
        Story::new("Grid", "Grid", "Keep hotkeys on move", grid_keep_hotkeys),
        Story::new("Grid", "Grid", "Human", grid_human),
        Story::new("Grid", "Grid", "Orc", grid_orc),
        Story::new("Grid", "Grid", "Night elf", grid_nightelf),
        Story::new("Grid", "Grid", "Undead", grid_undead),
        Story::new("Grid", "Grid", "Neutral", grid_neutral),
        Story::single("Grid", "CommandGridEditor", grid_editor_footman),
    ]
}

fn hotkey_badge_demo(state: HotkeyBadgeState) -> Element {
    let letter = HotkeyToken::try_from('Q').expect("letter");
    rsx! {
        div { class: "relative w-24 h-24 [container-type:inline-size]",
            HotkeyBadge { letter, state }
        }
    }
}

fn hotkey_badge_normal() -> Element {
    hotkey_badge_demo(HotkeyBadgeState::Normal)
}

fn hotkey_badge_passive() -> Element {
    hotkey_badge_demo(HotkeyBadgeState::Passive)
}

fn hotkey_badge_conflict() -> Element {
    hotkey_badge_demo(HotkeyBadgeState::Conflict)
}

fn grid_tile_empty() -> Element {
    rsx! {
        TileFace {
            state: GridTileState::Empty,
            hotkey: HotkeyToken::try_from('Q')
                    .expect("letter"),
        }
    }
}

fn grid_tile_filled() -> Element {
    rsx! {
        TileFace {
            state: GridTileState::Filled,
            icon: fixtures::sample_icon_url(),
            label: "Footman".to_string(),
            hotkey: HotkeyToken::try_from('Q')
                    .expect("letter"),
        }
    }
}

fn grid_tile_selected() -> Element {
    rsx! {
        TileFace {
            state: GridTileState::Selected,
            icon: fixtures::sample_icon_url(),
            label: "Footman".to_string(),
            hotkey: HotkeyToken::try_from('Q')
                    .expect("letter"),
        }
    }
}

fn grid_tile_command() -> Element {
    rsx! {
        TileFace {
            state: GridTileState::Command,
            icon: fixtures::sample_icon_url(),
            label: "Build".to_string(),
            hotkey: HotkeyToken::try_from('Q')
                    .expect("letter"),
        }
    }
}

fn grid_tile_drop_target() -> Element {
    rsx! {
        TileFace {
            state: GridTileState::DropTarget,
            hotkey: HotkeyToken::try_from('Q')
                    .expect("letter"),
        }
    }
}

fn grid_tile_blocked() -> Element {
    rsx! {
        TileFace {
            state: GridTileState::BlockedDropTarget,
            hotkey: HotkeyToken::try_from('Q').expect("letter"),
        }
    }
}

fn grid_tile_dragging() -> Element {
    rsx! {
        GridEditorTile {
            state: GridTileState::Filled,
            icon: fixtures::sample_icon_url(),
            label: "Footman".to_string(),
            hotkey: HotkeyToken::try_from('Q')
                    .expect("letter"),
            is_focusable: true,
            draggable: true,
            is_dragging_source: true,
        }
    }
}

fn grid_tile_drag_over() -> Element {
    rsx! {
        GridEditorTile {
            state: GridTileState::Filled,
            icon: fixtures::sample_icon_url(),
            label: "Footman".to_string(),
            hotkey: HotkeyToken::try_from('Q')
                    .expect("letter"),
            is_focusable: true,
            draggable: true,
            is_drag_over: true,
        }
    }
}

fn grid_tile_conflict() -> Element {
    rsx! {
        TileFace {
            state: GridTileState::Filled,
            icon: fixtures::sample_icon_url(),
            label: "Footman".to_string(),
            hotkey: HotkeyToken::try_from('Q')
                    .expect("letter"),
            badge_state: HotkeyBadgeState::Conflict,
        }
    }
}

fn grid_tile_passive() -> Element {
    rsx! {
        TileFace {
            state: GridTileState::Filled,
            icon: fixtures::sample_icon_url(),
            label: "Inner Fire".to_string(),
            hotkey: HotkeyToken::try_from('Q')
                    .expect("letter"),
            badge_state: HotkeyBadgeState::Passive,
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

fn grid_tile_selected_orc() -> Element {
    rsx! {
        TileFace {
            race: Race::Orc,
            state: GridTileState::Selected,
            icon: fixtures::sample_icon_url(),
            label: "Grunt".to_string(),
            hotkey: HotkeyToken::try_from('Q').expect("letter"),
        }
    }
}

fn drag_follower_with_ability() -> Element {
    let icon_source = fixtures::sample_icon_url();
    let letter = HotkeyToken::try_from('Q').expect("letter");
    let visual = DragFollowerVisual::new(icon_source, "Footman".to_string(), letter, false, false);
    let follower = DragFollower::new(visual, 0.0, 0.0, 120.0, 120.0, 96.0, 96.0);
    let drag_follower = use_signal(|| Some(follower));
    rsx! {
        DragFollowerOverlay { drag_follower, race: Race::Orc }
    }
}

fn drag_follower_empty() -> Element {
    let drag_follower = use_signal(|| None::<DragFollower>);
    rsx! {
        DragFollowerOverlay { drag_follower }
    }
}

fn grid_heading() -> Element {
    rsx! {
        GridHeading { heading: "Main Commands" }
    }
}

fn grid_update_hotkeys() -> Element {
    grid_footman(true, Race::Neutral)
}

fn grid_keep_hotkeys() -> Element {
    grid_footman(false, Race::Neutral)
}

fn grid_human() -> Element {
    grid_footman(true, Race::Human)
}

fn grid_orc() -> Element {
    grid_footman(true, Race::Orc)
}

fn grid_nightelf() -> Element {
    grid_footman(true, Race::Nightelf)
}

fn grid_undead() -> Element {
    grid_footman(true, Race::Undead)
}

fn grid_neutral() -> Element {
    grid_footman(true, Race::Neutral)
}

fn grid_footman(update_hotkeys: bool, race: Race) -> Element {
    let slot_ids = footman_command_slots();
    let loaded_keys = use_signal(|| Some(fixtures::sample_keys_layout_applied()));
    let selected_slot = use_signal(|| None::<GridSlotId>);
    let selected_from_research = use_signal(|| false);
    let selected_from_uprooted = use_signal(|| false);
    let tier_overrides: Signal<HashMap<String, usize>> = use_signal(HashMap::new);
    let dragging_slot: Signal<Option<DraggingSlot>> = use_signal(|| None);
    let drop_target_tile: Signal<Option<DropTargetTile>> = use_signal(|| None);
    let drag_follower: Signal<Option<DragFollower>> = use_signal(|| None);
    let grid_layout = use_signal(fixtures::sample_grid_layout);
    let host_unit_id = fixtures::sample_unit_id();
    let update_hotkeys_on_move = use_signal(move || update_hotkeys);
    let hotkey_assign_request = use_signal(|| false);
    rsx! {
        ToastMount {
            CustomKeysMount {
                loaded_keys,
                CommandGridEditor {
                    heading: "Main Commands",
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
                    host_unit_id,
                    update_hotkeys_on_move,
                    hotkey_assign_request,
                }
            }
        }
    }
}

fn grid_editor_footman() -> Element {
    let slot_ids = footman_command_slots();
    let loaded_keys = use_signal(|| Some(fixtures::sample_keys_layout_applied()));
    let selected_slot = use_signal(|| None::<GridSlotId>);
    let selected_from_research = use_signal(|| false);
    let selected_from_uprooted = use_signal(|| false);
    let tier_overrides: Signal<HashMap<String, usize>> = use_signal(HashMap::new);
    let dragging_slot: Signal<Option<DraggingSlot>> = use_signal(|| None);
    let drop_target_tile: Signal<Option<DropTargetTile>> = use_signal(|| None);
    let drag_follower: Signal<Option<DragFollower>> = use_signal(|| None);
    let grid_layout = use_signal(fixtures::sample_grid_layout);
    let host_unit_id = fixtures::sample_unit_id();
    let update_hotkeys_on_move = use_signal(|| true);
    let hotkey_assign_request = use_signal(|| false);
    rsx! {
        ToastMount {
            CustomKeysMount {
                loaded_keys,
                CommandGridEditor {
                    heading: "Main Commands",
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
                    host_unit_id,
                    update_hotkeys_on_move,
                    hotkey_assign_request,
                }
            }
        }
    }
}
