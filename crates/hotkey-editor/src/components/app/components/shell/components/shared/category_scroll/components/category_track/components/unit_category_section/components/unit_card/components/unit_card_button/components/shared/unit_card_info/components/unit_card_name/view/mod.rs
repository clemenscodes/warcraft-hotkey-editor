#[derive(Clone, PartialEq)]
pub struct UnitCardNameView {
    pub text: String,
}

impl ddd::Layered for UnitCardNameView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitCardNameView {}
