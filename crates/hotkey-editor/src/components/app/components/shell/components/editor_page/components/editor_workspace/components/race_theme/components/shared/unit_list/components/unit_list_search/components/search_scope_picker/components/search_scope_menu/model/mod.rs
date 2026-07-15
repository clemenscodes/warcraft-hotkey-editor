use super::view::SearchScopeMenuView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchScopeMenuModel {
    #[props(into)]
    pub unit_label: String,
    #[props(into)]
    pub ability_label: String,
    pub unit_is_active: bool,
    pub ability_is_active: bool,
    pub select_unit: EventHandler<MouseEvent>,
    pub select_ability: EventHandler<MouseEvent>,
}

impl From<&SearchScopeMenuView> for SearchScopeMenuModel {
    fn from(view: &SearchScopeMenuView) -> Self {
        let SearchScopeMenuView {
            unit_label,
            ability_label,
            unit_is_active,
            ability_is_active,
            select_unit,
            select_ability,
        } = view.clone();
        Self {
            unit_label,
            ability_label,
            unit_is_active,
            ability_is_active,
            select_unit,
            select_ability,
        }
    }
}

impl ddd::Model for SearchScopeMenuModel {
    type View = SearchScopeMenuView;
}
