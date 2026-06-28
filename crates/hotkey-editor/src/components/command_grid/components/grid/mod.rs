mod components;
mod logic;
mod props;
mod style;
mod view;

use dioxus::prelude::*;

use logic::{GridRender, PreparedTile};
use style::COMMAND_GRID_STYLE_SHEETS;

pub use components::DragFollowerOverlay;
pub use props::CommandGridProps;
pub use view::{GridTileFlags, GridTileView};

use super::grid_tile::GridTile;

#[component]
pub fn CommandGrid(props: CommandGridProps) -> Element {
    let grid_id = props.grid_id;
    let race = props.race;
    let drag_follower = props.drag_follower;
    let GridRender {
        tiles,
        drag_active_here,
    } = GridRender::new(&props);

    rsx! {
        for style_sheet in COMMAND_GRID_STYLE_SHEETS {
            document::Stylesheet { href: style_sheet }
        }
        div { class: "command-grid", "data-grid-id": grid_id,
            for tile in tiles {
                {
                    let PreparedTile {
                        column,
                        row,
                        icon,
                        label,
                        hotkey,
                        badge_state,
                        state,
                        is_dragging_source,
                        is_drag_over,
                        is_focusable,
                        draggable,
                        onkeydown,
                        onpointerdown,
                        onpointermove,
                        onpointerup,
                        onpointercancel,
                        onlostpointercapture,
                        onclick,
                        ondoubleclick,
                    } = tile;
                    rsx! {
                        GridTile {
                            "data-grid-row": row,
                            "data-grid-col": column,
                            race,
                            icon,
                            label,
                            hotkey,
                            badge_state,
                            state,
                            is_dragging_source,
                            is_drag_over,
                            is_focusable,
                            draggable,
                            onkeydown,
                            onpointerdown,
                            onpointermove,
                            onpointerup,
                            onpointercancel,
                            onlostpointercapture,
                            onclick,
                            ondoubleclick,
                        }
                    }
                }
            }
        }
        DragFollowerOverlay { drag_follower, race, visible: drag_active_here }
    }
}
