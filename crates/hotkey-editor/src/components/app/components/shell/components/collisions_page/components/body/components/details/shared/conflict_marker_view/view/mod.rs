use super::state::ConflictMarker;

/// The published `View` contract mirroring [`ConflictMarkerViewModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictMarkerViewView {
    pub marker: ConflictMarker,
    pub is_top: bool,
}

impl ddd::View for ConflictMarkerViewView {}
