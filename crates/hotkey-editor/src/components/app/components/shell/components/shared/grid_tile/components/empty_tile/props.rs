use super::super::super::GridTileProps;
use super::super::super::GridTileState;
use super::super::super::TileChrome;
use super::state::EmptyTileState;
use dioxus::prelude::*;

/// An empty command slot: the shared tile chrome and, during a drag, an overlay child
/// that makes it a drop target (or a blocked one), or a mini-grid highlight. The
/// overlay is what the tile root's own border keys off, so no look-flag attribute lives
/// on the root.
#[derive(Props, Clone, PartialEq)]
pub struct EmptyTileProps {
    pub chrome: TileChrome,
    pub state: EmptyTileState,
}

impl From<&GridTileProps> for EmptyTileProps {
    fn from(props: &GridTileProps) -> Self {
        let state = match props.state {
            GridTileState::DropTarget => EmptyTileState::DropTarget,
            GridTileState::BlockedDropTarget => EmptyTileState::BlockedDropTarget,
            GridTileState::Highlighted => EmptyTileState::Highlighted,
            _ => EmptyTileState::Empty,
        };
        let chrome = TileChrome::from(props);
        Self { chrome, state }
    }
}
