/// The published `View` contract mirroring [`FramedIconImageProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FramedIconImageView {
    pub source: String,
    pub alt: String,
}

impl ddd::View for FramedIconImageView {}
