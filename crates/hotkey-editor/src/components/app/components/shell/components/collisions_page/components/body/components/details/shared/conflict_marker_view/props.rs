use super::state::ConflictMarker;
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
