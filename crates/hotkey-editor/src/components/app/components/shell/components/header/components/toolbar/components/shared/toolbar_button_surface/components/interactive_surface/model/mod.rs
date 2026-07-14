use super::view::InteractiveSurfaceView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InteractiveSurfaceModel {
    pub icon: &'static str,
    pub aria_label: &'static str,
    pub aria_haspopup: Option<&'static str>,
    pub aria_expanded: Option<bool>,
    pub aria_pressed: Option<bool>,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&InteractiveSurfaceView> for InteractiveSurfaceModel {
    fn from(view: &InteractiveSurfaceView) -> Self {
        let InteractiveSurfaceView {
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

impl ddd::Model for InteractiveSurfaceModel {
    type View = InteractiveSurfaceView;
}
