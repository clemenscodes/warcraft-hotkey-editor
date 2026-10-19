#[derive(Clone, PartialEq)]
pub struct UnitNameView {
    pub text: &'static str,
}

impl ddd::Layered for UnitNameView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitNameView {}
