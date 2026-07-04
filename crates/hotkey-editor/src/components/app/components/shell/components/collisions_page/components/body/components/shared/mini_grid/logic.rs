use super::props::MiniGridProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::headed_grid::components::grid::GridProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::headed_grid::components::grid::components::grid_tile::{
    GridTileProps, GridTileState, PlainTileKind,
};
use warcraft_api::Race;
use warcraft_keybinds::{COMMAND_GRID_COLUMNS, COMMAND_GRID_TILE_COUNT};

/// The reused generic grid, bound to the plain base tile: the command grid's twelve
/// cells in row-major order, all empty but the one the given coordinate points at,
/// which is highlighted. The array index places each cell, so the highlight lands
/// at `row * columns + column`.
pub(super) fn grid(props: &MiniGridProps) -> GridProps<PlainTileKind> {
    let coordinate = props.coordinate;
    let highlight_column = u8::from(coordinate.column());
    let highlight_row = u8::from(coordinate.row());
    let columns = usize::from(COMMAND_GRID_COLUMNS);
    let highlight_index = usize::from(highlight_row) * columns + usize::from(highlight_column);
    let mut tile_list: Vec<GridTileProps> = Vec::with_capacity(COMMAND_GRID_TILE_COUNT);
    for index in 0..COMMAND_GRID_TILE_COUNT {
        let state = if index == highlight_index {
            GridTileState::Highlighted
        } else {
            GridTileState::Empty
        };
        let race = Race::Neutral;
        let icon = None;
        let label = String::new();
        let tile = GridTileProps {
            coordinate,
            race,
            icon,
            label,
            state,
        };
        tile_list.push(tile);
    }
    let tiles: [GridTileProps; COMMAND_GRID_TILE_COUNT] =
        tile_list
            .try_into()
            .unwrap_or_else(|list: Vec<GridTileProps>| {
                panic!(
                    "mini grid must render exactly {COMMAND_GRID_TILE_COUNT} tiles, got {}",
                    list.len(),
                )
            });
    let kind = PlainTileKind;
    GridProps { kind, tiles }
}
