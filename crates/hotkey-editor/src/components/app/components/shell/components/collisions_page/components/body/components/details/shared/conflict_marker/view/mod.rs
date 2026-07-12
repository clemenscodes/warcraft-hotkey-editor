use super::state::ConflictMarker;

/// The published `View` contract mirroring [`ConflictMarkerModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictMarkerView {
    pub marker: ConflictMarker,
    pub is_top: bool,
}

impl ddd::View for ConflictMarkerView {}
