use super::view::ClearSurfaceView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ClearSurfaceModel {
    pub icon: &'static str,
    pub aria_label: &'static str,
    pub aria_haspopup: Option<&'static str>,
    pub aria_expanded: Option<bool>,
    pub aria_pressed: Option<bool>,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ClearSurfaceView> for ClearSurfaceModel {
    fn from(view: &ClearSurfaceView) -> Self {
        let ClearSurfaceView {
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

impl ddd::Model for ClearSurfaceModel {
    type View = ClearSurfaceView;
}
