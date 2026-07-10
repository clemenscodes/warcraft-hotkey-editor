/// The published `View` contract mirroring [`TileOverrideNameProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TileOverrideNameView {
    pub text: String,
}

impl ddd::View for TileOverrideNameView {}
