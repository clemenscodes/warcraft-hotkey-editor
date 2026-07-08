use super::super::super::logic::FollowerPresentation;
use dioxus::prelude::*;

/// A command-menu follower ghost: the dragged tile's presentation plus the grid's race
/// accent, which tints the border and glow.
#[derive(Props, Clone, PartialEq)]
pub struct CommandFollowerGhostProps {
    pub race_attribute: &'static str,
    pub presentation: FollowerPresentation,
}
