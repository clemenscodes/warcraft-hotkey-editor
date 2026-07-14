#[derive(Clone, PartialEq)]
pub struct BrandTitleView {
    pub title: &'static str,
}

impl ddd::View for BrandTitleView {}
