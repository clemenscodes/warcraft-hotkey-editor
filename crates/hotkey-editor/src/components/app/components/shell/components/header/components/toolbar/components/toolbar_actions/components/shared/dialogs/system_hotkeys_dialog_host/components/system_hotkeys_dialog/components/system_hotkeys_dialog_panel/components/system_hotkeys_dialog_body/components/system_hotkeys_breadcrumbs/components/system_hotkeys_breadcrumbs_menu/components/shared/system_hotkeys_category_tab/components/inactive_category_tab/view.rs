use dioxus::prelude::*;

/// The published `View` contract mirroring [`InactiveCategoryTabProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct InactiveCategoryTabView {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}

impl ddd::View for InactiveCategoryTabView {}
