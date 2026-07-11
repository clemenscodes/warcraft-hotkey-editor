use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`ConflictMetaModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictMetaView {
    pub name: String,
    pub unit_id: WarcraftObjectId,
    pub count: usize,
}

impl ddd::View for ConflictMetaView {}
