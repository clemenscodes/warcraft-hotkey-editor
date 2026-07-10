use dioxus::prelude::*;

/// The published `View` contract mirroring [`IdleMobileCategoryTabProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IdleMobileCategoryTabView {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for IdleMobileCategoryTabView {}
