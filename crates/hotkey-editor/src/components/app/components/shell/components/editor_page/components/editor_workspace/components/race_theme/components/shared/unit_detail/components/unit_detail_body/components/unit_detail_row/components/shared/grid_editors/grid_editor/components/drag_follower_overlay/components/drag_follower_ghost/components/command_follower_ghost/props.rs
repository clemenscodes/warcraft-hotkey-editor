use super::super::super::logic::FollowerPresentation;
use super::view::CommandFollowerGhostView;
use dioxus::prelude::*;

/// A command-menu follower ghost: the dragged tile's presentation. Its border and glow
/// take the grid's race accent from the inherited `--race-color`, so no race is a prop.
#[derive(Props, Clone, PartialEq)]
pub struct CommandFollowerGhostProps {
    pub presentation: FollowerPresentation,
}

impl From<&CommandFollowerGhostView> for CommandFollowerGhostProps {
    fn from(view: &CommandFollowerGhostView) -> Self {
        let CommandFollowerGhostView { presentation } = view.clone();
        Self { presentation }
    }
}

impl ddd::Props for CommandFollowerGhostProps {
    type View = CommandFollowerGhostView;
}
