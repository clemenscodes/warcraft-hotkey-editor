use super::components::mini_tile::MiniTileProps;
use super::props::MiniGridProps;
use warcraft_keybinds::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS};

/// The twelve tiles in row-major order, with the highlighted coordinate's tile
/// flagged.
pub(super) fn tiles(props: &MiniGridProps) -> Vec<MiniTileProps> {
    let highlight_column = u8::from(props.coordinate.column());
    let highlight_row = u8::from(props.coordinate.row());
    let mut tiles = Vec::new();
    for row in 0..COMMAND_GRID_ROWS {
        for column in 0..COMMAND_GRID_COLUMNS {
            let is_highlighted = column == highlight_column && row == highlight_row;
            tiles.push(MiniTileProps { is_highlighted });
        }
    }
    tiles
}
