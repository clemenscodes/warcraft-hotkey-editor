use super::props::MiniGridProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::headed_grid::components::grid::GridProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::headed_grid::components::grid::components::grid_tile::{
    GridTileProps, GridTileState, PlainTileKind,
};
use warcraft_api::Race;
use warcraft_keybinds::{
    COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, COMMAND_GRID_TILE_COUNT, GridCoordinate,
};

/// The reused generic grid, bound to the plain base tile: the command grid's twelve
/// cells in row-major order, each drawn filled with its ability's icon where a move
/// places one and empty everywhere else. The mini is read-only, so the tiles carry
/// no meaningful coordinate — the default stands in for the unused display address.
pub(super) fn grid(props: &MiniGridProps) -> GridProps<PlainTileKind> {
    let coordinate = GridCoordinate::default();
    let mut tile_list: Vec<GridTileProps> = Vec::with_capacity(COMMAND_GRID_TILE_COUNT);
    for grid_row in 0..COMMAND_GRID_ROWS {
        for grid_column in 0..COMMAND_GRID_COLUMNS {
            let placement = props
                .placements
                .iter()
                .find(|placed| placed.column() == grid_column && placed.row() == grid_row);
            let icon = placement.and_then(|placed| placed.icon_url().map(str::to_owned));
            let state = if placement.is_some() {
                GridTileState::Filled
            } else {
                GridTileState::Empty
            };
            let race = Race::Neutral;
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
