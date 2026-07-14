use super::view::DraggingSourceGhostView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DraggingSourceGhostModel {
    pub active: bool,
}

impl From<&DraggingSourceGhostView> for DraggingSourceGhostModel {
    fn from(view: &DraggingSourceGhostView) -> Self {
        let DraggingSourceGhostView { active } = view.clone();
        Self { active }
    }
}

impl ddd::Model for DraggingSourceGhostModel {
    type View = DraggingSourceGhostView;
}
