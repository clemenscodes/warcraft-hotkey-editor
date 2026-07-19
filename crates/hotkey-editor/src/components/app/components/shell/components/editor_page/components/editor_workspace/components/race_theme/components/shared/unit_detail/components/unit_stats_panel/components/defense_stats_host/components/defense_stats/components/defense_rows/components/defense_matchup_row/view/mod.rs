use warcraft_api::DefenseType;

#[derive(Clone, PartialEq)]
pub struct DefenseMatchupRowView {
    pub defense_type: DefenseType,
}

impl ddd::View for DefenseMatchupRowView {}
