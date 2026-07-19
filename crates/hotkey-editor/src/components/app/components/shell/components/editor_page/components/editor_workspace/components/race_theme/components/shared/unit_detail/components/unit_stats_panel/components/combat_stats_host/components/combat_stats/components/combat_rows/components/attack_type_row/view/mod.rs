use warcraft_api::AttackType;

#[derive(Clone, PartialEq)]
pub struct AttackTypeRowView {
    pub value: AttackType,
}

impl ddd::View for AttackTypeRowView {}
