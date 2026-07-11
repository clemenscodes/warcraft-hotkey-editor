use super::view::DraggableMarkerView;
use dioxus::prelude::*;

/// Mounts only on tiles the domain marks draggable; a display-only tile early-returns,
/// so the marker's presence is the tile's draggable signal — the grab cursor and the
/// off-state picker's treatment key off it, replacing the old `data-draggable` attribute.
#[derive(Props, Clone, PartialEq)]
pub struct DraggableMarkerModel {
    pub active: bool,
}

impl From<&DraggableMarkerView> for DraggableMarkerModel {
    fn from(view: &DraggableMarkerView) -> Self {
        let DraggableMarkerView { active } = view.clone();
        Self { active }
    }
}

impl ddd::Model for DraggableMarkerModel {
    type View = DraggableMarkerView;
}
