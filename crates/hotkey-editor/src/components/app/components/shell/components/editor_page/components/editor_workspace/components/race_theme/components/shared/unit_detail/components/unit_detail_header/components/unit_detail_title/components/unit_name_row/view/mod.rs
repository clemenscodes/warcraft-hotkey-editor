#[derive(Clone, PartialEq)]
pub struct UnitNameRowView {
    pub unit_name: &'static str,
    pub has_hero_attributes: bool,
}

impl ddd::View for UnitNameRowView {}
