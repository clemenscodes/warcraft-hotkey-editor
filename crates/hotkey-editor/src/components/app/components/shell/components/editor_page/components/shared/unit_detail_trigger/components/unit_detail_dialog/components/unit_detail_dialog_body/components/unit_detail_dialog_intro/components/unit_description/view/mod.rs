#[derive(Clone, PartialEq)]
pub struct UnitDescriptionView {
    pub text: String,
}

impl ddd::Layered for UnitDescriptionView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitDescriptionView {}
