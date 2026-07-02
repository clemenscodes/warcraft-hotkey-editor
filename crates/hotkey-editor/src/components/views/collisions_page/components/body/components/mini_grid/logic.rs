use super::components::mini_cell::MiniCellProps;
use super::props::MiniGridProps;
use warcraft_keybinds::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS};

/// The twelve cells in row-major order, with the collision cell flagged.
pub(super) fn cells(props: &MiniGridProps) -> Vec<MiniCellProps> {
    let mut cells = Vec::new();
    for row in 0..COMMAND_GRID_ROWS {
        for column in 0..COMMAND_GRID_COLUMNS {
            let is_collision = column == props.collision_column && row == props.collision_row;
            cells.push(MiniCellProps { is_collision });
        }
    }
    cells
}
