use warcraft_api::DamageRange;

#[derive(Clone, PartialEq)]
pub struct DamageRowView {
    pub value: DamageRange,
}

impl ddd::View for DamageRowView {}
