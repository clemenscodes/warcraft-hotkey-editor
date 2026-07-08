use super::super::super::logic::FollowerPresentation;
use dioxus::prelude::*;

/// An ability-menu follower ghost: the dragged tile's presentation plus the grid's race
/// accent, which tints the border and glow.
#[derive(Props, Clone, PartialEq)]
pub struct AbilityFollowerGhostProps {
    pub race_attribute: &'static str,
    pub presentation: FollowerPresentation,
}
