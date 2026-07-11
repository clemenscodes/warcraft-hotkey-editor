/// The published `View` contract mirroring [`TileOverrideInfoOnlyModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TileOverrideInfoOnlyView {
    pub text: String,
}

impl ddd::View for TileOverrideInfoOnlyView {}
