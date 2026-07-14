use super::state::GridTileState;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) enum TileOccupancy {
    Filled,
    Empty,
}

impl From<GridTileState> for TileOccupancy {
    fn from(state: GridTileState) -> Self {
        match state {
            GridTileState::Filled | GridTileState::Selected | GridTileState::Command => {
                Self::Filled
            }
            GridTileState::Empty
            | GridTileState::DropTarget
            | GridTileState::BlockedDropTarget
            | GridTileState::Highlighted => Self::Empty,
        }
    }
}
