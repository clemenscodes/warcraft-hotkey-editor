use dioxus::prelude::*;

/// The published `View` contract mirroring [`ClearSurfaceModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ClearSurfaceView {
    pub icon: &'static str,
    pub aria_label: &'static str,
    pub aria_haspopup: Option<&'static str>,
    pub aria_expanded: Option<bool>,
    pub aria_pressed: Option<bool>,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for ClearSurfaceView {}
