/// The published `View` contract mirroring [`TileGlowIconProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TileGlowIconView {
    pub source: Option<String>,
    pub alt: String,
}

impl ddd::View for TileGlowIconView {}
