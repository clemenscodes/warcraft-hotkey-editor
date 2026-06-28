use std::collections::HashMap;
use std::rc::Rc;

use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::{
    CommandGridHeading, CommandGridSection, DragFollower, DragFollowerOverlay, DragFollowerVisual,
    DraggingSlot, DropTargetTile, GridTile, GridTileState, HotkeyBadge, HotkeyBadgeState,
    ToastMount,
};
use warcraft_api::Race;
use warcraft_database::WARCRAFT_DATABASE;
use warcraft_keybinds::{GridSlotId, HotkeyToken, UnitCommandSlots};

use super::fixtures;

const COMMAND_GRID_STORY_STYLES: Asset = asset!("/src/stories/command_grid.css");

pub fn stories() -> Vec<Story> {
    vec![
        Story::new("Command grid", "HotkeyBadge", "Normal", hotkey_badge_normal),
        Story::new(
            "Command grid",
            "HotkeyBadge",
            "Passive",
            hotkey_badge_passive,
        ),
        Story::new(
            "Command grid",
            "HotkeyBadge",
            "Conflict",
            hotkey_badge_conflict,
        ),
        Story::new("Command grid", "GridTile", "Empty", grid_tile_empty),
        Story::new("Command grid", "GridTile", "Filled", grid_tile_filled),
        Story::new("Command grid", "GridTile", "Selected", grid_tile_selected),
        Story::new("Command grid", "GridTile", "Command", grid_tile_command),
        Story::new(
            "Command grid",
            "GridTile",
            "Drop target",
            grid_tile_drop_target,
        ),
        Story::new(
            "Command grid",
            "GridTile",
            "Blocked drop target",
            grid_tile_blocked,
        ),
        Story::new(
            "Command grid",
            "GridTile",
            "Dragging source",
            grid_tile_dragging,
        ),
        Story::new("Command grid", "GridTile", "Drag over", grid_tile_drag_over),
        Story::new("Command grid", "GridTile", "Conflict", grid_tile_conflict),
        Story::new("Command grid", "GridTile", "Passive", grid_tile_passive),
        Story::new(
            "Command grid",
            "GridTile",
            "Selected orc",
            grid_tile_selected_orc,
        ),
        Story::new(
            "Command grid",
            "DragFollowerOverlay",
            "With ability",
            drag_follower_with_ability,
        ),
        Story::new(
            "Command grid",
            "DragFollowerOverlay",
            "Empty",
            drag_follower_empty,
        ),
        Story::single("Command grid", "CommandGridHeading", command_grid_heading),
        Story::new(
            "Command grid",
            "CommandGrid",
            "Update hotkeys on move",
            command_grid_update_hotkeys,
        ),
        Story::new(
            "Command grid",
            "CommandGrid",
            "Keep hotkeys on move",
            command_grid_keep_hotkeys,
        ),
        Story::new("Command grid", "CommandGrid", "Human", command_grid_human),
        Story::new("Command grid", "CommandGrid", "Orc", command_grid_orc),
        Story::new(
            "Command grid",
            "CommandGrid",
            "Night elf",
            command_grid_nightelf,
        ),
        Story::new("Command grid", "CommandGrid", "Undead", command_grid_undead),
        Story::new(
            "Command grid",
            "CommandGrid",
            "Neutral",
            command_grid_neutral,
        ),
        Story::single(
            "Command grid",
            "CommandGridSection",
            command_grid_section_footman,
        ),
    ]
}

