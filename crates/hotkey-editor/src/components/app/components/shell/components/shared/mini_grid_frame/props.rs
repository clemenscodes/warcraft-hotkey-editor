use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::headed_grid::components::grid::GridProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::headed_grid::components::grid::components::grid_tile::PlainTileKind;
use dioxus::prelude::*;

/// The already-built twelve read-only tiles the frame lays out. Each page shapes
/// its own tiles — placement icons on the resolve plan, one highlighted cell on the
/// collisions page — and hands the finished `GridProps` to this shared frame, which
/// owns only the surrounding chrome and the tile-scoped border/radius overrides.
#[derive(Props, Clone, PartialEq)]
pub struct MiniGridFrameProps {
    pub grid: GridProps<PlainTileKind>,
}
