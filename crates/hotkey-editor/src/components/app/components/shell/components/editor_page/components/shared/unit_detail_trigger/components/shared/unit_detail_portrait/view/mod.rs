#[derive(Clone, PartialEq)]
pub struct UnitDetailPortraitView {
    pub src: Option<String>,
}

impl ddd::Layered for UnitDetailPortraitView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitDetailPortraitView {}
