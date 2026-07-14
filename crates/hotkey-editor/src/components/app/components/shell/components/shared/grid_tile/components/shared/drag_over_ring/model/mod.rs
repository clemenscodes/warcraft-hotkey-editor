use super::view::DragOverRingView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DragOverRingModel {
    pub active: bool,
}

impl From<&DragOverRingView> for DragOverRingModel {
    fn from(view: &DragOverRingView) -> Self {
        let DragOverRingView { active } = view.clone();
        Self { active }
    }
}

impl ddd::Model for DragOverRingModel {
    type View = DragOverRingView;
}
