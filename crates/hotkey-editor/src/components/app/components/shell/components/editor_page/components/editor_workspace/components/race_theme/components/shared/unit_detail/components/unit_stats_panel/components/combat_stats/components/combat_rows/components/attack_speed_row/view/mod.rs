use warcraft_api::AttackSpeed;

#[derive(Clone, PartialEq)]
pub struct AttackSpeedRowView {
    pub value: AttackSpeed,
}

impl ddd::View for AttackSpeedRowView {}
