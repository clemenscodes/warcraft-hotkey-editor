use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`ConflictObjectIdProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictObjectIdView {
    pub object_id: WarcraftObjectId,
}

impl ddd::View for ConflictObjectIdView {}
