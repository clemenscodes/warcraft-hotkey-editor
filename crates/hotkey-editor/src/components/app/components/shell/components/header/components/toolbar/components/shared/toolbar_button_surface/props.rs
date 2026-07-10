use super::state::SurfaceState;
use super::view::ToolbarButtonSurfaceView;
use dioxus::prelude::*;

/// The surface carries every attribute of the clickable button: the icon, the resting
/// look, the click handler, and the aria/disabled state. Callers set the fields they
/// need by name; the rest default.
#[derive(Props, Clone, PartialEq, Default)]
pub struct ToolbarButtonSurfaceProps {
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

impl From<&ToolbarButtonSurfaceView> for ToolbarButtonSurfaceProps {
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

impl ddd::Props for ToolbarButtonSurfaceProps {
    type View = ToolbarButtonSurfaceView;
}
