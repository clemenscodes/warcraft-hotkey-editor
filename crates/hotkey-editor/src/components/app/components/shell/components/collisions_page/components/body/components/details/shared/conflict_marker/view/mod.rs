use super::state::ConflictMarker;

#[derive(Clone, PartialEq)]
pub struct ConflictMarkerView {
    pub marker: ConflictMarker,
    pub is_top: bool,
}

impl ddd::View for ConflictMarkerView {}
