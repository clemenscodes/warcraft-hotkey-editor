use super::super::super::presentation::FollowerPresentation;
use super::view::CommandFollowerGhostView;
use dioxus::prelude::*;

/// A command-menu follower ghost: the dragged tile's presentation. Its border and glow
/// take the grid's race accent from the inherited `--race-color`, so no race is a prop.
#[derive(Props, Clone, PartialEq)]
pub struct CommandFollowerGhostModel {
    pub presentation: FollowerPresentation,
}

impl From<&CommandFollowerGhostView> for CommandFollowerGhostModel {
    fn from(view: &CommandFollowerGhostView) -> Self {
        let CommandFollowerGhostView { presentation } = view.clone();
        Self { presentation }
    }
}

impl ddd::Model for CommandFollowerGhostModel {
    type View = CommandFollowerGhostView;
}
