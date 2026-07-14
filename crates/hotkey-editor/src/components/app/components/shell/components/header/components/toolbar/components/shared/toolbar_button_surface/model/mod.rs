use super::state::SurfaceState;
use super::view::ToolbarButtonSurfaceView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq, Default)]
pub struct ToolbarButtonSurfaceModel {
    pub icon: &'static str,
    pub aria_label: &'static str,
    #[props(default)]
    pub state: SurfaceState,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub aria_haspopup: Option<&'static str>,
    #[props(default)]
    pub aria_expanded: Option<bool>,
    #[props(default)]
    pub aria_pressed: Option<bool>,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ToolbarButtonSurfaceView> for ToolbarButtonSurfaceModel {
    fn from(view: &ToolbarButtonSurfaceView) -> Self {
        let ToolbarButtonSurfaceView {
            icon,
            aria_label,
            state,
            disabled,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            onclick,
        } = view.clone();
        Self {
            icon,
            aria_label,
            state,
            disabled,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            onclick,
        }
    }
}

impl ddd::Model for ToolbarButtonSurfaceModel {
    type View = ToolbarButtonSurfaceView;
}
