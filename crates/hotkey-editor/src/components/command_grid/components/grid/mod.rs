mod components;
mod hooks;
mod logic;
mod style;

use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_primitives::toast::use_toast;

use crate::components::command_grid::CommandGridSectionProps;
use crate::model::grid::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, GridSlotId};
use hooks::use_conflicting_hotkeys;
use logic::{TileInputs, resolve_tile};
use style::COMMAND_GRID_STYLE_SHEETS;

pub use components::DragFollowerOverlay;

use super::grid_tile::GridTile;

#[component]
pub fn CommandGrid(props: CommandGridSectionProps) -> Element {
    let CommandGridSectionProps {
        heading,
        race,
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
        update_hotkeys_on_move,
        hotkey_assign_request,
        is_research_grid,
        is_uprooted_grid,
        prevent_swap_on_drop,
        restrict_draggable_to,
        host_unit_id,
    } = props;

    let toast = use_toast();
    let restrict_draggable_to: Rc<[GridSlotId]> = restrict_draggable_to.into();
    let conflicting_hotkeys = use_conflicting_hotkeys(loaded_keys, &slot_ids, is_research_grid);

    let inputs = TileInputs {
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
        update_hotkeys_on_move,
        hotkey_assign_request,
        conflicting_hotkeys,
        is_research_grid,
        is_uprooted_grid,
        prevent_swap_on_drop,
        restrict_draggable_to,
        host_unit_id,
        heading,
        toast,
    };

    // The follower is rendered by the grid that owns the in-progress drag (its
    // source section matches), so exactly one follower shows even with several
    // grids on screen, and a standalone gallery grid gets one for free.
    let drag_from_this_section = dragging_slot
        .read()
        .as_ref()
        .is_some_and(|detail| detail.source_section() == heading);

    rsx! {
        for style_sheet in COMMAND_GRID_STYLE_SHEETS {
            document::Stylesheet { href: style_sheet }
        }
        div { class: "grid-tiles",
            for row in 0..COMMAND_GRID_ROWS {
                for column in 0..COMMAND_GRID_COLUMNS {
                    {
                        let tile = resolve_tile(&inputs, column, row);
                        rsx! {
                            GridTile {
                                "data-grid-row": row,
                                "data-grid-col": column,
                                "data-grid-section": heading,
                                race,
                                icon: tile.icon,
                                label: tile.label,
                                hotkey: tile.hotkey,
                                badge_state: tile.badge_state,
                                state: tile.state,
                                is_dragging_source: tile.is_dragging_source,
                                is_drag_over: tile.is_drag_over,
                                is_focusable: tile.is_focusable,
                                draggable: tile.draggable,
                                onkeydown: tile.handlers.keydown,
                                onpointerdown: tile.handlers.pointer_down,
                                onpointermove: tile.handlers.pointer_move,
                                onpointerup: tile.handlers.pointer_up,
                                onpointercancel: tile.handlers.pointer_cancel,
                                onlostpointercapture: tile.handlers.lost_pointer_capture,
                                onclick: tile.handlers.click,
                                ondoubleclick: tile.handlers.double_click,
                            }
                        }
                    }
                }
            }
        }
        DragFollowerOverlay { drag_follower, race, visible: drag_from_this_section }
    }
}
