use super::state::ConflictMarker;
use super::view::ConflictMarkerViewView;
use dioxus::prelude::*;

/// The marker between (or above) a conflict's abilities, plus whether it sits atop a
/// multi-way row. `ConflictMarkerView` switches on the marker to the hotkey badge or
/// the colliding-cell mini grid.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictMarkerViewModel {
    pub marker: ConflictMarker,
    #[props(default)]
    pub is_top: bool,
}

impl From<&ConflictMarkerViewView> for ConflictMarkerViewModel {
    fn from(view: &ConflictMarkerViewView) -> Self {
        let ConflictMarkerViewView { marker, is_top } = view.clone();
        Self { marker, is_top }
    }
}

impl ddd::Model for ConflictMarkerViewModel {
    type View = ConflictMarkerViewView;
}
