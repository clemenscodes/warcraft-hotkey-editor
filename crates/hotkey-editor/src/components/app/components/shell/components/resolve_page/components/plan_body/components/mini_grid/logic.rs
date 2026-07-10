use super::props::MiniGridProps;
use crate::components::app::components::shell::components::shared::grid_tile::{
    GridTileView, GridTileState,
};
use warcraft_keybinds::{
    COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, COMMAND_GRID_TILE_COUNT, GridCoordinate,
};

/// The twelve read-only base tiles: the command grid's cells in row-major order,
/// each drawn filled with its ability's icon where a move places one and empty
/// everywhere else. The mini is read-only, so the tiles carry no meaningful
/// coordinate — the default stands in for the unused display address.
pub(super) fn grid(props: &MiniGridProps) -> [GridTileView; COMMAND_GRID_TILE_COUNT] {
    let coordinate = GridCoordinate::default();
    let mut tile_list: Vec<GridTileView> = Vec::with_capacity(COMMAND_GRID_TILE_COUNT);
    for grid_row in 0..COMMAND_GRID_ROWS {
        for grid_column in 0..COMMAND_GRID_COLUMNS {
            let placement = props.placements.iter().find(|placed| {
                let coordinate = placed.coordinate();
                let placed_column = u8::from(coordinate.column());
                let placed_row = u8::from(coordinate.row());
                placed_column == grid_column && placed_row == grid_row
            });
            let icon = placement.and_then(|placed| placed.icon_url().map(str::to_owned));
            let state = if placement.is_some() {
                GridTileState::Filled
            } else {
                GridTileState::Empty
            };
            let label = String::new();
            let tile = GridTileView {
                coordinate,
                icon,
                label,
                state,
                is_dragging_source: false,
                is_drag_over: false,
            };
            tile_list.push(tile);
        }
    }
    let tiles: [GridTileView; COMMAND_GRID_TILE_COUNT] =
        tile_list
            .try_into()
            .unwrap_or_else(|list: Vec<GridTileView>| {
                panic!(
                    "mini grid must render exactly {COMMAND_GRID_TILE_COUNT} tiles, got {}",
                    list.len(),
                )
            });
    tiles
}
