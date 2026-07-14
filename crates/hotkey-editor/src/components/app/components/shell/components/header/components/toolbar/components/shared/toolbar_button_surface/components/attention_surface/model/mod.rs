use super::view::AttentionSurfaceView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AttentionSurfaceModel {
    pub icon: &'static str,
    pub aria_label: &'static str,
    pub aria_haspopup: Option<&'static str>,
    pub aria_expanded: Option<bool>,
    pub aria_pressed: Option<bool>,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&AttentionSurfaceView> for AttentionSurfaceModel {
    fn from(view: &AttentionSurfaceView) -> Self {
        let AttentionSurfaceView {
            icon,
            aria_label,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            disabled,
            onclick,
        } = view.clone();
        Self {
            icon,
            aria_label,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            disabled,
            onclick,
        }
    }
}

impl ddd::Model for AttentionSurfaceModel {
    type View = AttentionSurfaceView;
}
