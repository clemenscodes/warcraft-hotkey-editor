use dioxus::prelude::*;

/// The published `View` contract mirroring [`UnitCategoryHeadingProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitCategoryHeadingView {
    pub label: String,
    pub is_collapsed: bool,
    pub on_toggle: EventHandler<MouseEvent>,
}

impl ddd::View for UnitCategoryHeadingView {}
