use super::components::toolbar_button_icon::ToolbarButtonIconProps;
use dioxus::prelude::*;

/// The surface carries every attribute of the clickable button. Its parent
/// `ToolbarButton` builds it by conversion and spreads it, so callers never pass loose
/// attributes by hand.
#[derive(Props, Clone, PartialEq, Default)]
pub struct ToolbarButtonSurfaceProps {
    pub icon: &'static str,
    pub aria_label: &'static str,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub data_action: Option<&'static str>,
    #[props(default)]
    pub aria_haspopup: Option<&'static str>,
    #[props(default)]
    pub aria_expanded: Option<bool>,
    #[props(default)]
    pub aria_pressed: Option<bool>,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ToolbarButtonSurfaceProps> for ToolbarButtonIconProps {
    fn from(props: &ToolbarButtonSurfaceProps) -> Self {
        let icon = props.icon;
        Self { icon }
    }
}
