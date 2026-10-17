#[derive(Clone, PartialEq)]
pub struct UnitPortraitHostView {
    pub src: Option<String>,
    pub alt: &'static str,
}

impl ddd::Layered for UnitPortraitHostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitPortraitHostView {}
