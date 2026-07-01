use dioxus::prelude::*;

use super::components::ability_description::AbilityDescriptionProps;
use super::components::alt_state_section::AltStateSectionProps;
use super::components::tile_override_header::TileOverrideHeaderProps;
use super::components::upgrade_section::UpgradeSectionProps;
use super::components::upgrade_tier::UpgradeTierProps;

/// The override card owns the header and the four ability sections; each child's
/// props are built by the panel's hook and threaded through here.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideCardProps {
    pub header: TileOverrideHeaderProps,
    pub description: AbilityDescriptionProps,
    pub alt_state: AltStateSectionProps,
    pub upgrade: UpgradeSectionProps,
    pub tier: UpgradeTierProps,
}
