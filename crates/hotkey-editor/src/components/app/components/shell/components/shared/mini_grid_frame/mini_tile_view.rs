use crate::components::app::components::shell::components::shared::grid_tile::GridTileState;

/// One read-only mini-grid tile's data: its icon where a move places one, and
/// whether the cell is filled or empty. The shared frame builds each tile's private
/// `GridTile` props from these named fields; a mini's coordinate, label, and drag
/// flags are constants, so they are not part of the contract.
#[derive(Clone, PartialEq)]
pub struct MiniTileView {
    pub icon: Option<String>,
    pub state: GridTileState,
}
