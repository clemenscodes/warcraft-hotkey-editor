/// The published `View` contract mirroring [`BrandTitleModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BrandTitleView {
    pub title: &'static str,
}

impl ddd::View for BrandTitleView {}
