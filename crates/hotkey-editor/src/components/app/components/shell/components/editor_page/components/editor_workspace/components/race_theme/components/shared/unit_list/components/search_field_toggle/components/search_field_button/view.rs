use dioxus::prelude::*;

/// The published `View` contract mirroring [`SearchFieldButtonProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SearchFieldButtonView {
    pub label: &'static str,
    pub is_active: bool,
    pub on_select: EventHandler<MouseEvent>,
}

impl ddd::View for SearchFieldButtonView {}
