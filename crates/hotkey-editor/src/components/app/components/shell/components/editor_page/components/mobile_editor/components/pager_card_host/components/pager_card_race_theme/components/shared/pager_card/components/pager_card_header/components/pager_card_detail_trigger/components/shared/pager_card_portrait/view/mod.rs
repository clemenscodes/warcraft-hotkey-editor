#[derive(Clone, PartialEq)]
pub struct PagerCardPortraitView {
    pub src: Option<String>,
}

impl ddd::Layered for PagerCardPortraitView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PagerCardPortraitView {}
