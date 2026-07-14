use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AttentionSurfaceView {
    pub icon: &'static str,
    pub aria_label: &'static str,
    pub aria_haspopup: Option<&'static str>,
    pub aria_expanded: Option<bool>,
    pub aria_pressed: Option<bool>,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for AttentionSurfaceView {}
