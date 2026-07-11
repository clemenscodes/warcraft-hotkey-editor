/// The published `View` contract mirroring [`FramedIconImageModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FramedIconImageView {
    pub source: String,
    pub alt: String,
}

impl ddd::View for FramedIconImageView {}
