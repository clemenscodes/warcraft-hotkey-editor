use std::collections::HashMap;
use std::rc::Rc;

use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::CustomKeys;

use dioxus_primitives::dialog::{DialogContent, DialogRoot};

use crate::components::command_grid::{CommandGridSection, CommandGridSectionProps};
use crate::components::dialog_header::DialogHeader;
use crate::grid_layout::GridLayout;
use crate::grid_slot::{DragFollower, DraggingSlot, DropTargetCell, GridSlotId};

/// Modal dialog for repositioning the off-state button of a toggle ability
/// (e.g. Stop Defend, Unburrow) on the command card.
#[component]
pub(crate) fn AltPositionPicker(
    object_id: WarcraftObjectId,
    display_name: String,
    picker_slots: Rc<[GridSlotId]>,
    loaded_keys: Signal<Option<CustomKeys>>,
    grid_layout: Signal<GridLayout>,
    dragging_slot: Signal<Option<DraggingSlot>>,
    drop_target_cell: Signal<Option<DropTargetCell>>,
    drag_follower: Signal<Option<DragFollower>>,
    mut alt_position_picker_open: Signal<bool>,
) -> Element {
    let picker_selected_slot =
        use_signal::<Option<GridSlotId>>(|| Some(GridSlotId::ability_off(object_id)));
    let picker_selected_research = use_signal::<bool>(|| false);
    let picker_selected_uprooted = use_signal::<bool>(|| false);
    let picker_tier_overrides = use_signal::<HashMap<String, usize>>(HashMap::new);
    let dialog_title = format!("Position: {display_name}");
    let restrict_draggable: Vec<GridSlotId> = vec![GridSlotId::ability_off(object_id)];
    let grid_props = CommandGridSectionProps {
        heading: "Off-state position",
        slot_ids: picker_slots,
        loaded_keys,
        selected_slot: picker_selected_slot,
        selected_from_research: picker_selected_research,
        selected_from_uprooted: picker_selected_uprooted,
        tier_overrides: picker_tier_overrides,
        dragging_slot,
        drop_target_cell,
        drag_follower,
        grid_layout,
        is_research_grid: false,
        is_uprooted_grid: false,
        prevent_swap_on_drop: true,
        restrict_draggable_to: restrict_draggable,
        host_unit_id: String::new(),
    };
    rsx! {
        DialogRoot {
            class: "dialog-overlay",
            open: alt_position_picker_open(),
            on_open_change: move |is_open| alt_position_picker_open.set(is_open),
            DialogContent { class: "dialog-shell wc3-dialog alt-position-picker-shell".to_string(),
                DialogHeader {
                    title: dialog_title,
                    on_close: move |_| alt_position_picker_open.set(false),
                }
                div { class: "wc3-dialog-body alt-position-picker-body",
                    p { class: "alt-position-picker-explainer",
                        "Drag the off-state button to a different cell. Cells holding another ability are protected; drops on top of them are rejected so the unit's primary layout stays intact."
                    }
                    div { class: "alt-position-picker-grid-anchor",
                        CommandGridSection { ..grid_props }
                    }
                }
            }
        }
    }
}

/// Modal dialog for repositioning the upgraded-form button of a unit that
/// transforms into a different unit after an upgrade (e.g. post-Barrage
/// Siege Engine).
#[component]
pub(crate) fn UpgradePositionPicker(
    upgrade_unit_id: WarcraftObjectId,
    display_name: String,
    picker_slots: Rc<[GridSlotId]>,
    loaded_keys: Signal<Option<CustomKeys>>,
    grid_layout: Signal<GridLayout>,
    dragging_slot: Signal<Option<DraggingSlot>>,
    drop_target_cell: Signal<Option<DropTargetCell>>,
    drag_follower: Signal<Option<DragFollower>>,
    mut upgrade_position_picker_open: Signal<bool>,
) -> Element {
    let picker_selected_slot =
        use_signal::<Option<GridSlotId>>(|| Some(GridSlotId::ability(upgrade_unit_id)));
    let picker_selected_research = use_signal::<bool>(|| false);
    let picker_selected_uprooted = use_signal::<bool>(|| false);
    let picker_tier_overrides = use_signal::<HashMap<String, usize>>(HashMap::new);
    let dialog_title = format!("Position: {display_name} (upgraded)");
    let restrict_draggable: Vec<GridSlotId> = vec![GridSlotId::ability(upgrade_unit_id)];
    let grid_props = CommandGridSectionProps {
        heading: "Upgraded-form position",
        slot_ids: picker_slots,
        loaded_keys,
        selected_slot: picker_selected_slot,
        selected_from_research: picker_selected_research,
        selected_from_uprooted: picker_selected_uprooted,
        tier_overrides: picker_tier_overrides,
        dragging_slot,
        drop_target_cell,
        drag_follower,
        grid_layout,
        is_research_grid: false,
        is_uprooted_grid: false,
        prevent_swap_on_drop: true,
        restrict_draggable_to: restrict_draggable,
        host_unit_id: String::new(),
    };
    rsx! {
        DialogRoot {
            class: "dialog-overlay",
            open: upgrade_position_picker_open(),
            on_open_change: move |is_open| upgrade_position_picker_open.set(is_open),
            DialogContent { class: "dialog-shell wc3-dialog alt-position-picker-shell".to_string(),
                DialogHeader {
                    title: dialog_title,
                    on_close: move |_| upgrade_position_picker_open.set(false),
                }
                div { class: "wc3-dialog-body alt-position-picker-body",
                    p { class: "alt-position-picker-explainer",
                        "Drag the upgraded-form button to a different cell. Cells holding another ability are protected; drops on top of them are rejected so the unit's primary layout stays intact."
                    }
                    div { class: "alt-position-picker-grid-anchor",
                        CommandGridSection { ..grid_props }
                    }
                }
            }
        }
    }
}
