#[derive(Clone, PartialEq)]
pub struct UnitPortraitView {
    pub src: String,
    pub alt: &'static str,
}

impl ddd::Layered for UnitPortraitView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitPortraitView {}
