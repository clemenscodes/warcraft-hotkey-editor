use super::presentation::FollowerPresentation;
use super::view::DragFollowerGhostView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DragFollowerGhostModel {
    pub presentation: Option<FollowerPresentation>,
}

impl From<&DragFollowerGhostView> for DragFollowerGhostModel {
    fn from(view: &DragFollowerGhostView) -> Self {
        let DragFollowerGhostView { presentation } = view.clone();
        Self { presentation }
    }
}

impl ddd::Model for DragFollowerGhostModel {
    type View = DragFollowerGhostView;
}
