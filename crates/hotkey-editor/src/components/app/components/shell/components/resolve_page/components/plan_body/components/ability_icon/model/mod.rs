use super::view::AbilityIconView;
use crate::services::carriers::InspectedAbility;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AbilityIconModel {
    #[props(into)]
    pub name: String,
    pub icon_url: Option<String>,
    pub carrier_count: usize,
    pub is_winner: bool,
    pub disabled: bool,
    pub inspected: InspectedAbility,
}

impl From<&AbilityIconView> for AbilityIconModel {
    fn from(view: &AbilityIconView) -> Self {
        let AbilityIconView {
            name,
            icon_url,
            carrier_count,
            is_winner,
            disabled,
            inspected,
        } = view.clone();
        Self {
            name,
            icon_url,
            carrier_count,
            is_winner,
            disabled,
            inspected,
        }
    }
}

impl ddd::Model for AbilityIconModel {
    type View = AbilityIconView;
}
