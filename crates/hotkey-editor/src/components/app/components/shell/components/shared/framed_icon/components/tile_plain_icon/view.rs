/// The published `View` contract mirroring [`TilePlainIconProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TilePlainIconView {
    pub source: Option<String>,
    pub alt: String,
}

impl ddd::View for TilePlainIconView {}
