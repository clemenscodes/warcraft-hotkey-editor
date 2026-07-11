/// The published `View` contract mirroring [`UnitNameRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitNameRowView {
    pub unit_name: &'static str,
    pub has_hero_attributes: bool,
}

impl ddd::View for UnitNameRowView {}
