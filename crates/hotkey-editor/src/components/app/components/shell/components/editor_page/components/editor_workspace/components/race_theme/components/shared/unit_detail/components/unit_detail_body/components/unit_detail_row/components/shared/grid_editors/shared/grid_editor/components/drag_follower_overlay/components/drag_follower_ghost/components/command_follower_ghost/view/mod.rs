use super::super::super::presentation::FollowerPresentation;

#[derive(Clone, PartialEq)]
pub struct CommandFollowerGhostView {
    pub presentation: FollowerPresentation,
}

impl ddd::View for CommandFollowerGhostView {}
