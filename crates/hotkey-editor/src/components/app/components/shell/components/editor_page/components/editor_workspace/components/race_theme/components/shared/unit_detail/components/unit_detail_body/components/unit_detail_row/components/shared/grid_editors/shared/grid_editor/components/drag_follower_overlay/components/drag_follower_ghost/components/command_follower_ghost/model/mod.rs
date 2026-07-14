use super::super::super::presentation::FollowerPresentation;
use super::view::CommandFollowerGhostView;
use dioxus::prelude::*;

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
