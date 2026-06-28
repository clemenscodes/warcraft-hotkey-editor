mod components;
mod logic;
mod props;
mod state;
mod style;

use dioxus::prelude::*;

use components::{TileBadge, TileBadgeProps, TileFigure, TileFigureProps};
use logic::GridTilePresentation;
use style::GRID_TILE_STYLE_SHEETS;

pub use components::{HotkeyBadge, HotkeyBadgeProps, HotkeyBadgeState};
pub use props::GridTileProps;
pub use state::GridTileState;

#[component]
pub fn GridTile(props: GridTileProps) -> Element {
    let figure = TileFigureProps::from(&props);
    let badge = TileBadgeProps::from(&props);
    let GridTilePresentation {
        class,
        tabindex,
        draggable_attribute,
        race_attribute,
        row,
        column,
        onkeydown,
        onpointerdown,
        onpointermove,
        onpointerup,
        onpointercancel,
        onlostpointercapture,
        onclick,
        ondoubleclick,
    } = GridTilePresentation::from(&props);
    rsx! {
        for href in GRID_TILE_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        div {
            class,
            tabindex,
            "data-race": race_attribute,
            "data-draggable": draggable_attribute,
            "data-grid-row": row,
            "data-grid-col": column,
            onkeydown,
            onpointerdown,
            onpointermove,
            onpointerup,
            onpointercancel,
            onlostpointercapture,
            onclick,
            ondoubleclick,
            TileFigure { ..figure }
            TileBadge { ..badge }
        }
    }
}
