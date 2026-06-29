mod components;
mod logic;
mod props;
mod style;
mod view;

use dioxus::prelude::*;

use logic::GridRender;
use style::GRID_STYLE_SHEETS;

pub use components::{
    DragFollowerOverlay, DragFollowerOverlayProps, GridTile, GridTileProps, GridTileState,
    HotkeyBadge, HotkeyBadgeProps, HotkeyBadgeState,
};
pub use props::GridProps;
pub use view::{GridTileFlags, GridTileView};

#[component]
pub fn Grid(props: GridProps) -> Element {
    rsx! {
        for href in GRID_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        div {
            class: "grid",
            for tile in GridRender::from(&props).tiles {
                GridTile { ..tile }
            }
        }
        DragFollowerOverlay { ..DragFollowerOverlayProps::from(&props) }
    }
}
