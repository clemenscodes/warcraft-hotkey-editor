/// The published `View` contract mirroring [`FollowerFigureModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FollowerFigureView {
    /// The dragged tile's icon. A follower always has one; you cannot drag a tile
    /// without an icon.
    pub src: String,
    pub alt: String,
}

impl ddd::View for FollowerFigureView {}
