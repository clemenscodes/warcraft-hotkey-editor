#[derive(Clone, PartialEq)]
pub struct BrandTitleView {
    pub title: &'static str,
}

impl ddd::Layered for BrandTitleView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BrandTitleView {}
