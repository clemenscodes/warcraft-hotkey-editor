use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct ConflictMetaView {
    pub name: String,
    pub unit_id: WarcraftObjectId,
    pub count: usize,
}

impl ddd::Layered for ConflictMetaView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ConflictMetaView {}
