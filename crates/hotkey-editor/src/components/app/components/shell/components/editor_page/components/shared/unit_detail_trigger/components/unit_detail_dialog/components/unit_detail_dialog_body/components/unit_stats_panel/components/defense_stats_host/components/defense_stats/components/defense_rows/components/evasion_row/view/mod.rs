use warcraft_api::Evasion;

#[derive(Clone, PartialEq)]
pub struct EvasionRowView {
    pub value: Evasion,
}

impl ddd::Layered for EvasionRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for EvasionRowView {}
