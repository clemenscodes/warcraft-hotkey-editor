use super::props::MiniGridProps;
use crate::components::app::components::shell::components::shared::grid_tile::{
    GridTileProps, GridTileState,
};
use warcraft_keybinds::{COMMAND_GRID_COLUMNS, COMMAND_GRID_TILE_COUNT};

/// The twelve read-only base tiles: the command grid's cells in row-major order, all
/// empty but the one the given coordinate points at, which is highlighted. The array
/// index places each cell, so the highlight lands at `row * columns + column`.
pub(super) fn grid(props: &MiniGridProps) -> [GridTileProps; COMMAND_GRID_TILE_COUNT] {
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
        let icon = None;
        let label = String::new();
        let tile = GridTileProps {
            coordinate,
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
    tiles
}
