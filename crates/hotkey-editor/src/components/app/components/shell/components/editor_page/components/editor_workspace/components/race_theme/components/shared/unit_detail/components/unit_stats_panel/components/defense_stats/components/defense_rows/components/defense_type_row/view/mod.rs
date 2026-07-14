use warcraft_api::DefenseType;

#[derive(Clone, PartialEq)]
pub struct DefenseTypeRowView {
    pub value: DefenseType,
}

impl ddd::View for DefenseTypeRowView {}
