use super::components::mini_cell::MiniCellProps;
use super::props::MiniGridProps;
use warcraft_keybinds::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS};

/// The 4×3 grid's cells in row-major order, each carrying the ability (if any)
/// that lands on it.
pub(super) fn cells(props: &MiniGridProps) -> Vec<MiniCellProps> {
    let mut cells: Vec<MiniCellProps> = Vec::new();
    for grid_row in 0..COMMAND_GRID_ROWS {
        for grid_column in 0..COMMAND_GRID_COLUMNS {
            let placement = props
                .placements
                .iter()
                .find(|placed| placed.column == grid_column && placed.row == grid_row);
            let cell = match placement {
                Some(placed) => MiniCellProps {
                    has_placement: true,
                    icon_url: placed.icon_url.clone(),
                    name: placed.name.clone(),
                },
                None => MiniCellProps {
                    has_placement: false,
                    icon_url: None,
                    name: String::new(),
                },
            };
            cells.push(cell);
        }
    }
    cells
}
