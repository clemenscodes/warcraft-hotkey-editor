use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`TileOverrideIdProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TileOverrideIdView {
    pub object_id: WarcraftObjectId,
}

impl ddd::View for TileOverrideIdView {}
