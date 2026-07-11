/// The published `View` contract mirroring [`TileOverrideEmptyModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TileOverrideEmptyView {
    pub message: String,
}

impl ddd::View for TileOverrideEmptyView {}
