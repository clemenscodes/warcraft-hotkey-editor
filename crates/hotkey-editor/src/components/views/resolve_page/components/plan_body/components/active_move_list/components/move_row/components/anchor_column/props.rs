use crate::components::views::resolve_page::components::plan_body::components::ability_icon::AbilityIconProps;
use crate::components::views::resolve_page::components::plan_body::components::fight_name_plate::FightNamePlateProps;
use dioxus::prelude::*;

/// The rival ability's plate and icon, present only on Fight/Swap moves.
#[derive(Clone, PartialEq)]
pub struct AnchorParts {
    pub name_plate: FightNamePlateProps,
    pub ability: AbilityIconProps,
}

/// The optional rival column of a move card: the anchor ability's name plate and
/// icon, or nothing on moves without a rival.
#[derive(Props, Clone, PartialEq)]
pub struct AnchorColumnProps {
    pub anchor: Option<AnchorParts>,
}
