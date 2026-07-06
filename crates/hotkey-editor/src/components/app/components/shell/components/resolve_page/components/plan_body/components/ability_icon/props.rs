use crate::services::carriers::InspectedAbility;
use dioxus::prelude::*;

/// One ability icon with its carrier-count badge; clicking opens this ability's
/// carriers dialog. The winner of a Fight is ringed gold. `inspected` is the opaque
/// identity the dialog opens on — resolved to carrier views only by the dialog's host.
#[derive(Props, Clone, PartialEq)]
pub struct AbilityIconProps {
    #[props(into)]
    pub name: String,
    pub icon_url: Option<String>,
    pub carrier_count: usize,
    pub is_winner: bool,
    pub disabled: bool,
    pub inspected: InspectedAbility,
}
