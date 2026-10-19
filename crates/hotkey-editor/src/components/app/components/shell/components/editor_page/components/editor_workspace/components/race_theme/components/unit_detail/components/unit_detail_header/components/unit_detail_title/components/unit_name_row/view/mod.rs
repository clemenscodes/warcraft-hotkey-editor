#[derive(Clone, PartialEq)]
pub struct UnitNameRowView {
    pub unit_name: &'static str,
    pub has_hero_attributes: bool,
}

impl ddd::Layered for UnitNameRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitNameRowView {}
