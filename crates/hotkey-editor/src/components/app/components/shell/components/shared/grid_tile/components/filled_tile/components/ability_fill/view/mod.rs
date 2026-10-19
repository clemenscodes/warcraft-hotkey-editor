#[derive(Clone, PartialEq)]
pub struct AbilityFillView {
    pub active: bool,
}

impl ddd::Layered for AbilityFillView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for AbilityFillView {}
