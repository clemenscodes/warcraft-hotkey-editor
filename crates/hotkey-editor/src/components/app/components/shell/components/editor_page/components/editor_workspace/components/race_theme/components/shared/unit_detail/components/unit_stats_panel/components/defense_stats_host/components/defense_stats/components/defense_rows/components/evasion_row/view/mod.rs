use warcraft_api::Evasion;

#[derive(Clone, PartialEq)]
pub struct EvasionRowView {
    pub value: Evasion,
}

impl ddd::View for EvasionRowView {}
