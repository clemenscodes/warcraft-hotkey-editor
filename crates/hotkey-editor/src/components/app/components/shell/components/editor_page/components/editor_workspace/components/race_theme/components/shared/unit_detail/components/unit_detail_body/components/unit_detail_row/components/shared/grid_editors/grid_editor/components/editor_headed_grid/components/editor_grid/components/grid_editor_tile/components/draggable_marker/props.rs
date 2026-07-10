use super::super::super::props::GridEditorTileProps;
use dioxus::prelude::*;

/// Mounts only on tiles the domain marks draggable; a display-only tile early-returns,
/// so the marker's presence is the tile's draggable signal — the grab cursor and the
/// off-state picker's treatment key off it, replacing the old `data-draggable` attribute.
#[derive(Props, Clone, PartialEq)]
pub struct DraggableMarkerProps {
    pub active: bool,
}

impl From<&GridEditorTileProps> for DraggableMarkerProps {
    fn from(props: &GridEditorTileProps) -> Self {
        let active = props.draggable;
        Self { active }
    }
}
