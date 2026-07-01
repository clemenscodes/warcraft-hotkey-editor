pub mod components;
mod props;
mod state;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::tile_icon::{TileIcon, TileIconProps};
use components::tile_label::{TileLabel, TileLabelProps};

use super::super::TileChrome;
use super::tile_badge::{TileBadge, TileBadgeProps};

pub use props::FilledTileProps;

assert_component!(FilledTile);

/// An occupied command tile. Draws the ability icon (or its text fallback) and
/// the hotkey badge, themes its accent from the owning unit's race, and carries
/// the shared chrome (focus, drag, coordinate attributes, event handlers).
#[component]
pub fn FilledTile(props: FilledTileProps) -> Element {
    let icon = TileIconProps::from(&props);
    let label = TileLabelProps::from(&props);
    let badge = TileBadgeProps::from(&props);
    let class = style::class(props.state);
    let selected = props.selected;
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
            "data-selected": selected,
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
            TileIcon { ..icon }
            TileLabel { ..label }
            TileBadge { ..badge }
        }
    }
}
