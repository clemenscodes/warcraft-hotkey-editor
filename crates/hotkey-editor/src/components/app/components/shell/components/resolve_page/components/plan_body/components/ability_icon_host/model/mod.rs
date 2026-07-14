use super::view::AbilityIconHostView;
use crate::services::carriers::InspectedAbility;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AbilityIconHostModel {
    #[props(into)]
    pub name: String,
    pub icon_url: Option<String>,
    pub carrier_count: usize,
    pub is_winner: bool,
    pub disabled: bool,
    pub inspected: InspectedAbility,
}

impl From<&AbilityIconHostView> for AbilityIconHostModel {
    fn from(view: &AbilityIconHostView) -> Self {
        let AbilityIconHostView {
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

impl ddd::Model for AbilityIconHostModel {
    type View = AbilityIconHostView;
}
