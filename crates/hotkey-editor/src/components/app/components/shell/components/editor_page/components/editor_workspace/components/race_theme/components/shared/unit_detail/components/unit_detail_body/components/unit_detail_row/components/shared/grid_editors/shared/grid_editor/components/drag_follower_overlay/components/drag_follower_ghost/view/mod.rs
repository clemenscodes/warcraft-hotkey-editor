use super::presentation::FollowerPresentation;

#[derive(Clone, PartialEq)]
pub struct DragFollowerGhostView {
    pub presentation: Option<FollowerPresentation>,
}

impl ddd::View for DragFollowerGhostView {}
