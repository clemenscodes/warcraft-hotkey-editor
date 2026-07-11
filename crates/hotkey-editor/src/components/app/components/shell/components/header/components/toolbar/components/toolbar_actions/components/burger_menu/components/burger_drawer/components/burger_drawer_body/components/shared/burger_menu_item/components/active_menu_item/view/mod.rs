use dioxus::prelude::*;

/// The published `View` contract mirroring [`ActiveMenuItemModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ActiveMenuItemView {
    pub icon: &'static str,
    pub label: String,
    pub disabled: bool,
    pub role: Option<&'static str>,
    pub aria_haspopup: Option<&'static str>,
    pub aria_expanded: Option<&'static str>,
    pub aria_pressed: Option<&'static str>,
    pub aria_label: Option<&'static str>,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for ActiveMenuItemView {}
