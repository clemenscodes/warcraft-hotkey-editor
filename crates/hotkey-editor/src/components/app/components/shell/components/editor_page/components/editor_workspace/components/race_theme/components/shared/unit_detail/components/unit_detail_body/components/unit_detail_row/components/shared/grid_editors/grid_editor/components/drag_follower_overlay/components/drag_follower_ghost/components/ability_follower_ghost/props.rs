use super::super::super::logic::FollowerPresentation;
use super::view::AbilityFollowerGhostView;
use dioxus::prelude::*;

/// An ability-menu follower ghost: the dragged tile's presentation. Its border and glow
/// take the grid's race accent from the inherited `--race-color`, so no race is a prop.
#[derive(Props, Clone, PartialEq)]
pub struct AbilityFollowerGhostProps {
    pub presentation: FollowerPresentation,
}

impl From<&AbilityFollowerGhostView> for AbilityFollowerGhostProps {
    fn from(view: &AbilityFollowerGhostView) -> Self {
        let AbilityFollowerGhostView { presentation } = view.clone();
        Self { presentation }
    }
}

impl ddd::Props for AbilityFollowerGhostProps {
    type View = AbilityFollowerGhostView;
}
