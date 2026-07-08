use super::super::super::props::EmptyTileProps;
use super::super::super::state::EmptyTileState;
use dioxus::prelude::*;

/// Mounts only when a drop onto this empty slot is refused (another ability's off-state
/// reserves it); every other empty slot early-returns.
#[derive(Props, Clone, PartialEq)]
pub struct BlockedDropTargetOverlayProps {
    pub active: bool,
}

impl From<&EmptyTileProps> for BlockedDropTargetOverlayProps {
    fn from(props: &EmptyTileProps) -> Self {
        let active = matches!(props.state, EmptyTileState::BlockedDropTarget);
        Self { active }
    }
}
