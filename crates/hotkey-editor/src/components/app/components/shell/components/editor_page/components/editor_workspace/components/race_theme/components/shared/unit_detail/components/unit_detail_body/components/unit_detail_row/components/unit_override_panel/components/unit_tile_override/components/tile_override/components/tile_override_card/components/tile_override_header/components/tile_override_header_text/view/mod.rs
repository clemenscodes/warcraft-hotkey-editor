use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`TileOverrideHeaderTextModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TileOverrideHeaderTextView {
    pub name_text: String,
    pub object_id: WarcraftObjectId,
}

impl ddd::View for TileOverrideHeaderTextView {}
