use super::super::super::props::EmptyTileProps;
use super::super::super::state::EmptyTileState;
use dioxus::prelude::*;

/// Mounts only while the empty slot is the drag's drop-target candidate; every other
/// empty slot early-returns, so the overlay's presence is the drop-target signal the
/// tile root's border keys off.
#[derive(Props, Clone, PartialEq)]
pub struct DropTargetOverlayProps {
    pub active: bool,
}

impl From<&EmptyTileProps> for DropTargetOverlayProps {
    fn from(props: &EmptyTileProps) -> Self {
        let active = matches!(props.state, EmptyTileState::DropTarget);
        Self { active }
    }
}
