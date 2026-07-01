mod props;
mod state;
mod style;

use dioxus::prelude::*;

use crate::assert_component;

use super::super::TileChrome;
use super::tile_badge::{TileBadge, TileBadgeProps};

pub use props::EmptyTileProps;

assert_component!(EmptyTile);

/// An empty command slot. Draws the position's hotkey badge, carries the shared
/// chrome, and shows the drop-target / blocked-drop-target look during a drag.
#[component]
pub fn EmptyTile(props: EmptyTileProps) -> Element {
    let badge = TileBadgeProps::from(&props);
    let class = style::class(props.state);
    let drop_target = props.drop_target;
    let TileChrome {
        tabindex,
        draggable_attribute,
        race_attribute,
        row,
        column,
        dragging_source,
        drag_over,
        onkeydown,
        onpointerdown,
        onpointermove,
        onpointerup,
        onpointercancel,
        onlostpointercapture,
        onclick,
        ondoubleclick,
    } = props.chrome;
    rsx! {
        div {
            class,
            tabindex,
            "data-race": race_attribute,
            "data-draggable": draggable_attribute,
            "data-grid-row": row,
            "data-grid-col": column,
            "data-drop-target": drop_target,
            "data-dragging-source": dragging_source,
            "data-drag-over": drag_over,
            onkeydown,
            onpointerdown,
            onpointermove,
            onpointerup,
            onpointercancel,
            onlostpointercapture,
            onclick,
            ondoubleclick,
            TileBadge { ..badge }
        }
    }
}
