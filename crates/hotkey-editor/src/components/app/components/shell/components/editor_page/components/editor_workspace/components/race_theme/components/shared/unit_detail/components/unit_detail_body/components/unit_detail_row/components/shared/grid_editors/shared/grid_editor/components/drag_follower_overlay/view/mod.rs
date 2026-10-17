use crate::services::drag_state::DragFollower;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DragFollowerOverlayView {
    pub drag_follower: Signal<Option<DragFollower>>,
    pub visible: bool,
}

impl ddd::Layered for DragFollowerOverlayView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for DragFollowerOverlayView {}
