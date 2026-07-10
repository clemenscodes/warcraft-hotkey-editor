pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::shared::grid_tile::{
    GridTile, GridTileProps,
};
use components::tile_badge::{TileBadge, TileBadgeProps};
use dioxus::prelude::*;
pub use props::TileFaceProps;
use style::CLASS;
use tw_macro::assert_component;

/// The tile painter: the resting visual of a command-grid slot — the inert base
/// `GridTile` (filled or empty) with the `TileBadge` hotkey letter layered on top,
/// inside the square, container-query box the badge sizes against. Purely
/// presentational: props in, markup out, no handlers and no drag state. The editor's
/// `GridEditorTile` Host wraps this and adds the interaction; the templates preview and
/// the gallery render it directly, read-only.
#[component]
pub fn TileFace(props: TileFaceProps) -> Element {
    let base = GridTileProps::from(&props);
    let badge = TileBadgeProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            GridTile { ..base }
            TileBadge { ..badge }
        }
    }
}

assert_component!(TileFace);
