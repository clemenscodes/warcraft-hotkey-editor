use dioxus::prelude::*;

/// The published `View` contract mirroring [`AltStatePositionButtonProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AltStatePositionButtonView {
    pub title: String,
    pub aria_label: &'static str,
    pub on_click: EventHandler<()>,
}

impl ddd::View for AltStatePositionButtonView {}
