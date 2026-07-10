/// The published `View` contract mirroring [`BrandTitleProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BrandTitleView {
    pub title: &'static str,
}

impl ddd::View for BrandTitleView {}
