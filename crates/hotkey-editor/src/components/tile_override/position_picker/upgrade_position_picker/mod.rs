use std::collections::HashMap;
use std::rc::Rc;

use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::CustomKeys;

use dioxus_primitives::dialog::{DialogContent, DialogRoot};

use crate::components::command_grid::{CommandGridSection, CommandGridSectionProps};
use crate::components::dialogs::dialog_header::DialogHeader;
use crate::model::grid::GridLayout;
use crate::model::grid::{DragFollower, DraggingSlot, DropTargetCell, GridSlotId};

#[derive(Props, Clone, PartialEq)]
pub(crate) struct UpgradePositionPickerProps {
    pub(crate) upgrade_unit_id: WarcraftObjectId,
    pub(crate) display_name: String,
    pub(crate) picker_slots: Rc<[GridSlotId]>,
    pub(crate) loaded_keys: Signal<Option<CustomKeys>>,
    pub(crate) grid_layout: Signal<GridLayout>,
    pub(crate) dragging_slot: Signal<Option<DraggingSlot>>,
    pub(crate) drop_target_cell: Signal<Option<DropTargetCell>>,
    pub(crate) drag_follower: Signal<Option<DragFollower>>,
    pub(crate) upgrade_position_picker_open: Signal<bool>,
}

#[component]
pub(crate) fn UpgradePositionPicker(props: UpgradePositionPickerProps) -> Element {
    let upgrade_unit_id = props.upgrade_unit_id;
    let display_name = props.display_name;
    let picker_slots = props.picker_slots;
    let loaded_keys = props.loaded_keys;
    let grid_layout = props.grid_layout;
    let dragging_slot = props.dragging_slot;
    let drop_target_cell = props.drop_target_cell;
    let drag_follower = props.drag_follower;
    let mut upgrade_position_picker_open = props.upgrade_position_picker_open;
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
    let handle_open_change = move |is_open| upgrade_position_picker_open.set(is_open);
    let handle_close = move |_| upgrade_position_picker_open.set(false);
    rsx! {
        DialogRoot {
            class: "dialog-overlay",
            open: upgrade_position_picker_open(),
            on_open_change: handle_open_change,
            DialogContent { class: "dialog-shell wc3-dialog alt-position-picker-shell".to_string(),
                DialogHeader {
                    title: dialog_title,
                    on_close: handle_close,
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
