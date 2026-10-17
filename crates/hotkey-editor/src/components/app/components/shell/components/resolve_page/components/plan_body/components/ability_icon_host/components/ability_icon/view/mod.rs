use crate::services::carriers::InspectedAbility;

#[derive(Clone, PartialEq)]
pub struct AbilityIconView {
    pub name: String,
    pub icon_url: Option<String>,
    pub carrier_count: usize,
    pub is_winner: bool,
    pub disabled: bool,
    pub inspected: InspectedAbility,
}

impl ddd::Layered for AbilityIconView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for AbilityIconView {}
