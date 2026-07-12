use super::state::ConflictMarker;
use super::view::ConflictMarkerView;
use dioxus::prelude::*;

/// The marker between (or above) a conflict's abilities, plus whether it sits atop a
/// multi-way row. `ConflictMarker` switches on the marker to the hotkey badge or
/// the colliding-cell mini grid.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictMarkerModel {
    pub marker: ConflictMarker,
    #[props(default)]
    pub is_top: bool,
}

impl From<&ConflictMarkerView> for ConflictMarkerModel {
    fn from(view: &ConflictMarkerView) -> Self {
        let ConflictMarkerView { marker, is_top } = view.clone();
        Self { marker, is_top }
    }
}

impl ddd::Model for ConflictMarkerModel {
    type View = ConflictMarkerView;
}
