/// The published `View` contract mirroring [`TileOverrideNameModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TileOverrideNameView {
    pub text: String,
}

impl ddd::View for TileOverrideNameView {}
