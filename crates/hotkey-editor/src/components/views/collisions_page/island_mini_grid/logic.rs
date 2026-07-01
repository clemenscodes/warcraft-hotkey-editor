use super::components::island_mini_cell::IslandMiniCellProps;
use super::props::IslandMiniGridProps;
use warcraft_keybinds::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS};

/// The twelve cells in row-major order, with the collision cell flagged.
pub(super) fn cells(props: &IslandMiniGridProps) -> Vec<IslandMiniCellProps> {
    let mut cells = Vec::new();
    for row in 0..COMMAND_GRID_ROWS {
        for column in 0..COMMAND_GRID_COLUMNS {
            let is_collision = column == props.collision_column && row == props.collision_row;
            cells.push(IslandMiniCellProps { is_collision });
        }
    }
    cells
}
