use super::state::ConflictMarker;
use super::view::ConflictMarkerViewView;
use dioxus::prelude::*;

/// The marker between (or above) a conflict's abilities, plus whether it sits atop a
/// multi-way row. `ConflictMarkerView` switches on the marker to the hotkey badge or
/// the colliding-cell mini grid.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictMarkerViewProps {
    pub marker: ConflictMarker,
    #[props(default)]
    pub is_top: bool,
}

impl From<&ConflictMarkerViewView> for ConflictMarkerViewProps {
    fn from(view: &ConflictMarkerViewView) -> Self {
        let ConflictMarkerViewView { marker, is_top } = view.clone();
        Self { marker, is_top }
    }
}

impl ddd::Props for ConflictMarkerViewProps {
    type View = ConflictMarkerViewView;
}
