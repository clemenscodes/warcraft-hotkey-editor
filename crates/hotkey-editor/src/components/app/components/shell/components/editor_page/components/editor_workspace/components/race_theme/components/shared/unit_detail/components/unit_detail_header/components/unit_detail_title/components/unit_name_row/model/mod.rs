use super::view::UnitNameRowView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnitNameRowModel {
    pub unit_name: &'static str,
    pub has_hero_attributes: bool,
}

impl From<&UnitNameRowView> for UnitNameRowModel {
    fn from(view: &UnitNameRowView) -> Self {
        let UnitNameRowView {
            unit_name,
            has_hero_attributes,
        } = view.clone();
        Self {
            unit_name,
            has_hero_attributes,
        }
    }
}

impl ddd::Model for UnitNameRowModel {
    type View = UnitNameRowView;
}
