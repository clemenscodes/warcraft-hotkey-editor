use super::super::super::logic::FollowerPresentation;
use dioxus::prelude::*;

/// An ability-menu follower ghost: the dragged tile's presentation. Its border and glow
/// take the grid's race accent from the inherited `--race-accent`, so no race is a prop.
#[derive(Props, Clone, PartialEq)]
pub struct AbilityFollowerGhostProps {
    pub presentation: FollowerPresentation,
}
