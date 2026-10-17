use super::presentation::FollowerPresentation;

#[derive(Clone, PartialEq)]
pub struct DragFollowerGhostView {
    pub presentation: Option<FollowerPresentation>,
}

impl ddd::Layered for DragFollowerGhostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for DragFollowerGhostView {}
