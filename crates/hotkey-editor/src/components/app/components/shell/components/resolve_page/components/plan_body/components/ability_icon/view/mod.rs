use crate::services::carriers::InspectedAbility;

/// The published `View` contract mirroring [`AbilityIconModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AbilityIconView {
    pub name: String,
    pub icon_url: Option<String>,
    pub carrier_count: usize,
    pub is_winner: bool,
    pub disabled: bool,
    pub inspected: InspectedAbility,
}

impl ddd::View for AbilityIconView {}
