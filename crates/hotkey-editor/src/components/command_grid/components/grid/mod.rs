mod components;
mod logic;
mod props;
mod style;
mod view;

use dioxus::prelude::*;

use logic::GridRender;
use style::COMMAND_GRID_STYLE_SHEETS;

pub use components::{DragFollowerOverlay, DragFollowerOverlayProps};
pub use props::CommandGridProps;
pub use view::{GridTileFlags, GridTileView};

use super::grid_tile::GridTile;

#[component]
pub fn CommandGrid(props: CommandGridProps) -> Element {
    rsx! {
        for href in COMMAND_GRID_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        div {
            class: "command-grid",
            "data-grid-id": props.grid_id,
            for tile in GridRender::from(&props).tiles {
                GridTile { ..tile }
            }
        }
        DragFollowerOverlay { ..DragFollowerOverlayProps::from(&props) }
    }
}
