use warcraft_api::DefenseType;

#[derive(Clone, PartialEq)]
pub struct DefenseTypeRowView {
    pub value: DefenseType,
}

impl ddd::Layered for DefenseTypeRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for DefenseTypeRowView {}
