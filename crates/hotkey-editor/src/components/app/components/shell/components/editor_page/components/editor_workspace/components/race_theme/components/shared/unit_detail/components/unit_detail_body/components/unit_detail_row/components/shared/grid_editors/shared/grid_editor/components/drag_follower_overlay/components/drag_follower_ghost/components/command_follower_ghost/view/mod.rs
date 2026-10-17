use super::super::super::presentation::FollowerPresentation;

#[derive(Clone, PartialEq)]
pub struct CommandFollowerGhostView {
    pub presentation: FollowerPresentation,
}

impl ddd::Layered for CommandFollowerGhostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CommandFollowerGhostView {}
