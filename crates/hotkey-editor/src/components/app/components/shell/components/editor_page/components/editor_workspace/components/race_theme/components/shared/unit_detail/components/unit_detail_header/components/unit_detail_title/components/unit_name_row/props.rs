use super::view::UnitNameRowView;
use dioxus::prelude::*;

/// The name row: the unit name, and — for heroes — the level picker beside it.
#[derive(Props, Clone, PartialEq)]
pub struct UnitNameRowProps {
    pub unit_name: &'static str,
    pub has_hero_attributes: bool,
}

impl From<&UnitNameRowView> for UnitNameRowProps {
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

impl ddd::Props for UnitNameRowProps {
    type View = UnitNameRowView;
}
