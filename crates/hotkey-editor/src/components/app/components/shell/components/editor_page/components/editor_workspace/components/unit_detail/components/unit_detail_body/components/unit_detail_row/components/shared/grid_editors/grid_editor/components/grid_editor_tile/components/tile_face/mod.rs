pub mod components;
mod kind;
mod props;
mod style;

use tw_macro::assert_component;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::headed_grid::components::grid::components::grid_tile::{
    GridTile, GridTileProps,
};
use components::tile_badge::{TileBadge, TileBadgeProps};
use dioxus::prelude::*;
pub use kind::TileFaceKind;
pub use props::TileFaceProps;
use style::CLASS;
assert_component!(TileFace);

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
