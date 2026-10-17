use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct ConflictObjectIdView {
    pub object_id: WarcraftObjectId,
}

impl ddd::Layered for ConflictObjectIdView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ConflictObjectIdView {}
