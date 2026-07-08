use super::super::super::GridTileProps;
use super::super::super::GridTileState;
use super::super::super::TileChrome;
use super::state::EmptyTileState;
use dioxus::prelude::*;

/// An empty command slot: the shared tile chrome and its drop-target look. During
/// a drag it can become a drop target (or a blocked one).
#[derive(Props, Clone, PartialEq)]
pub struct EmptyTileProps {
    pub chrome: TileChrome,
    pub state: EmptyTileState,
    /// `"true"` when this empty slot is the active drop-target candidate, as a
    /// `data-drop-target` hook for the position-picker styling (the tile's own
    /// look comes from `state`).
    pub drop_target: &'static str,
}

impl From<&GridTileProps> for EmptyTileProps {
    fn from(props: &GridTileProps) -> Self {
        let state = match props.state {
            GridTileState::DropTarget => EmptyTileState::DropTarget,
            GridTileState::BlockedDropTarget => EmptyTileState::BlockedDropTarget,
            GridTileState::Highlighted => EmptyTileState::Highlighted,
            _ => EmptyTileState::Empty,
        };
        let drop_target = if matches!(props.state, GridTileState::DropTarget) {
            "true"
        } else {
            "false"
        };
        let chrome = TileChrome::from(props);
        Self {
            chrome,
            state,
            drop_target,
        }
    }
}
