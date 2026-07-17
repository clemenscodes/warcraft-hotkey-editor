use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SearchScopeMenuView {
    pub unit_label: String,
    pub ability_label: String,
    pub unit_is_active: bool,
    pub ability_is_active: bool,
    pub select_unit: EventHandler<MouseEvent>,
    pub select_ability: EventHandler<MouseEvent>,
}

impl ddd::View for SearchScopeMenuView {}
