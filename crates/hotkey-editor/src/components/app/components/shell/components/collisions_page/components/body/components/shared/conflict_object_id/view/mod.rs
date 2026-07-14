use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct ConflictObjectIdView {
    pub object_id: WarcraftObjectId,
}

impl ddd::View for ConflictObjectIdView {}
