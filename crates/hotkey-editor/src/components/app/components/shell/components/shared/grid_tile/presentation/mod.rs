use super::state::GridTileState;

/// Whether a grid slot is occupied. This is the dispatcher's whole decision: an
/// occupied slot renders `FilledTile`, an empty one `EmptyTile`. It is derived purely
/// from the slot's `GridTileState`, so the body only matches and renders — it never
/// builds a child's props to decide.
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
