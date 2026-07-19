use warcraft_api::DamagePerSecond;

#[derive(Clone, PartialEq)]
pub struct DamagePerSecondRowView {
    pub value: Option<DamagePerSecond>,
}

impl ddd::View for DamagePerSecondRowView {}