fn hotkey_badge_demo(state: HotkeyBadgeState) -> Element {
    let letter = HotkeyToken::try_from('Q').expect("letter");
    rsx! {
        document::Stylesheet { href: COMMAND_GRID_STORY_STYLES }
        div { class: "command-grid-preview-slot",
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

// The presentational tile sizes itself, so each state story renders it directly.
// Hover and keyboard-focus styles are exercised by interacting with any filled
// tile in the browser.

fn grid_tile_empty() -> Element {
    rsx! {
        GridTile {
            state: GridTileState::Empty,
            hotkey: HotkeyToken::try_from('Q').expect("letter"),
        }
    }
}

fn grid_tile_filled() -> Element {
    rsx! {
        GridTile {
            state: GridTileState::Filled,
            icon: fixtures::sample_icon_url(),
            label: "Footman".to_string(),
            hotkey: HotkeyToken::try_from('Q').expect("letter"),
            is_focusable: true,
            draggable: true,
        }
    }
}

fn grid_tile_selected() -> Element {
    rsx! {
        GridTile {
            state: GridTileState::Selected,
            icon: fixtures::sample_icon_url(),
            label: "Footman".to_string(),
            hotkey: HotkeyToken::try_from('Q').expect("letter"),
            is_focusable: true,
            draggable: true,
        }
    }
}

fn grid_tile_command() -> Element {
    rsx! {
        GridTile {
            state: GridTileState::Command,
            icon: fixtures::sample_icon_url(),
            label: "Build".to_string(),
            hotkey: HotkeyToken::try_from('Q').expect("letter"),
            is_focusable: true,
        }
    }
}

fn grid_tile_drop_target() -> Element {
    rsx! {
        GridTile {
            state: GridTileState::DropTarget,
            hotkey: HotkeyToken::try_from('Q').expect("letter"),
        }
    }
}

fn grid_tile_blocked() -> Element {
    rsx! {
        GridTile {
            state: GridTileState::BlockedDropTarget,
            hotkey: HotkeyToken::try_from('Q').expect("letter"),
        }
    }
}

fn grid_tile_dragging() -> Element {
    rsx! {
        GridTile {
            state: GridTileState::Filled,
            icon: fixtures::sample_icon_url(),
            label: "Footman".to_string(),
            hotkey: HotkeyToken::try_from('Q').expect("letter"),
            is_focusable: true,
            draggable: true,
            is_dragging_source: true,
        }
    }
}

fn grid_tile_drag_over() -> Element {
    rsx! {
        GridTile {
            state: GridTileState::Filled,
            icon: fixtures::sample_icon_url(),
            label: "Footman".to_string(),
            hotkey: HotkeyToken::try_from('Q').expect("letter"),
            is_focusable: true,
            draggable: true,
            is_drag_over: true,
        }
    }
}

fn grid_tile_conflict() -> Element {
    rsx! {
        GridTile {
            state: GridTileState::Filled,
            icon: fixtures::sample_icon_url(),
            label: "Footman".to_string(),
            hotkey: HotkeyToken::try_from('Q').expect("letter"),
            badge_state: HotkeyBadgeState::Conflict,
            is_focusable: true,
            draggable: true,
        }
    }
}

fn grid_tile_passive() -> Element {
    rsx! {
        GridTile {
            state: GridTileState::Filled,
            icon: fixtures::sample_icon_url(),
            label: "Inner Fire".to_string(),
            hotkey: HotkeyToken::try_from('Q').expect("letter"),
            badge_state: HotkeyBadgeState::Passive,
            is_focusable: true,
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
        GridTile {
            race: Race::Orc,
            state: GridTileState::Selected,
            icon: fixtures::sample_icon_url(),
            label: "Grunt".to_string(),
            hotkey: HotkeyToken::try_from('Q').expect("letter"),
            is_focusable: true,
            draggable: true,
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

fn command_grid_heading() -> Element {
    rsx! {
        CommandGridHeading { heading: "Main Commands" }
    }
}

fn command_grid_update_hotkeys() -> Element {
    command_grid_footman(true, Race::Neutral)
}

fn command_grid_keep_hotkeys() -> Element {
    command_grid_footman(false, Race::Neutral)
}

fn command_grid_human() -> Element {
    command_grid_footman(true, Race::Human)
}

fn command_grid_orc() -> Element {
    command_grid_footman(true, Race::Orc)
}

fn command_grid_nightelf() -> Element {
    command_grid_footman(true, Race::Nightelf)
}

fn command_grid_undead() -> Element {
    command_grid_footman(true, Race::Undead)
}

fn command_grid_neutral() -> Element {
    command_grid_footman(true, Race::Neutral)
}

// `update_hotkeys` ON: a move/swap rebinds hotkeys to the new cells' layout
// letters. OFF: positions move but each ability keeps its own hotkey. `race`
// themes every tile's hover/selected accent.
fn command_grid_footman(update_hotkeys: bool, race: Race) -> Element {
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
            CommandGridSection {
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

fn command_grid_section_footman() -> Element {
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
            CommandGridSection {
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
